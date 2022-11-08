//! Game Actions
//! 
//! This module represents all actions that can be performed as a part of user intervention.

/// All game actions, performed by a given [User]
pub enum GameAction {
  /// Build a building
  BuildBuilding(String),
  /// Attempt to perform an action on a building
  PerformBuildingAction {
    building_entity_id: u32,
    action_index: u32,
  }
}