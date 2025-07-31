use avian2d::prelude::*;
use bevy::prelude::*;

pub struct EnvironmentPlugin;

const TILE_SIZE: f32 = 20.;

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
    ) {
        for i in 0..20 {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle {
                    half_size: vec2(TILE_SIZE / 2., TILE_SIZE / 2.),
                })),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
                RigidBody::Static,
                Collider::rectangle(TILE_SIZE, TILE_SIZE),
                Transform::from_xyz(-100. + i as f32 * TILE_SIZE, -200., 0.0),
            ));
        }
    }
}
