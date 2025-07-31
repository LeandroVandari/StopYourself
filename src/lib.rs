use bevy::{prelude::*, window::PrimaryWindow};

pub mod camera;
pub mod environment;
pub mod player;

pub struct SetupPlugin;

#[derive(Debug, Resource)]
struct LevelDimensions {
    start: Vec2,
    tile_size: f32,
    // length in tiles
    level_length: u32
 }

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::level_dimensions);
    }
}

impl SetupPlugin {
    fn level_dimensions(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
        commands.insert_resource(LevelDimensions {
            start: window.size().map(|dimension| -dimension / 2.),
            tile_size: 20.,
            level_length: 200
    });
    }
}
