#[macro_use]
extern crate diesel;

use args::{process_command, Args};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use clap::Parser;

pub mod args;
pub mod db;
pub mod game;
pub mod properties;

fn main() {
  dotenv::dotenv().ok();

  let mut app: &mut App = &mut App::new();
  app = app.add_plugin(LogPlugin);

  let args = Args::parse();
  if process_command(args.command) {
    return;
  }

  app
    .add_plugins(MinimalPlugins)
    .add_plugin(properties::PropertiesPlugin)
    .add_plugin(db::DatabasePlugin)
    .add_plugins(game::GamePlugins)
    .run();
}
