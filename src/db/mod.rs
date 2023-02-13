//! Database Schema and Associated Models.

use std::env;

use bevy::prelude::*;

mod manager;
pub mod models;
mod schema;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
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

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
  fn build(&self, app: &mut App) {
    let db = DatabaseManager::new(get_db_url());

    if let Err(err) = db {
      panic!("Error while starting DatabaseManager {}", err);
    }

    let db = db.unwrap();
    info!("Performing migrations...");
    db.try_take().unwrap().run_pending_migrations(MIGRATIONS).unwrap();

    app.insert_resource(db);
  }
}
