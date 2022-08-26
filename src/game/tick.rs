use bevy::prelude::*;

use crate::game::stages::GameStage;

/// A component that is ticked every game step. use `::fired(&self)` to check whether
/// the tick has fired and work should be done. When querying for [Ticked] use a label
/// component alongside to filter better.
#[derive(Component)]
pub struct Ticked {
  counter: u16,
  count_to: u16,
  fired: bool,
}

impl Ticked {
  pub fn new(every_n_ticks: u16) -> Self {
    Self {
      counter: 0,
      count_to: every_n_ticks,
      fired: false,
    }
  }

  pub fn every_tick() -> Self {
    Self {
      counter: 0,
      count_to: 0,
      fired: true,
    }
  }

  /// Increments the tick counter.
  pub fn inc(&mut self) {
    if self.count_to == 0 {
      return;
    }

    self.fired = false;
    self.counter += 1;
    if self.counter >= self.count_to {
      self.counter = 0;
      self.fired = true;
    }
  }

  /// Has this component been fired this game tick?
  pub fn fired(&self) -> bool {
    self.fired
  }
}

fn tick_system(mut query: Query<&mut Ticked>) {
  query.par_for_each_mut(32, |mut ticked| {
    ticked.inc();
  });
}

pub struct TickPlugin;

impl Plugin for TickPlugin {
  fn build(&self, app: &mut App) {
    info!("Loading Tick System...");
    app
      .add_system_to_stage(GameStage::Start, tick_system);
  }
}
