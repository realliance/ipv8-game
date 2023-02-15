use noise::NoiseFn;

use super::WorldResource;
use crate::db::models::World;
use crate::game::world::gen::TerrainTile;
use crate::game::world::StaticTerrainTile;

pub struct Impassable;

impl WorldResource for Impassable {
  fn priority(&self) -> u8 {
    100
  }

  fn name(&self) -> &str {
    "Impassable"
  }

  fn terrain_tile(&self, _: &World, _: [i64; 2]) -> TerrainTile {
    TerrainTile::Static(StaticTerrainTile::Impassable)
  }

  fn get_tile(&self, world: &World, position: [i64; 2], base_terrain_mod: f64) -> bool {
    self.get_value(world, position) - base_terrain_mod > 0.4
  }

  fn get_value(&self, world: &World, [x, y]: [i64; 2]) -> f64 {
    const NOISE_SCALE: f64 = 0.01;
    world.noise_gen.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE])
  }
}
