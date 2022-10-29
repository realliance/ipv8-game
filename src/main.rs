#![feature(path_try_exists)]

#[macro_use]
extern crate diesel;

use args::{Args, process_command};
use clap::Parser;
use bevy::log::LogPlugin;
use bevy::prelude::*;

pub mod db;
pub mod game;
pub mod args;
pub mod properties;

#[cfg(tests)]
mod tests;

fn main() {
  dotenv::dotenv().ok();

  let args = Args::parse();

  if process_command(args.command) {
    return;
  }

  App::new()
    .add_plugins(MinimalPlugins)
    .add_system(bevy::window::close_on_esc)
    .add_plugin(LogPlugin)
    .add_plugin(properties::PropertiesPluign)
    .add_plugin(db::DatabasePlugin)
    .add_plugins(game::GamePlugins)
    .run();
}
