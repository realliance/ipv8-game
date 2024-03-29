use bevy::prelude::Resource;
use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, PgConnection, RunQueryDsl};
use noise::Perlin;
use rand::{thread_rng, Rng};
use tracing::info;

use crate::db::schema::worlds;

#[derive(Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = worlds)]
pub struct WorldObj {
  pub id: i32,
  pub origin_time: NaiveDateTime,
  pub seed: i64,
}

#[derive(Clone, Debug, Resource)]
pub struct World {
  pub id: i32,
  pub origin_time: NaiveDateTime,
  pub seed: i64,
  pub noise_gen: Perlin,
}

impl From<WorldObj> for World {
  fn from(value: WorldObj) -> Self {
    Self {
      id: value.id,
      origin_time: value.origin_time,
      seed: value.seed,
      noise_gen: Perlin::new(value.seed as u32),
    }
  }
}

impl World {
  pub fn build() -> WorldBuilder {
    WorldBuilder {
      origin_time: Utc::now().naive_utc(),
      seed: thread_rng().gen(),
    }
  }

  pub fn build_with_seed(seed: i64) -> WorldBuilder {
    WorldBuilder {
      origin_time: Utc::now().naive_utc(),
      seed,
    }
  }

  pub fn from_db(conn: &mut PgConnection) -> Option<World> {
    use crate::db::schema::worlds::dsl::*;

    worlds.first::<WorldObj>(conn).ok().map(|x| x.into())
  }

  pub fn reset_db(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    use crate::db::schema::worlds::dsl::*;

    diesel::delete(worlds).execute(conn).map(|_| ())
  }
}

#[derive(Insertable)]
#[diesel(table_name = worlds)]
pub struct WorldBuilder {
  origin_time: NaiveDateTime,
  seed: i64,
}

impl WorldBuilder {
  pub fn save(self, conn: &mut PgConnection) -> World {
    use crate::db::schema::worlds::dsl::*;

    if let Ok(found_world) = worlds.first::<WorldObj>(conn) {
      info!("Found existing world,");
      info!("{:?}", found_world);

      return found_world.into();
    }

    info!("Inserting a new world.");

    match insert_into(worlds).values(&self).get_results::<WorldObj>(conn) {
      Ok(new_world) => new_world
        .first()
        .expect("Expected to insert a world into the database but didn't")
        .clone()
        .into(),
      Err(err) => {
        panic!(
          "Could not find current world nor insert new world. Game cannot run without a database-backed world. {}",
          err
        );
      },
    }
  }
}
