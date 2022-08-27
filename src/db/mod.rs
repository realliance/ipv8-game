//! Database Schema and Associated Models.

use std::env;
use std::ops::Deref;

use bevy::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod models;
mod schema;

pub use schema::*;

#[inline(always)]
fn get_db_url() -> String {
  #[cfg(not(test))]
  let uri = env::var("DATABASE_URI").expect("Could not find the environment variable DATABASE_URL");

  #[cfg(test)]
  let uri = env::var("TEST_DATABASE_URI").expect("Could not find the environment variable TEST_DATABASE_URI");

  format!(
    "postgres://{}:{}@{}/{}",
    env::var("DATABASE_USER").expect("Could not find the environment variable DATABASE_USER"),
    env::var("DATABASE_PASS").expect("Could not find the environment variable DATABASE_PASS"),
    uri,
    env::var("DATABASE_DB").expect("Could not find the environment variable DATABASE_DB"),
  )
}

fn build_db_pool() -> Pool<ConnectionManager<PgConnection>> {
  info!("Building Database Pool");
  Pool::new(ConnectionManager::new(get_db_url())).unwrap()
}

pub struct DatabasePool(Pool<ConnectionManager<PgConnection>>);

impl Deref for DatabasePool {
  type Target = Pool<ConnectionManager<PgConnection>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
  fn build(&self, app: &mut App) {
    let pool = build_db_pool();
    info!("Database Pool built with {} connections", pool.state().connections);

    app.insert_resource(DatabasePool(pool));
  }
}
