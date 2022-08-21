use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use world::WorldPlugin;

use self::building::BuildingPlugin;
use self::user::UserPlugin;

mod building;
mod resources;
mod user;
mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group.add(BuildingPlugin).add(WorldPlugin).add(UserPlugin);
  }
}
