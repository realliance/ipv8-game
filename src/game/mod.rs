use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use world::WorldPlugin;

use self::building::BuildingPlugin;
use self::stages::StagePlugin;
use self::tick::TickPlugin;
use self::user::UserPlugin;

pub mod building;
pub mod resources;
pub mod user;
pub mod world;
pub mod stages;
pub mod tick;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(StagePlugin)
      .add(TickPlugin)
      .add(BuildingPlugin)
      .add(WorldPlugin)
      .add(UserPlugin);
  }
}
