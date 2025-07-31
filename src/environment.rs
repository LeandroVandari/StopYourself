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
    fn spawn_ground(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        level_dimensions: Res<LevelDimensions>,
    ) {
        let ground_width = level_dimensions.tile_size * level_dimensions.level_length as f32;

        commands.spawn((
            Mesh2d(meshes.add(Rectangle {
                half_size: vec2(ground_width / 2., level_dimensions.tile_size / 2.),
            })),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            RigidBody::Static,
            Collider::rectangle(ground_width, level_dimensions.tile_size),
            // Default anchor is center, and for some reason setting it to something else didn't work for me, so
            // i do the position calculation manually
            Transform::from_xyz(
                level_dimensions.start.x + ground_width / 2.,
                level_dimensions.start.y + level_dimensions.tile_size + 10.,
                0.0,
            ),
        ));
    }
}
