use std::ops::Deref;

use bevy::prelude::*;
use hashbrown::HashMap;
use serde::Deserialize;
use uuid::Uuid;

use super::{resources::{ResourceDelta, TickedResourceCost}, user::UserOwned, tick::Ticked};

lazy_static::lazy_static! {
  pub static ref BUILDINGS: Vec<BuildingDefinition> = toml::from_str::<BuildingDefinitionFile>(&include_str!("../../buildings.toml")).unwrap().buildings;
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
      .insert(self.clone())
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

/// Represents the products a building produces if the costs are paid.
#[derive(Component)]
pub struct BuildingTickedResourceProduct(pub Vec<ResourceDelta>);

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

// TODO add function to consume [BuildingTickedResourceProduct] and give it to players.

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
  fn build(&self, app: &mut App) {
    info!("Loading Buildings...");
    let building_table: HashMap<String, BuildingDefinition> =
      BUILDINGS.clone().into_iter().map(|x| (x.name.clone(), x)).collect();
    info!("Buildings Loaded: {}", building_table.len());

    app.insert_resource(BuildingDefinitionTable(building_table));
  }
}
