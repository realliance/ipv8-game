use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use world::WorldPlugin;

mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group.add(WorldPlugin);
  }
}
