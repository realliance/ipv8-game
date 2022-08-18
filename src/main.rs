use bevy::{prelude::*, log::LogPlugin};

mod db;

fn main() {
  dotenv::dotenv().ok();

  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugin(LogPlugin)
    .add_plugin(db::DatabasePlugin)
    .run();
}
