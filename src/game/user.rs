use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use hashbrown::HashMap;
use uuid::Uuid;

use crate::db::models::User;
use crate::db::DatabasePool;

#[derive(Clone)]
pub struct UserResourceTable(HashMap<Uuid, User>);

/// Used to represent a component that is owned by a user, for things such as paying recurring costs.
#[derive(Component)]
pub struct UserOwned(pub Uuid);

impl Deref for UserResourceTable {
  type Target = HashMap<Uuid, User>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for UserResourceTable {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub struct UserPlugin;

impl Plugin for UserPlugin {
  fn build(&self, app: &mut App) {
    info!("Building User Table");
    let database_connection: &DatabasePool = app
      .world
      .get_resource()
      .expect("Failed to get DatabasePool. Ensure the DatabasePlugin is added before this plugin.");
    let conn = &database_connection
      .get()
      .expect("Failed to get database connection from pool.");
    let all_users = User::get_all_users(conn);

    info!("Found users {:?}", all_users);

    app.insert_resource(UserResourceTable(all_users));
  }
}
