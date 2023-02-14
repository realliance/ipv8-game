use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::StaticTerrainTile;
use crate::game::world::gen::TerrainTile;

pub struct Water;

impl WorldResource for Water {
  fn priority(&self) -> u8 {
    99
  }

  fn name(&self) -> &str {
    "Water"
  }

  fn terrain_tile(&self, _: &World, _: [i32; 2]) -> TerrainTile {
    TerrainTile::Static(StaticTerrainTile::Water)
  }

  fn get_tile(&self, world: &World, position: [i32; 2], base_terrain_mod: f64) -> bool {
    self.get_value(world, position) - base_terrain_mod > 0.5
  }

  fn get_value(&self, world: &World, [x, y]: [i32; 2]) -> f64 {
    const NOISE_SCALE: f64 = 0.02;
    world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE])
  }
}
