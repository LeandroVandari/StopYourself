use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{LevelDimensions, player::movement::CharacterControllerBundle};

mod movement;
/// Player spawning and movement handling.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(movement::PlayerMovementPlugin)
            .add_systems(Startup, Self::spawn_player);
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
        level_dimensions: Res<LevelDimensions>,
    ) {
        commands.spawn((
            // Appearance
            Mesh2d(meshes.add(Rectangle {
                half_size: vec2(20., 40.),
            })),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            // Movement
            CharacterControllerBundle::new(Collider::rectangle(40., 80.))
                .with_movement(1250., 0.92, 400.),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(1.5),
            Transform::from_xyz(level_dimensions.start.x + 40., level_dimensions.start.y + 80., 0.0),
        ));
    }
}
