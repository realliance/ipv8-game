use std::ops::Deref;

use bevy::prelude::*;
use hashbrown::HashMap;
use serde::Deserialize;
use uuid::Uuid;

use super::resources::{Resource, ResourceDelta};

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
pub struct BuildingDefinition {
  name: String,
  size: [i32; 2],
  priority: u32,
  placement: BuildingPlacementFlags,
  actions: Vec<BuildingAction>,
}

#[derive(Deserialize)]
pub struct BuildingDefinitionFile {
  pub buildings: Vec<BuildingDefinition>,
}

#[derive(Component)]
pub struct OwnedEntity(pub Uuid);

#[derive(Component)]
pub struct BuildingType(pub String);

pub struct Building {
  pub building_type: String,
  pub position: IVec2,
  pub owner: Uuid,
}

impl Building {
  pub fn new(building_type: String, position: IVec2, owner: Uuid) -> Self {
    Self {
      building_type,
      position,
      owner,
    }
  }

  pub fn build(self, mut commands: Commands) {
    commands
      .spawn()
      .insert(BuildingType(self.building_type))
      .insert(OwnedEntity(self.owner))
      .insert(Transform::from_xyz(self.position.x as f32, self.position.y as f32, 0.0));
  }
}

pub struct BuildingDefinitionTable(HashMap<String, BuildingDefinition>);

impl Deref for BuildingDefinitionTable {
  type Target = HashMap<String, BuildingDefinition>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

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
