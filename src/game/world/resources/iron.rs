use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::gen::TerrainTile;
use crate::game::world::ComplexTerrainTile;

pub struct Iron;

impl WorldResource for Iron {
  fn priority(&self) -> u8 {
    6
  }

  fn name(&self) -> &str {
    "Iron"
  }

  fn terrain_tile(&self, world: &World, position: [i32; 2]) -> TerrainTile {
    let value = self.get_complex_tile_value(world, position, 2000..8000);
    TerrainTile::Complex(ComplexTerrainTile::Iron(value))
  }

  fn get_tile(&self, world: &World, position: [i32; 2], base_terrain_mod: f64) -> bool {
    self.get_value(world, position) - base_terrain_mod > 0.8
  }

  fn get_value(&self, world: &World, [x, y]: [i32; 2]) -> f64 {
    const NOISE_SCALE: f64 = 0.03;
    world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE])
  }
}
