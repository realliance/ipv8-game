use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use world::WorldPlugin;

use self::building::BuildingPlugin;
use self::stages::StagePlugin;
use self::tick::TickPlugin;
use self::user::UserPlugin;

mod building;
mod resources;
mod user;
mod world;
mod stages;
mod tick;

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
