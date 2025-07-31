use bevy::{prelude::*, window::PrimaryWindow};

pub mod environment;
pub mod player;
pub mod camera;

pub struct SetupPlugin;

#[derive(Debug, Resource)]
struct LevelStartPos(Vec2);

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::level_start);
    }
}

impl SetupPlugin {
    fn level_start(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>, ) {
        commands.insert_resource(LevelStartPos(window.size().map(|dimension| -dimension/2.)));
    }
}
