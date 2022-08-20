use bevy::prelude::*;

use crate::db::models::World;
use crate::db::DatabasePool;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    info!("Building World");

    let database_connection: &DatabasePool = app.world.resource();
    let conn = &database_connection.get().unwrap();
    let world = World::from_db(conn).unwrap_or(World::build().save(conn));

    app.insert_resource(world);
  }
}
