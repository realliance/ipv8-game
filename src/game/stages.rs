use bevy::prelude::*;

#[cfg(not(test))]
use bevy::time::FixedTimestep;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum GameStage {
  Start,
  OnTicked,
  OnResourcesPaid,
  Cleanup,
}

#[cfg(test)]
fn get_system_stage() -> SystemStage {
  SystemStage::single_threaded()
}

#[cfg(not(test))]
fn get_system_stage() -> SystemStage {
  SystemStage::parallel().with_run_criteria(FixedTimestep::step(1.0))
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
  fn build(&self, app: &mut App) {
    info!("Building Game Stages...");
    app
      .add_stage_after(
        CoreStage::Update, 
        GameStage::Start,
        get_system_stage()
      )
      .add_stage_after(
        GameStage::Start, 
        GameStage::OnTicked, 
        get_system_stage()
      )
      .add_stage_after(
        GameStage::OnTicked, 
        GameStage::OnResourcesPaid, 
        get_system_stage()
      )
      .add_stage_after(
        GameStage::OnResourcesPaid, 
        GameStage::Cleanup, 
        get_system_stage()
      );
  }
}
