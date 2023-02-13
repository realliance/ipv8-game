use crate::db::models::World;

mod water;
pub use water::*;

mod copper;
pub use copper::*;

mod iron;
pub use iron::*;

mod coal;
pub use coal::*;

mod impassable;
pub use impassable::*;

use super::gen::TerrainTile;

pub trait WorldResource: Send + Sync {
  fn terrain_tile(&self) -> TerrainTile;
  fn priority(&self) -> u8;
  fn name(&self) -> &str;
  fn get_value(&self, world: &World, position: [i32; 2]) -> f64;
  fn get_tile(&self, world: &World, position: [i32; 2], base_terrain_modifier: f64) -> bool;
}

pub struct WorldGenerator {
  base_terrain: Vec<Box<dyn WorldResource>>,
  world_resources: Vec<Box<dyn WorldResource>>,
}

impl WorldGenerator {
  pub fn new() -> Self {
    Self {
      base_terrain: Vec::new(),
      world_resources: Vec::new(),
    }
  }

  pub fn add_base(mut self, resource: Box<dyn WorldResource>) -> Self {
    self.base_terrain.push(resource);
    self.base_terrain.sort_by(|a, b| b.priority().cmp(&a.priority()));
    Self {
      base_terrain: self.base_terrain,
      world_resources: self.world_resources,
    }
  }
  pub fn add(mut self, resource: Box<dyn WorldResource>) -> Self {
    self.world_resources.push(resource);
    self.world_resources.sort_by(|a, b| b.priority().cmp(&a.priority()));
    Self {
      base_terrain: self.base_terrain,
      world_resources: self.world_resources,
    }
  }

  pub fn get_base_terrain_modifier(&self, world: &World, pos: [i32; 2]) -> f64 {
    self
      .base_terrain
      .iter()
      .find(|x| x.get_tile(world, pos, -0.1))
      .map(|x| x.get_value(world, pos))
      .unwrap_or(0.0)
  }

  pub fn get_tile(&self, world: &World, pos: [i32; 2]) -> TerrainTile {
    if let Some(base_terrain) = self.base_terrain.iter().find(|x| x.get_tile(world, pos, 0.0)) {
      return base_terrain.terrain_tile();
    }

    let base_terrain_mod = self.get_base_terrain_modifier(world, pos);

    self
      .world_resources
      .iter()
      .find(|x| x.get_tile(world, pos, base_terrain_mod))
      .map(|x| x.terrain_tile())
      .unwrap_or(TerrainTile::Stone)
  }
}

impl Default for WorldGenerator {
  fn default() -> Self {
    WorldGenerator::new()
      .add(Box::new(Copper))
      .add(Box::new(Iron))
      .add(Box::new(Coal))
      .add_base(Box::new(Water))
      .add_base(Box::new(Impassable))
  }
}
