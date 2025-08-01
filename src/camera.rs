use bevy::{prelude::*, window::PrimaryWindow};

use crate::{LevelDimensions, modes::GameMode, player::Player};

pub struct CameraPlugin;
/// How many tiles ahead of the player the camera should be.
const CAMERA_AHEAD: usize = 15;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::spawn_camera).add_systems(
            Update,
            (
                Self::follow_player
                    .run_if(in_state(GameMode::Survive).or(in_state(GameMode::Replay))),
                Self::keyboard_input.run_if(in_state(GameMode::Defend)),
            ),
        );
    }
}

impl CameraPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }

    /// Follow the player smoothly
    fn follow_player(
        mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
        player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
        level_dimensions: Res<LevelDimensions>,
        main_window: Single<&Window, With<PrimaryWindow>>,
    ) {
        let target_translation = vec3(
            (player.translation.x + level_dimensions.tile_size * CAMERA_AHEAD as f32)
                .max(level_dimensions.start.x + main_window.size().x / 2.) // Clamp to the level size
                .min(
                    level_dimensions.start.x
                        + level_dimensions.tile_size * level_dimensions.level_length as f32
                        - main_window.size().x / 2.,
                ),
            0.,
            0.,
        );

        let diff = target_translation - camera.translation;
        let diff_length = diff.length();
        if diff_length <= 0.1 {
            return;
        }

        let dir = diff.clone().normalize();

        camera.translation += dir * f32::min(10., diff_length);
    }

    /// Move the camera with keyboard on defend mode
    fn keyboard_input(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut camera: Single<&mut Transform, With<Camera2d>>,
        level_dimensions: Res<LevelDimensions>,
        main_window: Single<&Window, With<PrimaryWindow>>,
    ) {
        if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            camera.translation.x -= 10.;
        }
        if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            camera.translation.x += 10.;
        }
        if keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            camera.translation.y += 10.;
        }
        if keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            camera.translation.y -= 10.;
        }

        camera.translation.x = camera
            .translation
            .x
            .max(level_dimensions.start.x + main_window.size().x / 2.) // Clamp to the level size
            .min(
                level_dimensions.start.x
                    + level_dimensions.tile_size * level_dimensions.level_length as f32
                    - main_window.size().x / 2.,
            );
        camera.translation.y = camera.translation.y.max(0.0).min(100.);
    }
}
