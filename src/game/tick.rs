use bevy::prelude::*;

use crate::{game::stages::GameStage, properties::GameProperties};

/// A component that is ticked every game step. use `::fired(&self)` to check whether
/// the tick has fired and work should be done. When querying for [Ticked] use a label
/// component alongside to filter better.
#[derive(Component)]
pub struct Ticked {
  counter: u32,
  count_to: u32,
  fired: u32,
}

impl Ticked {
  pub fn new(every_n_ticks: u32) -> Self {
    Self {
      counter: 0,
      count_to: every_n_ticks,
      fired: 0,
    }
  }

  pub fn every_tick() -> Self {
    Self {
      counter: 0,
      count_to: 0,
      fired: 0,
    }
  }

  /// Increments the tick counter.
  pub fn inc(&mut self, properties: &GameProperties) {
    if self.count_to == 0 {
      self.fired = properties.tick_speed;
      return;
    }

    self.fired = 0;
    self.counter += properties.tick_speed;
    while self.counter >= self.count_to {
      self.counter -= self.count_to;
      self.fired += 1;
    }
  }

  /// Enter callback that will be executed the correct number of times for this game tick
  pub fn fire<F>(&self, mut callback: F) where F: FnMut() -> () {
    for _ in 0..self.fired {
      callback();
    }
  }

  pub fn fire_count(&self) -> u32 {
    self.fired
  }
}

fn tick_system(mut query: Query<&mut Ticked>, properties: Res<GameProperties>) {
  query.par_for_each_mut(32, |mut ticked| {
    ticked.inc(&properties);
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

#[cfg(test)]
mod tests {
use bevy::prelude::*;

use crate::{game::stages::StagePlugin, properties::GameProperties};

use super::{TickPlugin, Ticked};

  #[test]
  fn test_ticking() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .init_resource::<GameProperties>();

    // Build ticking entity
    let ent = app.world.spawn().insert(Ticked::new(2)).id();

    // Tick through the world, observing the entity firing on every other tick.
    for i in 1..100 {
      app.update();
      if i % 2 == 0 {
        let ticked: &Ticked = app.world.entity(ent).get().unwrap();
        assert_eq!(ticked.fire_count(), 1, "Ticked did not fire on tick {}", i);
      }
    }
  }

  #[test]
  fn test_increased_tick_speed() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .insert_resource(GameProperties {
        tick_speed: 2,
        ..Default::default()
      });

    // Build ticking entity
    let ent = app.world.spawn().insert(Ticked::new(2)).id();

    // Tick through the world, observing the entity firing on every tick.
    for i in 1..100 {
      app.update();
      let ticked: &Ticked = app.world.entity(ent).get().unwrap();
      assert_eq!(ticked.fire_count(), 1, "Ticked did not fire on tick {}", i);
    }
  }

  #[test]
  fn test_multiple_fires_every_game_tick() {
    // Build App
    let mut app = App::new();
    app
      .add_plugins(MinimalPlugins)
      .add_plugin(StagePlugin)
      .add_plugin(TickPlugin)
      .insert_resource(GameProperties {
        tick_speed: 25,
        ..Default::default()
      });

    // Build ticking entity
    let ent = app.world.spawn().insert(Ticked::every_tick()).id();

    // Tick through the world, observing the entity firing on every tick 25 times.
    for i in 1..100 {
      app.update();
      let ticked: &Ticked = app.world.entity(ent).get().unwrap();
      assert_eq!(ticked.fire_count(), 25, "Ticked did not fire on tick {}", i);
    }
  }
}
