use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::gen::TerrainTile;

pub struct Copper;

impl WorldResource for Copper {
  fn priority(&self) -> u8 {
    5
  }

  fn name(&self) -> &str {
    "Copper"
  }

  fn terrain_tile(&self) -> TerrainTile {
    TerrainTile::Copper
  }


  fn get_tile(&self, world: &World, position: [i32; 2], base_terrain_mod: f64) -> bool {
    self.get_value(world, position) - base_terrain_mod > 0.8
  }

  fn get_value(&self, world: &World, [x, y]: [i32; 2]) -> f64 {
    const NOISE_SCALE: f64 = 0.04;
    world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE])
  }
}
