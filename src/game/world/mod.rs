use bevy::prelude::*;

use crate::db::models::World;
use crate::db::DatabaseManager;
use crate::game::world::gen::WorldGenPlugin;
use crate::properties::GameProperties;

mod gen;
mod resources;

pub use gen::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    info!("Building World");

    let properties: &GameProperties = app
      .world
      .get_resource()
      .expect("Failed to load Game Properties while loading World. Is the PropertiesPlugin loaded?");

    let world = {
      let conn = &mut app
        .world
        .get_resource::<DatabaseManager>()
        .expect("Failed to get DatabasePool. Ensure the DatabasePlugin is added before this plugin.")
        .try_take()
        .expect("Failed to get database connection from pool.");

      World::from_db(conn)
        .and_then(|x| {
          if x.seed == properties.seed {
            Some(x)
          } else {
            warn!("Seed differed from that in database! Rebuilding world...");
            World::reset_db(conn).unwrap();
            None
          }
        })
        .unwrap_or(World::build_with_seed(properties.seed).save(conn))
    };

    app.insert_resource(world).add_plugin(WorldGenPlugin);
  }
}

#[cfg(test)]
mod tests {
  use bevy::ecs::system::CommandQueue;
  use bevy::prelude::*;
  use chrono::NaiveDateTime;

  use super::gen::WorldGenPlugin;
  use super::LoadedChunkTable;
  use crate::db::models::{World, WorldObj};
  use crate::db::DatabaseManager;
  use crate::game::stages::StagePlugin;
  use crate::game::world::{ComplexTerrainTile, LoadChunkCommand, StaticTerrainTile, TerrainTile};

  #[test]
  fn verify_load_chunk() {
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .insert_resource(DatabaseManager::test_harness())
      .insert_resource::<World>(
        WorldObj {
          id: 0,
          origin_time: NaiveDateTime::MIN,
          seed: 1337,
        }
        .into(),
      )
      .add_plugin(WorldGenPlugin);

    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &app.world);

    let chunk_table: &LoadedChunkTable = app.world.get_resource().unwrap();
    assert!(chunk_table.get_if_exists([0, 0]).is_none());

    // Ensure load works
    commands.spawn(LoadChunkCommand([0, 0]));
    queue.apply(&mut app.world);
    app.update();

    let chunk_table: &LoadedChunkTable = app.world.get_resource().unwrap();
    let chunk = chunk_table.get_if_exists([0, 0]);
    assert!(chunk.is_some());
    let chunk = chunk.unwrap();

    // Ensure seed produces repeatable generation
    assert_eq!(chunk.chunk[0], TerrainTile::Static(StaticTerrainTile::Stone));
    assert_eq!(
      chunk.chunk[2056],
      TerrainTile::Complex(ComplexTerrainTile::Copper(4562))
    );

    // Ensure unload works
    //commands.spawn().insert(UnloadChunkCommand([0, 0]));
    //queue.apply(&mut app.world);
    //app.update();

    //let chunk_table: &LoadedChunkTable = app.world.get_resource().unwrap();
    //assert!(chunk_table.get_if_exists([0, 0]).is_none());
  }
}
