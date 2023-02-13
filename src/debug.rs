use bevy::prelude::*;
use itertools::Itertools;

use crate::db::models::World;
use crate::game::world::{ChunkRequests, LoadedChunkTable};

pub struct DebugCameraPlugin;

impl DebugCameraPlugin {
  pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
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
    query: Query<&Transform, With<Camera>>,
    mut requests: EventWriter<ChunkRequests>,
    chunk_table: Res<LoadedChunkTable>,
  ) {
    const CHUNK_RADIUS: i32 = 2;

    query.for_each(|trans| {
      let rounded_position =
        (trans.translation.truncate() / World::CHUNK_SIDE_LENGTH as f32 / World::TILE_PIXEL_SIZE).round();
      let pos = [rounded_position.x as i32, rounded_position.y as i32];

      if !chunk_table.0.contains_key(&pos) {
        let [x, y] = pos;
        // Load New Chunks
        ((x - CHUNK_RADIUS)..(x + CHUNK_RADIUS))
          .cartesian_product((y - CHUNK_RADIUS)..(y + CHUNK_RADIUS))
          .for_each(|(x, y)| {
            requests.send(ChunkRequests::Load(x, y));
          });

        // Unload Old Chunks
        let unloading_boundary: Vec<[i32; 2]> = ((x - CHUNK_RADIUS - 1)..(x + CHUNK_RADIUS + 1))
          .cartesian_product((y - CHUNK_RADIUS - 1)..(y + CHUNK_RADIUS + 1))
          .map(|(x, y)| [x, y])
          .collect_vec();
        chunk_table.0.iter().for_each(|(key, _)| {
          if !unloading_boundary.contains(key) {
            let [x, y] = key;
            requests.send(ChunkRequests::Unload(*x, *y));
          }
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
