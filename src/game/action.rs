//! Game Actions
//!
//! This module represents all actions that can be performed as a part of user
//! intervention.

use bevy::prelude::*;
use uuid::Uuid;

use super::building::{BuildingPerformAction, BUILDING_TABLE};
use super::stages::GameStage;

/// All game actions, performed by a given [User]
pub enum GameAction {
  /// Build a building
  BuildBuilding { building_id: String, position: IVec2 },
  /// Attempt to perform an action on a building
  PerformBuildingAction { building_entity_id: u32, action_id: String },
}

/// An action tied to a [User]
pub struct UserGameAction {
  pub user_id: Uuid,
  pub action: GameAction,
}

pub fn process_game_actions(mut commands: Commands, mut events: EventReader<UserGameAction>) {
  events
    .iter()
    .for_each(|user_game_action| match &user_game_action.action {
      GameAction::BuildBuilding { building_id, position } => {
        if let Some(building_def) = BUILDING_TABLE.get(building_id) {
          building_def.spawn(&mut commands, user_game_action.user_id, *position);
        } else {
          warn!("Attempted to spawn unknown building with id {}", building_id);
        }
      },
      GameAction::PerformBuildingAction {
        building_entity_id,
        action_id,
      } => {
        commands
          .entity(Entity::from_raw(*building_entity_id))
          .insert(BuildingPerformAction {
            user_origin: user_game_action.user_id,
            id: action_id.clone(),
          });
      },
    });
}

pub struct GameActionPlugin;

impl Plugin for GameActionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<UserGameAction>()
      .add_system_to_stage(GameStage::Start, process_game_actions);
  }
}
