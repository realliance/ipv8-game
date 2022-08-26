use bevy::{prelude::*, time::FixedTimestep};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum GameStage {
  Start,
  OnTicked,
  OnResourcesPaid,
  Cleanup,
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
  fn build(&self, app: &mut App) {
    info!("Building Game Stages...");
    app
      .add_stage_after(
        CoreStage::Update, 
        GameStage::Start, 
        SystemStage::parallel().with_run_criteria(FixedTimestep::step(1.0))
      )
      .add_stage_after(
        GameStage::Start, 
        GameStage::OnTicked, 
        SystemStage::parallel().with_run_criteria(FixedTimestep::step(1.0))
      )
      .add_stage_after(
        GameStage::OnTicked, 
        GameStage::OnResourcesPaid, 
        SystemStage::parallel().with_run_criteria(FixedTimestep::step(1.0))
      )
      .add_stage_after(
        GameStage::OnResourcesPaid, 
        GameStage::Cleanup, 
        SystemStage::parallel().with_run_criteria(FixedTimestep::step(1.0))
      );
  }
}
