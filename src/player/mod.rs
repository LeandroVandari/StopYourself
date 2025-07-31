use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{LevelDimensions, player::movement::CharacterControllerBundle};

mod movement;
pub mod record_movement;
/// Player spawning and movement handling.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::PlayerMovementPlugin,
            record_movement::RecordMovementPlugin,
        ))
        .add_event::<ResetPlayer>()
        .add_systems(Startup, Self::spawn_player)
        .add_systems(
            Update,
            Self::move_to_start_pos.run_if(on_event::<ResetPlayer>),
        );
    }
}
/// Marker for the player character.
#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Event)]
pub struct ResetPlayer;

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
            Transform::from_translation(
                level_dimensions
                    .grid_pos_to_pixels((1, 3), vec2(40., 80.))
                    .extend(1.),
            ),
        ));
    }

    fn move_to_start_pos(
        mut player: Single<&mut Transform, With<Player>>,
        level_dimensions: Res<LevelDimensions>,
    ) {
        player.translation = level_dimensions
            .grid_pos_to_pixels((1, 3), vec2(40., 80.))
            .extend(1.);
    }
}
