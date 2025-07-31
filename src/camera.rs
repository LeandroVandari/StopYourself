use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::spawn_camera)
        .add_systems(Update, Self::follow_player);
    }
}

impl CameraPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }

    /// Follow the player smoothly
    fn follow_player(mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>, player: Single<&Transform, (With<Player>, Without<Camera2d>)>) {
        let target_translation = player.translation;

        let diff = target_translation - camera.translation;
        let diff_length = diff.length();
        if diff_length <= 0.1 {
            return;
        }

        let dir = diff.clone().normalize();

        camera.translation += dir*f32::min(10., diff_length) * Vec3::X;

    }
}