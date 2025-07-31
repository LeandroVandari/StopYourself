use avian2d::prelude::*;
use bevy::prelude::*;

use crate::LevelStartPos;

pub struct EnvironmentPlugin;

const TILE_SIZE: f32 = 20.;
const LEVEL_WIDTH: usize = 200;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::spawn_map);
    }
}

impl EnvironmentPlugin {
    fn spawn_map(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        level_start: Res<LevelStartPos>
    ) {

        commands.spawn((
            Mesh2d(meshes.add(Rectangle {
                half_size: vec2(TILE_SIZE * LEVEL_WIDTH as f32 / 2., TILE_SIZE / 2.),
            })),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            RigidBody::Static,
            Collider::rectangle(TILE_SIZE * LEVEL_WIDTH as f32, TILE_SIZE),
            Transform::from_xyz(
                level_start.0.x,
                level_start.0.y + TILE_SIZE + 10.,
                0.0,
            ),
        ));
    }
}
