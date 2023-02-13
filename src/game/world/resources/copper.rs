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

  fn get_tile(&self, world: &World, [x, y]: [i32; 2]) -> bool {
    const NOISE_SCALE: f64 = 0.15;

    let value = world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);

    value > 0.6
  }
}
