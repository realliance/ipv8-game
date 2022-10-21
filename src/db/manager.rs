//! DatabaseManager Resource
//! The [DatabaseManager] handles the database connection pool for the game. Since [Pool] defaults it's taking to a syncronous timeout, 
//! [DatabaseManager] adds a later of safety and ergonomics with a [Semaphore], allowing for async waiting for a connection.

use std::{ops::{Deref, DerefMut}, marker::PhantomData};

use diesel::{r2d2::{Pool, ConnectionManager, PooledConnection}, PgConnection};
use tokio::sync::{Semaphore, SemaphorePermit};

pub type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Represents an acquired database connection, ready to be used.
pub struct AcquiredDatabaseConnection<'a> {
  _permit: SemaphorePermit<'a>,
  connection: PooledPgConnection,
}

impl<'a> Deref for AcquiredDatabaseConnection<'a> {
  type Target = PooledPgConnection;

  fn deref(&self) -> &Self::Target {
    &self.connection
  }
}

impl<'a> DerefMut for AcquiredDatabaseConnection<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.connection
  }
}

/// The Main Database Management Resource.
pub struct DatabaseManager<'a> {
  /// Database Pool.
  pool: Pool<ConnectionManager<PgConnection>>,
  /// Tracks number of connections remaining avaliable.
  take_count: Semaphore,
  _marker: PhantomData<&'a Self>
}

impl<'a> DatabaseManager<'a> {
  /// Creates a new PostgreSQL database connection pool within the manager.
  pub fn new(connection: String) -> Result<Self, String> {
    let pool = Pool::new(ConnectionManager::new(connection)).map_err(|x| x.to_string())?;
    Ok(Self {
      take_count: Semaphore::new(pool.state().idle_connections as usize),
      pool,
      _marker: PhantomData::default(),
    })
  }

  /// Async waits for a connection to become avaliable and then takes it.
  pub async fn take(&'a self) -> Result<AcquiredDatabaseConnection<'a>, String> {
    let permit = self.take_count.acquire().await.map_err(|x| x.to_string())?;
    Ok(AcquiredDatabaseConnection {
      connection: self.pool.get().map_err(|x| x.to_string())?,
      _permit: permit
    })
  }

  /// Attempts to take a free connection, and errors if fails to.
  pub fn try_take(&'a self) -> Result<AcquiredDatabaseConnection<'a>, String> {
    let permit = self.take_count.try_acquire().map_err(|x| x.to_string())?;
    Ok(AcquiredDatabaseConnection {
      connection: self.pool.get().map_err(|x| x.to_string())?,
      _permit: permit
    })
  }
}
