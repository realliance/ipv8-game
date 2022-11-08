//! Database Schema and Associated Models.

use std::env;

use bevy::prelude::*;

mod manager;
pub mod models;
mod schema;

pub use manager::*;
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

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
  fn build(&self, app: &mut App) {
    let db = DatabaseManager::new(get_db_url());
    if let Err(err) = db {
      panic!("Error while starting DatabaseManager {}", err);
    }

    app.insert_resource(db.unwrap());
  }
}
