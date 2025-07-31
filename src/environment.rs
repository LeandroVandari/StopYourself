use avian2d::prelude::*;
use bevy::prelude::*;

use crate::LevelDimensions;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_ground);
    }
}

impl EnvironmentPlugin {
    fn spawn_ground(mut commands: Commands, level_dimensions: Res<LevelDimensions>) {
        let ground_width = level_dimensions.tile_size * level_dimensions.level_length as f32;

        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(vec2(ground_width, level_dimensions.tile_size)),
                ..Default::default()
            },
            RigidBody::Static,
            Collider::rectangle(ground_width, level_dimensions.tile_size),
            Transform::from_translation(
                level_dimensions
                    .grid_pos_to_pixels((0, 2), vec2(ground_width, level_dimensions.tile_size))
                    .extend(0.),
            ),
        ));
    }
}
