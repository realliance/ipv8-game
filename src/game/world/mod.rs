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

    app.insert_resource(world).add_plugin(WorldGenPlugin);
  }
}

#[cfg(test)]
mod tests {
  use bevy::prelude::*;
  use chrono::NaiveDateTime;

  use super::gen::WorldGenPlugin;
  use super::LoadedChunkTable;
  use crate::db::models::{World, WorldObj};
  use crate::game::stages::StagePlugin;
  use crate::game::world::{ChunkRequests, ComplexTerrainTile, StaticTerrainTile, TerrainTile};

  #[test]
  fn verify_gen() {
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .insert_resource::<World>(
        WorldObj {
          id: 0,
          origin_time: NaiveDateTime::MIN,
          seed: 1337,
        }
        .into(),
      )
      .add_plugin(WorldGenPlugin);

    let chunk_table: &LoadedChunkTable = app.world.get_resource().unwrap();
    assert!(chunk_table.0.get(&[0, 0]).is_none());

    // Ensure load works
    app.world.send_event(ChunkRequests::Load(0, 0));
    app.update();

    let chunk_table: &LoadedChunkTable = app.world.get_resource().unwrap();
    let chunk = chunk_table.0.get(&[0, 0]);
    assert!(chunk.is_some());
    let chunk = chunk.unwrap();

    // Ensure seed produces repeatable generation
    assert_eq!(chunk.chunk[0], TerrainTile::Static(StaticTerrainTile::Stone));
    assert_eq!(
      chunk.chunk[2056],
      TerrainTile::Complex(ComplexTerrainTile::Copper(4562))
    );

    // Ensure unload works
    app.world.send_event(ChunkRequests::Unload(0, 0));
    app.update();

    let chunk_table: &LoadedChunkTable = app.world.get_resource().unwrap();
    assert!(chunk_table.0.get(&[0, 0]).is_none());
  }
}
