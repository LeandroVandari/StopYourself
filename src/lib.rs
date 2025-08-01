use bevy::{prelude::*, window::PrimaryWindow};

pub mod camera;
pub mod environment;
pub mod modes;
pub mod obstacles;
pub mod player;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::level_dimensions);
    }
}

#[derive(Debug, Resource)]
pub struct LevelDimensions {
    start: Vec2,
    tile_size: f32,
    // length in tiles
    level_length: u32,
}

impl SetupPlugin {
    fn level_dimensions(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
        commands.insert_resource(LevelDimensions {
            start: window.size().map(|dimension| -dimension / 2.),
            tile_size: 20.,
            level_length: 75,
        });
    }
}

impl LevelDimensions {
    /// Uses top-left anchor (because the physics engine doesn't work with anchors, we need to do this manually).
    /// If you want a center anchor, just pass Vec2::ZERO as the object_size.
    pub fn grid_pos_to_pixels(&self, pos: (u32, u32), object_size: Vec2) -> Vec2 {
        self.start
            + vec2(
                pos.0 as f32 * self.tile_size + object_size.x / 2.,
                pos.1 as f32 * self.tile_size + object_size.y / 2.,
            )
    }
}
