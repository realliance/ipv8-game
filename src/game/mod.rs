use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use world::WorldPlugin;

use self::user::UserPlugin;

mod world;
mod user;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(WorldPlugin)
      .add(UserPlugin);
  }
}
