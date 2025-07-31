use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::spawn_camera);
    }
}

impl CameraPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }
}