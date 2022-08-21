use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, PgConnection, QueryDsl, RunQueryDsl};
use rand::{thread_rng, Rng};
use tracing::info;

use crate::db::schema::worlds;

#[derive(Queryable, Identifiable, Clone, Debug)]
pub struct World {
  pub id: i32,
  pub origin_time: NaiveDateTime,
  pub seed: i64,
}

impl World {
  pub fn build() -> WorldBuilder {
    WorldBuilder {
      origin_time: Utc::now().naive_utc(),
      seed: thread_rng().gen(),
    }
  }

  pub fn from_db(conn: &PgConnection) -> Option<World> {
    use crate::db::schema::worlds::dsl::*;

    worlds.find(0).first::<World>(conn).ok()
  }
}

#[derive(Insertable)]
#[table_name = "worlds"]
pub struct WorldBuilder {
  origin_time: NaiveDateTime,
  seed: i64,
}

impl WorldBuilder {
  pub fn save(self, conn: &PgConnection) -> World {
    use crate::db::schema::worlds::dsl::*;

    if let Ok(found_world) = worlds.first::<World>(conn) {
      info!("Found existing world,");
      info!("{:?}", found_world);

      return found_world;
    }

    info!("Inserting a new world.");

    match insert_into(worlds).values(&self).get_results::<World>(conn) {
      Ok(new_world) => new_world
        .first()
        .expect("Expected to insert a world into the database but didn't")
        .clone(),
      Err(err) => {
        panic!(
          "Could not find current world nor insert new world. Game cannot run without a database-backed world. {}",
          err
        );
      },
    }
  }
}
