use bevy::prelude::*;

use crate::db::models::World;
use crate::db::DatabaseManager;
use crate::game::world::gen::WorldGenPlugin;

mod gen;
mod resources;

pub use gen::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    info!("Building World");

    let world = {
      let conn = &mut app
        .world
        .get_resource::<DatabaseManager>()
        .expect("Failed to get DatabasePool. Ensure the DatabasePlugin is added before this plugin.")
        .try_take()
        .expect("Failed to get database connection from pool.");

      World::from_db(conn).unwrap_or(World::build().save(conn))
    };

    app
      .insert_resource(world)
      .add_plugin(WorldGenPlugin);
  }
}
