use avian2d::prelude::*;
use bevy::prelude::*;

/// Player spawning and movement handling.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_player);
    }
}
/// Marker for the player character.
#[derive(Debug, Component)]
pub struct Player;

impl PlayerPlugin {
    fn spawn_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Player,
            // Appearance
            Mesh2d(meshes.add(Rectangle {
                half_size: vec2(20., 40.),
            })),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            // Physics
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(40., 80.),

            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}
