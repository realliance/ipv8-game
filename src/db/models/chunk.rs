use diesel::{insert_into, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use tracing::warn;

use super::World;
use crate::db::chunks;
use crate::db::models::ComplexTile;
use crate::game::world::{StaticTerrainTile, TerrainTile};

#[derive(Debug)]
pub enum ChunkError {
  ChunkMalformed,
  DieselError(diesel::result::Error),
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = chunks)]
pub struct Chunk {
  pub x: i64,
  pub y: i64,
  pub tiles: Vec<u8>,
}

impl Chunk {
  pub fn from_xy(
    conn: &mut PgConnection,
    chunk_x: i64,
    chunk_y: i64,
  ) -> Result<[TerrainTile; World::CHUNK_SIZE], ChunkError> {
    use crate::db::schema::chunks::dsl::*;

    let chunk = chunks
      .filter(x.eq(chunk_x))
      .filter(y.eq(chunk_y))
      .first::<Chunk>(conn)
      .map_err(ChunkError::DieselError)?;

    if chunk.tiles.len() != World::CHUNK_SIZE {
      return Err(ChunkError::ChunkMalformed);
    }

    let complex_tiles = ComplexTile::from_chunk(conn, chunk_x, chunk_y).map_err(ChunkError::DieselError)?;

    let mut chunk_array = [TerrainTile::Static(StaticTerrainTile::Stone); World::CHUNK_SIZE];
    chunk
      .tiles
      .into_iter()
      .enumerate()
      .map(|(index, tile)| {
        let pos = World::get_localized_tile_position_form_index(index);
        TerrainTile::from_chunk_tile_id_and_metadata(tile, complex_tiles.get(&pos).map(|val| *val)).unwrap_or_else(
          || {
            warn!("Failed to parse tile from db chunk at {:?}. Tile {}", pos, tile);
            TerrainTile::Static(StaticTerrainTile::Stone)
          },
        )
      })
      .enumerate()
      .for_each(|(index, tile)| {
        chunk_array[index] = tile;
      });

    Ok(chunk_array)
  }

  pub fn save_chunk(
    conn: &mut PgConnection,
    chunk_x: i64,
    chunk_y: i64,
    chunk_tiles: &[TerrainTile],
  ) -> Result<(), diesel::result::Error> {
    use crate::db::schema::chunks::dsl::*;

    ComplexTile::save_chunk(conn, chunk_x, chunk_y, chunk_tiles)?;

    let tile_ids = chunk_tiles
      .iter()
      .map(|tile| tile.into_chunk_tile_id())
      .collect::<Vec<_>>();

    let chunk = Chunk {
      x: chunk_x,
      y: chunk_y,
      tiles: tile_ids,
    };

    insert_into(chunks).values(&chunk).execute(conn).map(|_| ())
  }
}
