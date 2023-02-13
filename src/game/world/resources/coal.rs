use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::gen::TerrainTile;

pub struct Coal;

impl WorldResource for Coal {
  fn priority(&self) -> u8 {
    25
  }

  fn name(&self) -> &str {
    "Coal"
  }

  fn terrain_tile(&self) -> TerrainTile {
    TerrainTile::Coal
  }

  fn get_tile(&self, world: &World, [x, y]: [i32; 2]) -> bool {
    const NOISE_SCALE: f64 = 0.08;

    let value = world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);

    value > 0.4
  }
}
