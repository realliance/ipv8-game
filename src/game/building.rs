use std::fs;
use std::ops::Deref;

use bevy::prelude::*;
use glob::glob;
use hashbrown::HashMap;
use serde::Deserialize;
use uuid::Uuid;

use crate::game::stages::GameStage;

use super::resources::{ResourceDelta, TickedResourceCost};
use super::tick::Ticked;
use super::user::{UserOwned, UserResourceTable};

lazy_static::lazy_static! {
  /// All building definitions present in the game.
  pub static ref BUILDING_TABLE: HashMap<String, BuildingDefinition> = load_building_definitions();
}

/// Loads all building files into a single list
fn load_building_definitions() -> HashMap<String, BuildingDefinition> {
  glob("buildings/*.toml")
    .unwrap()
    .filter_map(|x| x.ok())
    .filter_map(|file| fs::read_to_string(file).ok())
    .filter_map(|contents| toml::from_str::<BuildingDefinitionFile>(&contents).ok())
    .map(|files| {
      files
        .buildings
        .iter()
        .for_each(|building| debug!("Registered Building {}", building.name));
      files.buildings
    })
    .flatten()
    .map(|x| (x.name.clone(), x))
    .collect()
}

#[derive(Deserialize, Clone)]
pub struct BuildingAction {
  id: String,
  #[allow(dead_code)]
  name: String,
  cooldown: u32,
  products: Option<Vec<ResourceDelta>>,
  costs: Option<Vec<ResourceDelta>>,
}

#[derive(Deserialize, Clone)]
pub struct BuildingPlacementFlags {
  pub on_water: bool,
  pub on_mineral: bool,
}

#[derive(Deserialize, Clone)]
pub struct BuildingTickedAction {
  every_n_ticks: u32,
  products: Option<Vec<ResourceDelta>>,
  costs: Option<Vec<ResourceDelta>>,
}

#[derive(Deserialize, Clone, Component)]
pub struct BuildingDefinition {
  pub name: String,
  pub size: [i32; 2],
  pub priority: u32,
  pub placement: BuildingPlacementFlags,
  pub actions: Option<Vec<BuildingAction>>,
  pub ticked: Option<Vec<BuildingTickedAction>>,
}

impl BuildingDefinition {
  pub fn spawn(&self, commands: &mut Commands, owner: Uuid, position: IVec2) -> Entity {
    let ent = commands
      .spawn()
      .insert(Building(self.name.clone()))
      .insert(UserOwned(owner))
      .insert(
        Transform::from_xyz(position.x as f32, position.y as f32, 0.0)
          .with_scale(Vec2::new(self.size[0] as f32, self.size[1] as f32).extend(0.0)),
      )
      .id();

    if let Some(ticked) = &self.ticked {
      ticked.iter().for_each(|x| {
        commands
          .entity(ent)
          .insert(Ticked::new(x.every_n_ticks))
          .insert(TickedResourceCost::new(x.costs.clone().unwrap_or_default()))
          .insert(BuildingTickedResourceProduct(x.products.clone().unwrap_or_default()));
      });
    }

    ent
  }
}

/// Component that represents a building with a specific name. This maps to
/// something in the Building definitions table.
#[derive(Component)]
pub struct Building(pub String);

#[derive(Deserialize)]
pub struct BuildingDefinitionFile {
  pub buildings: Vec<BuildingDefinition>,
}

pub struct BuildingDefinitionTable(HashMap<String, BuildingDefinition>);

