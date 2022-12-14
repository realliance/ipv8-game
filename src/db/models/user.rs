use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use hashbrown::HashMap;
use tracing::warn;
use uuid::Uuid;

use crate::db::schema::users;
use crate::db::PooledPgConnection;
use crate::game::resources::{Resource, ResourceDelta};

#[derive(Queryable, Identifiable, Insertable, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
  pub id: Uuid,
  pub credits: i64,
}

impl Default for User {
  fn default() -> Self {
    Self {
      id: Default::default(),
      credits: Default::default(),
    }
  }
}

impl User {
  pub fn new(conn: &mut PooledPgConnection, user_id: Uuid) -> Self {
    use crate::db::schema::users::dsl::*;

    if let Ok(found_user) = users.find(user_id).first::<User>(conn) {
      return found_user;
    } else {
      let new_user = User {
        id: user_id,
        ..Default::default()
      };

      new_user.save(conn).ok();

      return new_user;
    }
  }

  pub fn save(&self, conn: &mut PooledPgConnection) -> Result<(), diesel::result::Error> {
    use crate::db::schema::users::dsl::*;

    if let Ok(found_user) = users.find(self.id).first::<User>(conn) {
      match diesel::update(&found_user).set(credits.eq(self.credits)).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => {
          warn!("Error while updating user {}", err);
          Err(err)
        },
      }
    } else {
      match insert_into(users).values(self).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => {
          warn!("Error while inserting user {}", err);
          Err(err)
        },
      }
    }
  }

  pub fn get_all_users(conn: &mut PooledPgConnection) -> HashMap<Uuid, Self> {
    use crate::db::schema::users::dsl::*;

    let all_users = users
      .load::<Self>(conn)
      .expect("Failed to get all users, connection dead?");
    all_users.into_iter().map(|u| (u.id.clone(), u)).collect()
  }

  pub fn pay_resources(&mut self, delta: &ResourceDelta) -> bool {
    macro_rules! resource_cost {
      ($access:expr) => {
        if $access >= delta.value.abs() {
          $access += delta.value;
          true
        } else {
          false
        }
      };
    }

    match delta.resource {
      Resource::Watt => todo!(),
      Resource::Credit => resource_cost!(self.credits),
    }
  }

  pub fn pay_resource_transaction(&mut self, delta: Vec<ResourceDelta>) -> bool {
    let pay_attempt = delta
      .into_iter()
      .map(|x| (self.pay_resources(&x.as_cost()), x))
      .collect::<Vec<_>>();
    if !pay_attempt.iter().fold(true, |acc, (success, _)| acc && *success) {
      pay_attempt
        .into_iter()
        .filter(|(success, _)| *success)
        .for_each(|(_, delta)| {
          self.give_resources(&delta);
        });

      false
    } else {
      true
    }
  }

  pub fn give_resources(&mut self, delta: &ResourceDelta) {
    match delta.resource {
      Resource::Watt => todo!(),
      Resource::Credit => self.credits += delta.value,
    }
  }
}
