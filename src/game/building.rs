use std::{ops::Deref, fs};

use bevy::prelude::*;
use glob::glob;
use hashbrown::HashMap;
use serde::Deserialize;
use uuid::Uuid;

use super::{resources::{ResourceDelta, TickedResourceCost}, user::{UserOwned, UserResourceTable}, tick::Ticked};

lazy_static::lazy_static! {
  /// All building definitions present in the game.
  pub static ref BUILDINGS: Vec<BuildingDefinition> = load_building_definitions();
}

/// Loads all building files into a single list
fn load_building_definitions() -> Vec<BuildingDefinition> {
  glob("buildings/*.toml")
    .unwrap()
    .filter_map(|x| x.ok())
    .filter_map(|file| fs::read_to_string(file).ok())
    .filter_map(|contents| toml::from_str::<BuildingDefinitionFile>(&contents).ok())
    .map(|files| {
      files.buildings.iter().for_each(|building| debug!("Registered Building {}", building.name));
      files.buildings
    })
    .flatten()
    .collect()
}

#[derive(Deserialize, Clone)]
pub struct BuildingAction {
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
  pub fn spawn(&self, commands: &mut Commands, owner: Uuid, position: IVec2) {
    let ent = commands
      .spawn()
      .insert(Building(self.name.clone()))
      .insert(UserOwned(owner))
      .insert(
        Transform::from_xyz(position.x as f32, position.y as f32, 0.0)
          .with_scale(Vec2::new(self.size[0] as f32, self.size[1] as f32).extend(0.0))
      ).id();

    if let Some(ticked) = &self.ticked {
      ticked.iter().for_each(|x| {
        commands.entity(ent)
          .insert(Ticked::new(x.every_n_ticks))
          .insert(TickedResourceCost::new(x.costs.clone().unwrap_or_default()))
          .insert(BuildingTickedResourceProduct(x.products.clone().unwrap_or_default()));
      });
    }
  }
}

/// Component that represents a building with a specific name. This maps to something in the Building definitions table.
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

fn on_tick_building_ticked_resources(mut user_table: ResMut<UserResourceTable>, ticked_owned_building: Query<(&TickedResourceCost, &BuildingTickedResourceProduct, &UserOwned), With<Building>>) {
  ticked_owned_building.for_each(|(cost, product, user_owned)| {
    if cost.paid() {
      if let Some(user) = user_table.get_mut(&user_owned.0) {
        product.0.iter().for_each(|delta| user.give_resources(delta));
      }
    }
  });
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
  fn build(&self, app: &mut App) {
    info!("Loading Buildings...");
    let building_table: HashMap<String, BuildingDefinition> =
      BUILDINGS.clone().into_iter().map(|x| (x.name.clone(), x)).collect();
    info!("Buildings Loaded: {}", building_table.len());

    app
      .insert_resource(BuildingDefinitionTable(building_table))
      .add_system(on_tick_building_ticked_resources);
  }
}

#[cfg(test)]
mod tests {
  use bevy::{prelude::*, ecs::system::CommandQueue};
  use uuid::Uuid;

  use crate::{game::building::Building};

use super::BUILDINGS;

  #[test]
  fn building_spawns() {
    // Build World
    let mut world = World::default();
    let mut queue = CommandQueue::default();
    let commands = &mut Commands::new(&mut queue, &world);

    let owner = Uuid::new_v4();
    let building_name = BUILDINGS[0].name.clone();
    // Spawn from the building definitions
    BUILDINGS[0].spawn(commands, owner, IVec2 { x: 2, y: 4 });
    queue.apply(&mut world);

    assert_eq!(world.entities().len(), 1, "Building was not created");
    let building: Vec<&Building> = world.query::<&Building>().iter(&world).collect::<Vec<_>>();
    assert_eq!(building[0].0, building_name);
  }
}
