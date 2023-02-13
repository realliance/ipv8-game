use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::gen::TerrainTile;

pub struct Impassable;

impl WorldResource for Impassable {
  fn priority(&self) -> u8 {
    100
  }

  fn name(&self) -> &str {
    "Impassable"
  }

  fn terrain_tile(&self) -> TerrainTile {
    TerrainTile::Impassable
  }

  fn get_tile(&self, world: &World, [x, y]: [i32; 2]) -> bool {
    const NOISE_SCALE: f64 = 0.01;

    let value = world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);

    value > 0.4
  }
}
