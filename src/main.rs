#![feature(let_chains)]
#![feature(test)]
#![feature(iter_collect_into)]

#[macro_use]
extern crate diesel;

use args::{process_command, Args};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use clap::Parser;

use crate::args::ArgsSideEffect;
use crate::debug::DebugCameraPlugin;

pub mod args;
pub mod db;
pub mod debug;
pub mod game;
pub mod properties;

fn main() {
  dotenv::dotenv().ok();

  let mut app: &mut App = &mut App::new();

  let args = Args::parse();
  let args_effect = process_command(args.command);
  if Some(ArgsSideEffect::Exit) == args_effect {
    return;
  }

  app = if Some(ArgsSideEffect::AddDebuggingWindowPlugins) == args_effect {
    app.add_plugins(DefaultPlugins).add_plugin(DebugCameraPlugin);
    info!("Debug Window Enabled");
    app
  } else {
    app.add_plugins(MinimalPlugins).add_plugin(LogPlugin::default())
  };

  info!("Loading plugins...");

  app = app
    .add_plugin(properties::PropertiesPlugin)
    .add_plugin(db::DatabasePlugin)
    .add_plugins(game::GamePlugins);

  info!("Starting game...");

  app.run();
}
