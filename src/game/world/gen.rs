use bevy::prelude::*;
use hashbrown::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;

use super::resources::*;
use crate::db::models::{Chunk, World};
use crate::db::{AcquiredDatabaseConnection, DatabaseManager};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticTerrainTile {
  Water = 0,
  Stone = 1,
  Impassable = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexTerrainTile {
  Copper(u32),
  Coal(u32),
  Iron(u32),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainTile {
  Static(StaticTerrainTile),
  Complex(ComplexTerrainTile),
}

impl TerrainTile {
  pub fn get_tile_color(&self) -> Color {
    match self {
      Self::Static(StaticTerrainTile::Water) => Color::BLUE,
      Self::Static(StaticTerrainTile::Stone) => Color::GRAY,
      Self::Complex(ComplexTerrainTile::Iron(_)) => Color::SILVER,
      Self::Complex(ComplexTerrainTile::Copper(_)) => Color::ORANGE,
      Self::Complex(ComplexTerrainTile::Coal(_)) => Color::DARK_GRAY,
      Self::Static(StaticTerrainTile::Impassable) => Color::BLACK,
    }
  }

  pub fn into_chunk_tile_id(&self) -> u8 {
    match self {
      Self::Static(StaticTerrainTile::Water) => 0,
      Self::Static(StaticTerrainTile::Stone) => 1,
      Self::Static(StaticTerrainTile::Impassable) => 2,
      Self::Complex(ComplexTerrainTile::Iron(_)) => 3,
      Self::Complex(ComplexTerrainTile::Copper(_)) => 4,
      Self::Complex(ComplexTerrainTile::Coal(_)) => 5,
    }
  }

  pub fn get_metadata(&self) -> Option<u32> {
    match self {
      Self::Complex(ComplexTerrainTile::Iron(metadata)) => Some(*metadata),
      Self::Complex(ComplexTerrainTile::Copper(metadata)) => Some(*metadata),
      Self::Complex(ComplexTerrainTile::Coal(metadata)) => Some(*metadata),
      _ => None,
    }
  }

  pub fn from_chunk_tile_id_and_metadata(chunk_tile_id: u8, metadata: Option<u32>) -> Option<Self> {
    match (chunk_tile_id, metadata) {
      (0, _) => Some(Self::Static(StaticTerrainTile::Water)),
      (1, _) => Some(Self::Static(StaticTerrainTile::Stone)),
      (2, _) => Some(Self::Static(StaticTerrainTile::Impassable)),
      (3, Some(metadata)) => Some(Self::Complex(ComplexTerrainTile::Iron(metadata))),
      (4, Some(metadata)) => Some(Self::Complex(ComplexTerrainTile::Copper(metadata))),
      (5, Some(metadata)) => Some(Self::Complex(ComplexTerrainTile::Coal(metadata))),
      _ => None,
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
  pub fn get_tile_position_from_index([c_x, c_y]: [i64; 2], index: usize) -> [i64; 2] {
    let x = (index % World::CHUNK_SIDE_LENGTH) as i64 + (c_x * World::CHUNK_SIDE_LENGTH as i64);
    let y = (index / World::CHUNK_SIDE_LENGTH) as i64 + (c_y * World::CHUNK_SIDE_LENGTH as i64);
    [x, y]
  }

  /// Returns position of tile from within the chunk.
  #[inline(always)]
  pub fn get_localized_tile_position_form_index(index: usize) -> [i32; 2] {
    let x = (index % World::CHUNK_SIDE_LENGTH) as i32;
    let y = (index / World::CHUNK_SIDE_LENGTH) as i32;
    [x, y]
  }

  /// Returns the index of a tile in a chunk list given it's offset coordinates.
  #[inline(always)]
  pub fn get_chunk_index([x_offset, y_offset]: [u32; 2]) -> usize {
    y_offset as usize * World::CHUNK_SIDE_LENGTH as usize + x_offset as usize
  }

  /// Returns a chunk for the given world, using the properties of the provided
  /// world generator.
  pub fn get_chunk(&self, generator: &WorldGenerator, [x, y]: [i64; 2]) -> [TerrainTile; Self::CHUNK_SIZE] {
    let x = x * Self::CHUNK_SIDE_LENGTH as i64;
    let y = y * Self::CHUNK_SIDE_LENGTH as i64;

    let mut chunk = [TerrainTile::Static(StaticTerrainTile::Stone); Self::CHUNK_SIZE];

    CHUNK_TABLE.iter().for_each(|(x_offset, y_offset)| {
      chunk[Self::get_chunk_index([*x_offset, *y_offset])] =
        generator.get_tile(&self, [x + *x_offset as i64, y + *y_offset as i64])
    });

    chunk
  }
}

#[derive(Component)]
pub struct SpawnedChunk([i64; 2]);

pub struct LoadedChunk {
  pub chunk: [TerrainTile; World::CHUNK_SIZE],
  pub spawned_entity: Option<Entity>,
}

#[derive(Default, Resource)]
pub struct LoadedChunkTable(HashMap<[i64; 2], LoadedChunk>);

impl LoadedChunkTable {
  pub fn get(
    &mut self,
    conn: Option<AcquiredDatabaseConnection>,
    generator: &WorldGenerator,
    world: &World,
    position: [i64; 2],
  ) -> &LoadedChunk {
    if !self.0.contains_key(&position) {
      let [chunk_x, chunk_y] = position;
      if let Some(mut conn) = conn {
        if let Ok(chunk) = Chunk::from_xy(&mut *conn, chunk_x, chunk_y) {
          self.0.insert(
            position,
            LoadedChunk {
              chunk,
              spawned_entity: None,
            },
          );
        } else {
          self.gen(generator, world, position);
          Chunk::save_chunk(&mut *conn, chunk_x, chunk_y, &self.0.get(&position).unwrap().chunk).unwrap()
        }
      } else {
        self.gen(generator, world, position);
      }
    }

    self.0.get(&position).unwrap()
  }

  pub fn gen(&mut self, generator: &WorldGenerator, world: &World, position: [i64; 2]) {
    let chunk = world.get_chunk(generator, position);
    self.0.insert(
      position,
      LoadedChunk {
        chunk,
        spawned_entity: None,
      },
    );
  }

  pub fn update_entity(&mut self, position: [i64; 2], ent: Entity) {
    if let Some(loaded_chunk) = self.0.get_mut(&position) {
      loaded_chunk.spawned_entity = Some(ent);
    }
  }

  pub fn get_mut_if_exists(&mut self, position: [i64; 2]) -> Option<&mut LoadedChunk> {
    self.0.get_mut(&position)
  }

  pub fn get_if_exists(&self, position: [i64; 2]) -> Option<&LoadedChunk> {
    self.0.get(&position)
  }
}

pub struct WorldGenPlugin;

#[derive(Component)]
pub struct LoadChunkCommand(pub [i64; 2]);

#[derive(Component)]
pub struct UnloadChunkCommand(pub [i64; 2]);

impl WorldGenPlugin {
  pub fn spawn_chunk(
    mut commands: Commands,
    generator: Res<WorldGenerator>,
    world: Res<World>,
    database: Res<DatabaseManager>,
    mut chunk_table: ResMut<LoadedChunkTable>,
    query: Query<(Entity, &LoadChunkCommand)>,
  ) {
    query.for_each(|(command_ent, chunk_command)| {
      let position = chunk_command.0;
      commands.entity(command_ent).despawn();

      let loaded_chunk = {
        let conn = database.try_take().ok();
        chunk_table.get(conn, &generator, &world, position)
      };

      if loaded_chunk.spawned_entity.is_some() {
        return;
      }

      let ent = commands
        .spawn(SpriteBundle::default())
        .insert(SpawnedChunk(position))
        .with_children(|parent| {
          loaded_chunk.chunk.iter().enumerate().for_each(|(i, tile)| {
            let [x, y] = World::get_tile_position_from_index(position, i);

            parent.spawn(SpriteBundle {
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

      chunk_table.update_entity(position, ent);
    });
  }

  pub fn despawn_chunk(
    mut commands: Commands,
    mut chunk_table: ResMut<LoadedChunkTable>,
    query: Query<(Entity, &UnloadChunkCommand)>,
  ) {
    query.for_each(|(command_ent, chunk_command)| {
      if let Some(loaded_chunk) = chunk_table.get_mut_if_exists(chunk_command.0) {
        if loaded_chunk.spawned_entity.is_some() {
          commands
            .entity(loaded_chunk.spawned_entity.unwrap())
            .despawn_recursive();
          loaded_chunk.spawned_entity = None;
        }
      }
      commands.entity(command_ent).despawn();
    });
  }
}

impl Plugin for WorldGenPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<WorldGenerator>()
      .init_resource::<LoadedChunkTable>()
      .add_system(Self::spawn_chunk)
      .add_system(Self::despawn_chunk);
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