impl Deref for BuildingDefinitionTable {
  type Target = HashMap<String, BuildingDefinition>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// Represents the products a building produces if the costs are paid.
#[derive(Component)]
pub struct BuildingTickedResourceProduct(pub Vec<ResourceDelta>);

fn on_tick_building_ticked_resources(
  mut user_table: ResMut<UserResourceTable>,
  ticked_owned_building: Query<(&TickedResourceCost, &BuildingTickedResourceProduct, &UserOwned), With<Building>>,
) {
  ticked_owned_building.for_each(|(cost, product, user_owned)| {
    if cost.paid() {
      if let Some(user) = user_table.get_mut(&user_owned.0) {
        product.0.iter().for_each(|delta| user.give_resources(delta));
      }
    }
  });
}

/// Represents that the building is on cooldown. This will automatically tick
/// down until it is deleted.
#[derive(Component, Debug)]
pub struct BuildingCooldown(pub u32);

fn tick_down_building_cooldowns(
  mut commands: Commands,
  mut building_cooldowns: Query<(Entity, &mut BuildingCooldown)>,
) {
  building_cooldowns.for_each_mut(|(e, mut cooldown)| {
    if cooldown.0 > 0 {
      cooldown.0 -= 1;
    } else {
      commands.entity(e).remove::<BuildingCooldown>();
    }
  });
}

/// Represents a request to complete an action on a given entity
#[derive(Component)]
pub struct BuildingPerformAction {
  pub user_origin: Uuid,
  pub id: String,
}

fn dismiss_actions_when_on_cooldown(
  mut commands: Commands,
  query: Query<Entity, (With<Building>, With<BuildingPerformAction>, With<BuildingCooldown>)>,
) {
  query.for_each(|e| {
    commands.entity(e).remove::<BuildingPerformAction>();
  });
}

fn process_actions(
  mut commands: Commands,
  mut user_table: ResMut<UserResourceTable>,
  query: Query<(Entity, &BuildingPerformAction, &Building, &UserOwned), Without<BuildingCooldown>>,
) {
  query.for_each(|(e, action_command, building, owner)| {
    if let Some(user) = user_table.get_mut(&owner.0)
    && let Some(building) = BUILDING_TABLE.get(&building.0)
    && let Some(action) = building.actions.as_ref().unwrap_or(&Vec::new()).iter().find(|x| x.id == action_command.id)
    && owner.0 == action_command.user_origin
    && user.pay_resource_transaction(action.costs.clone().unwrap_or(Vec::new())) {
      if let Some(products) = &action.products {
        products.iter().for_each(|x| {
          user.give_resources(x);
        });

        commands.entity(e).insert(BuildingCooldown(action.cooldown));
      }
    } else {
      warn!("Unable to perform action");
    }

    commands.entity(e).remove::<BuildingPerformAction>();
  });
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
  fn build(&self, app: &mut App) {
    info!("Loading Buildings...");
    info!("Buildings Loaded: {}", BUILDING_TABLE.len());

    app
      .insert_resource(BuildingDefinitionTable(BUILDING_TABLE.clone()))
      .add_system_to_stage(GameStage::OnResourcesPaid, on_tick_building_ticked_resources)
      .add_system_to_stage(GameStage::Start, tick_down_building_cooldowns)
      .add_system_to_stage(GameStage::OnResourcesPaid, process_actions)
      .add_system_to_stage(GameStage::OnTicked, dismiss_actions_when_on_cooldown);
  }
}

#[cfg(test)]
mod tests {
  use bevy::ecs::system::CommandQueue;
  use bevy::prelude::*;
  use hashbrown::HashMap;
  use uuid::Uuid;

  use super::{BuildingCooldown, BuildingPerformAction, BUILDING_TABLE};
  use crate::db::models::User;
  use crate::game::building::{Building, BuildingPlugin};
  use crate::game::resources::ResourcePlugin;
  use crate::game::stages::StagePlugin;
  use crate::game::tick::TickPlugin;
  use crate::game::user::UserResourceTable;
  use crate::properties::GameProperties;

  #[test]
  fn building_spawns() {
    // Build World
    let mut world = World::default();
    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &world);

    let owner = Uuid::new_v4();
    // Spawn from the building definitions
    BUILDING_TABLE["Headquarters"].spawn(commands, owner, IVec2 { x: 2, y: 4 });
    queue.apply(&mut world);

