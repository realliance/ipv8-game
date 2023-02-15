use bevy::prelude::*;
use itertools::Itertools;

use crate::db::models::World;
use crate::game::world::{LoadChunkCommand, LoadedChunkTable};

pub struct DebugCameraPlugin;

impl DebugCameraPlugin {
  pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
      transform: Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        scale: Vec3::ONE * 5.0,
        ..default()
      },
      ..default()
    });
  }

  pub fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
  ) {
    const CAMERA_SPEED: f32 = 64.0;
    if let Some(mut trans) = query.iter_mut().next() {
      let mut x = trans.translation.x;
      let mut y = trans.translation.y;
      let modifier = time.delta_seconds() + CAMERA_SPEED;

      if keyboard_input.pressed(KeyCode::A) {
        x -= modifier;
      }

      if keyboard_input.pressed(KeyCode::D) {
        x += modifier;
      }

      if keyboard_input.pressed(KeyCode::W) {
        y += modifier;
      }

      if keyboard_input.pressed(KeyCode::S) {
        y -= modifier;
      }

      trans.translation.x = x;
      trans.translation.y = y;
    }
  }

  pub fn load_chunks_in_view(
    mut commands: Commands,
    query: Query<&Transform, With<Camera>>,
    chunk_table: Res<LoadedChunkTable>,
  ) {
    const CHUNK_RADIUS: i64 = 2;

    query.for_each(|trans| {
      let rounded_position =
        (trans.translation.truncate() / World::CHUNK_SIDE_LENGTH as f32 / World::TILE_PIXEL_SIZE).round();
      let pos = [rounded_position.x as i64, rounded_position.y as i64];

      if chunk_table.get_if_exists(pos).is_none() {
        let [x, y] = pos;
        // Load New Chunks
        ((x - CHUNK_RADIUS)..(x + CHUNK_RADIUS))
          .cartesian_product((y - CHUNK_RADIUS)..(y + CHUNK_RADIUS))
          .for_each(|(x, y)| {
            commands.spawn(LoadChunkCommand([x, y]));
          });
      }
    });
  }
}

impl Plugin for DebugCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(Self::setup_camera)
      .add_system(Self::load_chunks_in_view)
      .add_system(Self::move_camera);
  }
}
