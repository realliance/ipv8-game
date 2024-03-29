//! Game Login and Systems.

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use world::WorldPlugin;

use self::building::BuildingPlugin;
use self::resources::ResourcePlugin;
use self::stages::StagePlugin;
use self::tick::TickPlugin;
use self::user::UserPlugin;

pub mod action;
pub mod building;
pub mod resources;
pub mod stages;
pub mod tick;
pub mod user;
pub mod world;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(WorldPlugin)
      .add(StagePlugin)
      .add(ResourcePlugin)
      .add(TickPlugin)
      .add(BuildingPlugin)
      .add(UserPlugin)
  }
}