    assert_eq!(world.entities().len(), 1, "Building was not created");
    let building: Vec<&Building> = world.query::<&Building>().iter(&world).collect::<Vec<_>>();
    assert_eq!(building[0].0, "Headquarters");
  }

  #[test]
  fn building_idle_gen() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .add_plugin(ResourcePlugin)
      .add_plugin(BuildingPlugin)
      .init_resource::<GameProperties>();

    let id = Uuid::new_v4();
    let user = User {
      id: id.clone(),
      ..Default::default()
    };

    // Insert a User with Data
    app
      .world
      .insert_resource(UserResourceTable::new(HashMap::from([(user.id, user)])));

    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &app.world);

    BUILDING_TABLE["Headquarters"].spawn(commands, id.clone(), IVec2 { x: 0, y: 0 });
    queue.apply(&mut app.world);

    app.update();

    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 1);

    app.update();

    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 2);

    app.update();

    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 3);
  }

  #[test]
  fn building_cooldown() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .add_plugin(ResourcePlugin)
      .add_plugin(BuildingPlugin)
      .init_resource::<GameProperties>();

    let id = Uuid::new_v4();
    let user = User {
      id: id.clone(),
      ..Default::default()
    };

    // Insert a User with Data
    app
      .world
      .insert_resource(UserResourceTable::new(HashMap::from([(user.id, user)])));

    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &app.world);

    let ent = commands.spawn().insert(BuildingCooldown(5)).id();
    queue.apply(&mut app.world);

    for x in 0..5 {
      assert_eq!(app.world.entity(ent).get::<BuildingCooldown>().unwrap().0, 5 - x);
      app.update();
    }

    // Component deleted itself
    app.update();
    let cooldown = app.world.entity(ent).get::<BuildingCooldown>();
    assert!(
      cooldown.is_none(),
      "Expected to be deleted but instead was {:?}",
      cooldown
    );
  }

  #[test]
  fn building_perform_action() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .add_plugin(ResourcePlugin)
      .add_plugin(BuildingPlugin)
      .init_resource::<GameProperties>();

    let id = Uuid::new_v4();
    let user = User {
      id: id.clone(),
      credits: 1,
    };

    // Insert a User with Data
    app
      .world
      .insert_resource(UserResourceTable::new(HashMap::from([(user.id, user)])));

    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &app.world);

    let ent = BUILDING_TABLE["Headquarters"].spawn(commands, id.clone(), IVec2 { x: 0, y: 0 });
    commands.entity(ent).insert(BuildingPerformAction {
      user_origin: id,
      id: "increase_cash_flow".to_string(),
    });

    // Before spawn in
    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 1);

    queue.apply(&mut app.world);
    app.update();

    // Spawned in, action is processed and idle gen proced
    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 4);

    app.update();

    // Idle Gen
    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 5);
  }

  #[test]
  fn wont_perform_action_on_cooldown() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .add_plugin(ResourcePlugin)
      .add_plugin(BuildingPlugin)
      .init_resource::<GameProperties>();

    let id = Uuid::new_v4();
    let user = User {
      id: id.clone(),
      credits: 1,
    };

    // Insert a User with Data
    app
      .world
      .insert_resource(UserResourceTable::new(HashMap::from([(user.id, user)])));

    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &app.world);

    let ent = BUILDING_TABLE["Headquarters"].spawn(commands, id.clone(), IVec2 { x: 0, y: 0 });
    commands
      .entity(ent)
      .insert(BuildingPerformAction {
        user_origin: id,
        id: "increase_cash_flow".to_string(),
      })
      .insert(BuildingCooldown(5));

    // Start
    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 1);

    queue.apply(&mut app.world);
    app.update();

    // Spawned in, action didn't proc
    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 2);

    queue.apply(&mut app.world);
    app.update();

    // Verify won't proc
    let user_table: &UserResourceTable = app.world.get_resource().unwrap();
    assert_eq!(user_table.get(&id).unwrap().credits, 3);
  }
}
