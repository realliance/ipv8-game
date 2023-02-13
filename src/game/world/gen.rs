use bevy::prelude::*;
use hashbrown::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;

use super::resources::*;
use crate::db::models::World;

#[derive(Debug, Clone, Copy)]
pub enum TerrainTile {
  Water = 0,
  Stone = 1,
  Iron = 2,
  Copper = 3,
  Coal = 4,
  Impassable = 5,
}

impl TerrainTile {
  pub fn get_tile_color(&self) -> Color {
    match self {
      TerrainTile::Water => Color::BLUE,
      TerrainTile::Stone => Color::GRAY,
      TerrainTile::Iron => Color::SILVER,
      TerrainTile::Copper => Color::ORANGE,
      TerrainTile::Coal => Color::DARK_GRAY,
      TerrainTile::Impassable => Color::BLACK,
    }
  }

  pub fn get_letter(&self) -> &str {
    match self {
      TerrainTile::Water => "W",
      TerrainTile::Stone => ".",
      TerrainTile::Iron => "I",
      TerrainTile::Copper => "C",
      TerrainTile::Coal => "L",
      TerrainTile::Impassable => "X",
    }
  }
}

lazy_static! {
  static ref CHUNK_TABLE: Vec<(u32, u32)> = (0..World::CHUNK_SIDE_LENGTH as u32)
    .cartesian_product(0..World::CHUNK_SIDE_LENGTH as u32)
    .collect_vec();
}

impl World {
  pub const CHUNK_SIDE_LENGTH: usize = 64;
  pub const CHUNK_SIZE: usize = Self::CHUNK_SIDE_LENGTH * Self::CHUNK_SIDE_LENGTH;
  pub const TILE_PIXEL_SIZE: f32 = 16.0;

  /// Returns the position of a tile given it's index in the chunk list.
  #[inline(always)]
  pub fn get_tile_position_from_index([c_x, c_y]: [i32; 2], index: usize) -> [i32; 2] {
    let x = (index % World::CHUNK_SIDE_LENGTH) as i32 + (c_x * World::CHUNK_SIDE_LENGTH as i32);
    let y = (index / World::CHUNK_SIDE_LENGTH) as i32 + (c_y * World::CHUNK_SIDE_LENGTH as i32);
    [x, y]
  }

  /// Returns the index of a tile in a chunk list given it's offset coordinates.
  #[inline(always)]
  pub fn get_chunk_index([x_offset, y_offset]: [u32; 2]) -> usize {
    y_offset as usize * World::CHUNK_SIDE_LENGTH as usize + x_offset as usize
  }

  /// Returns a chunk for the given world, using the properties of the provided
  /// world generator.
  pub fn get_chunk(&self, generator: &WorldGenerator, [x, y]: [i32; 2]) -> [TerrainTile; Self::CHUNK_SIZE] {
    let x = x * Self::CHUNK_SIDE_LENGTH as i32;
    let y = y * Self::CHUNK_SIDE_LENGTH as i32;

    let mut chunk = [TerrainTile::Stone; Self::CHUNK_SIZE];

    CHUNK_TABLE.iter().for_each(|(x_offset, y_offset)| {
      chunk[Self::get_chunk_index([*x_offset, *y_offset])] =
        generator.get_tile(&self, [x + *x_offset as i32, y + *y_offset as i32])
    });

    chunk
  }

  /// Debug output a chunk to standard output
  pub fn print_chunk(chunk: &[TerrainTile; Self::CHUNK_SIZE]) {
    let text_block = chunk
      .chunks(Self::CHUNK_SIDE_LENGTH)
      .map(|row| row.into_iter().map(|x| x.get_letter()).collect::<Vec<_>>().join(""))
      .collect::<Vec<_>>()
      .join("\n");

    info!("Chunk Output:\n{}", text_block);
  }
}

pub enum ChunkRequests {
  Load(i32, i32),
  Unload(i32, i32),
}

#[derive(Component)]
pub struct SpawnedChunk([i32; 2]);

#[derive(Default)]
pub struct LoadedChunkTable(pub HashMap<[i32; 2], Entity>);

pub struct WorldGenPlugin;

impl WorldGenPlugin {
  pub fn spawn_chunk(
    commands: &mut Commands,
    generator: &WorldGenerator,
    world: &World,
    chunk_table: &mut LoadedChunkTable,
    position: [i32; 2],
  ) {
    if chunk_table.0.contains_key(&position) {
      return;
    }

    let ent = commands
      .spawn_bundle(SpriteBundle::default())
      .insert(SpawnedChunk(position))
      .with_children(|parent| {
        world
          .get_chunk(&generator, position)
          .into_iter()
          .enumerate()
          .for_each(|(i, tile)| {
            let [x, y] = World::get_tile_position_from_index(position, i);

            parent.spawn_bundle(SpriteBundle {
              sprite: Sprite {
                color: tile.get_tile_color(),
                custom_size: Some(Vec2::new(World::TILE_PIXEL_SIZE, World::TILE_PIXEL_SIZE)),
                ..default()
              },
              transform: Transform {
                translation: Vec2::from([x as f32 * World::TILE_PIXEL_SIZE, y as f32 * World::TILE_PIXEL_SIZE])
                  .extend(0.0),
                ..default()
              },
              ..default()
            });
          })
      })
      .id();

    chunk_table.0.insert(position, ent);
  }

  pub fn unload_chunk(commands: &mut Commands, chunk_table: &mut LoadedChunkTable, position: [i32; 2]) {
    if let Some(chunk) = chunk_table.0.remove(&position) {
      commands.entity(chunk).despawn_recursive();
    }
  }

  pub fn process_chunk_requests(
    mut commands: Commands,
    mut events: EventReader<ChunkRequests>,
    generator: Res<WorldGenerator>,
    mut chunk_table: ResMut<LoadedChunkTable>,
    world: Res<World>,
  ) {
    events.iter().for_each(|event| match event {
      ChunkRequests::Load(x, y) => Self::spawn_chunk(&mut commands, &generator, &world, &mut chunk_table, [*x, *y]),
      ChunkRequests::Unload(x, y) => Self::unload_chunk(&mut commands, &mut chunk_table, [*x, *y]),
    });
  }
}

impl Plugin for WorldGenPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<WorldGenerator>()
      .init_resource::<LoadedChunkTable>()
      .add_event::<ChunkRequests>()
      .add_system(Self::process_chunk_requests);
  }
}

#[cfg(test)]
mod tests {
  extern crate test;
  use chrono::NaiveDateTime;
  use test::{black_box, Bencher};

  use crate::db::models::{World, WorldObj};
  use crate::game::world::resources::WorldGenerator;

  #[bench]
  fn bench_chunk_gen(b: &mut Bencher) {
    let world = World::from(WorldObj {
      id: 0,
      origin_time: NaiveDateTime::MAX,
      seed: 0,
    });

    let generator = WorldGenerator::default();

    b.iter(|| {
      black_box(world.get_chunk(&generator, [0, 0]));
    });
  }
}
