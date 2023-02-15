use diesel::{insert_into, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use hashbrown::HashMap;

use crate::db::complex_tiles;
use crate::db::models::World;
use crate::game::world::TerrainTile;

#[derive(Queryable, Insertable, Clone, Debug)]
#[diesel(table_name = complex_tiles)]
pub struct ComplexTile {
  pub chunk_x: i64,
  pub chunk_y: i64,
  pub x: i32,
  pub y: i32,
  pub metadata: i64,
}

impl ComplexTile {
  pub fn from_chunk(
    conn: &mut PgConnection,
    x_pos: i64,
    y_pos: i64,
  ) -> Result<HashMap<[i32; 2], u32>, diesel::result::Error> {
    use crate::db::schema::complex_tiles::dsl::*;

    Ok(
      complex_tiles
        .filter(chunk_x.eq(x_pos))
        .filter(chunk_y.eq(y_pos))
        .get_results::<ComplexTile>(conn)?
        .into_iter()
        .map(|tile_list| ([tile_list.x, tile_list.y], tile_list.metadata as u32))
        .collect(),
    )
  }

  pub fn save_chunk(
    conn: &mut PgConnection,
    x_pos: i64,
    y_pos: i64,
    chunk: &[TerrainTile],
  ) -> Result<(), diesel::result::Error> {
    use crate::db::schema::complex_tiles::dsl::*;

    let inserts = chunk
      .iter()
      .enumerate()
      .filter_map(|(index, tile)| {
        tile.get_metadata().map(|mdata| {
          let [tile_x, tile_y] = World::get_localized_tile_position_form_index(index);
          (tile_x, tile_y, mdata)
        })
      })
      .map(|(tile_x, tile_y, mdata)| ComplexTile {
        chunk_x: x_pos,
        chunk_y: y_pos,
        x: tile_x,
        y: tile_y,
        metadata: mdata as i64,
      })
      .collect::<Vec<_>>();

    insert_into(complex_tiles).values(inserts).execute(conn).map(|_| ())
  }
}
