use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, LevelDimensions, modes::GoalReached, player::Player};

pub struct EnvironmentPlugin;

#[derive(Debug, Event)]
pub struct ResetEnvironment;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (Self::spawn_ground, Self::spawn_goal),
        )
        .add_event::<ResetEnvironment>();
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

    fn spawn_goal(mut commands: Commands, level_dimensions: Res<LevelDimensions>) {
        let size = vec2(level_dimensions.tile_size, level_dimensions.tile_size * 2.);

        commands
            .spawn((
                Goal,
                Sprite {
                    color: Color::srgb(1.0, 1.0, 0.),
                    custom_size: Some(size),
                    ..Default::default()
                },
                CollisionEventsEnabled,
                Sensor,
                Collider::rectangle(size.x, size.y),
                Transform::from_translation(
                    level_dimensions
                        .grid_pos_to_pixels((level_dimensions.level_length - 10, 3), size)
                        .extend(0.),
                ),
            ))
            .observe(
                |trigger: Trigger<OnCollisionStart>,
                 player_query: Query<(), With<Player>>,
                 mut flag_event_writer: EventWriter<GoalReached>| {
                    // if it's the player that collided, send the event
                    if player_query.contains(trigger.collider) {
                        flag_event_writer.write(GoalReached);
                    }
                },
            );
    }
}

/// Marker component for the goal
#[derive(Component)]
pub struct Goal;
