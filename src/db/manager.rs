//! DatabaseManager Resource
//! The [DatabaseManager] handles the database connection pool for the game.
//! Since [Pool] defaults it's taking to a syncronous timeout, [DatabaseManager]
//! adds a later of safety and ergonomics with a [Semaphore], allowing for async
//! waiting for a connection.

use std::ops::{Deref, DerefMut};

use bevy::prelude::Resource;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
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
#[derive(Resource)]
pub struct DatabaseManager {
  /// Database Pool.
  pool: Option<Pool<ConnectionManager<PgConnection>>>,
  /// Tracks number of connections remaining avaliable.
  take_count: Semaphore,
}

impl DatabaseManager {
  /// Creates a new PostgreSQL database connection pool within the manager.
  pub fn new(connection: String) -> Result<Self, String> {
    let pool = Pool::new(ConnectionManager::new(connection)).map_err(|x| x.to_string())?;
    Ok(Self {
      take_count: Semaphore::new(pool.state().idle_connections as usize),
      pool: Some(pool),
    })
  }

  pub fn test_harness() -> Self {
    Self {
      take_count: Semaphore::new(0),
      pool: None,
    }
  }

  /// Async waits for a connection to become avaliable and then takes it.
  pub async fn take(&self) -> Result<AcquiredDatabaseConnection, String> {
    let permit = self.take_count.acquire().await.map_err(|x| x.to_string())?;
    Ok(AcquiredDatabaseConnection {
      connection: self.pool.as_ref().unwrap().get().map_err(|x| x.to_string())?,
      _permit: permit,
    })
  }

  /// Attempts to take a free connection, and errors if fails to.
  pub fn try_take(&self) -> Result<AcquiredDatabaseConnection, String> {
    let permit = self.take_count.try_acquire().map_err(|x| x.to_string())?;
    Ok(AcquiredDatabaseConnection {
      connection: self.pool.as_ref().unwrap().get().map_err(|x| x.to_string())?,
      _permit: permit,
    })
  }
}
