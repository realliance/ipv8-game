use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::gen::TerrainTile;
use crate::game::world::ComplexTerrainTile;

pub struct Coal;

impl WorldResource for Coal {
  fn priority(&self) -> u8 {
    25
  }

  fn name(&self) -> &str {
    "Coal"
  }

  fn terrain_tile(&self, world: &World, position: [i64; 2]) -> TerrainTile {
    let value = self.get_complex_tile_value(world, position, 1000..10000);
    TerrainTile::Complex(ComplexTerrainTile::Coal(value))
  }

  fn get_tile(&self, world: &World, position: [i64; 2], base_terrain_mod: f64) -> bool {
    self.get_value(world, position) - base_terrain_mod > 0.75
  }

  fn get_value(&self, world: &World, [x, y]: [i64; 2]) -> f64 {
    const NOISE_SCALE: f64 = 0.0333;
    world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE])
  }
}
