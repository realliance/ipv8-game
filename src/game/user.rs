use std::ops::Deref;

use bevy::prelude::*;
use hashbrown::HashMap;
use uuid::Uuid;

use crate::db::{models::User, DatabasePool};

pub struct UserResourceTable(HashMap<Uuid, User>);

impl Deref for UserResourceTable {
  type Target = HashMap<Uuid, User>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct UserPlugin;

impl Plugin for UserPlugin {
  fn build(&self, app: &mut App) {
    info!("Building User Table");
    let database_connection: &DatabasePool = app.world.get_resource().expect("Failed to get DatabasePool. Ensure the DatabasePlugin is added before this plugin.");
    let conn = &database_connection.get().expect("Failed to get database connection from pool.");
    let all_users = User::get_all_users(conn);

    info!("Found users {:?}", all_users);

    app.insert_resource(UserResourceTable(all_users));
  }
}
