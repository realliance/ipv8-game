#[macro_use]
extern crate diesel;

use bevy::log::LogPlugin;
use bevy::prelude::*;

pub mod db;
pub mod game;

fn main() {
  dotenv::dotenv().ok();

  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(LogPlugin)
    .add_plugin(db::DatabasePlugin)
    .add_plugins(game::GamePlugins)
    .run();
}
