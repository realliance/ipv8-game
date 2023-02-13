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
  fn get_tile(&self, world: &World, position: [i32; 2]) -> bool;
}

pub struct WorldGenerator {
  world_resources: Vec<Box<dyn WorldResource>>,
}

impl WorldGenerator {
  pub fn new() -> Self {
    Self {
      world_resources: Vec::new(),
    }
  }

  pub fn add(mut self, resource: Box<dyn WorldResource>) -> Self {
    self.world_resources.push(resource);
    self.world_resources.sort_by(|a, b| b.priority().cmp(&a.priority()));
    Self {
      world_resources: self.world_resources,
    }
  }

  pub fn get_tile(&self, world: &World, pos: [i32; 2]) -> TerrainTile {
    self
      .world_resources
      .iter()
      .find(|x| x.get_tile(world, pos))
      .map(|x| x.terrain_tile())
      .unwrap_or(TerrainTile::Stone)
  }
}

impl Default for WorldGenerator {
  fn default() -> Self {
    WorldGenerator::new()
      .add(Box::new(Water))
      .add(Box::new(Copper))
      .add(Box::new(Iron))
      .add(Box::new(Coal))
      .add(Box::new(Impassable))
  }
}
