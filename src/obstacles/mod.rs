use avian2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

use crate::{
    modes::GameMode,
    player::{Player, PlayerDeath},
};

#[derive(Debug)]
pub enum ObstacleType {
    Spike,
}

/// Marker component for obstacles
#[derive(Debug, Component)]
pub struct ObstacleMarker;

/// mark the last inserted obstacle to allow the player to move it
#[derive(Debug, Component)]
pub struct LastInsertedObstacle;

/// Ghost obstacle, for an obstacle that's not placed yet.
#[derive(Debug, Component)]
pub struct GhostObstacle;

#[derive(Debug, Event)]
pub struct SpawnGhostObstacleEvent {
    obs_type: ObstacleType,
}

impl SpawnGhostObstacleEvent {
    // TODO: make it actually random
    pub fn random() -> Self {
        Self {
            obs_type: ObstacleType::Spike,
        }
    }
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnGhostObstacleEvent>().add_systems(
            Update,
            (
                Self::spawn_obstacle_ghost.run_if(on_event::<SpawnGhostObstacleEvent>),
                Self::ghost_obstacle_follow_mouse,
                Self::place_ghost_obs.run_if(input_just_pressed(MouseButton::Left)),
            ),
        );
    }
}

impl ObstaclePlugin {
    fn spawn_obstacle_ghost(
        mut commands: Commands,
        mut obstacle_event: EventReader<SpawnGhostObstacleEvent>,
        window: Single<&Window, With<PrimaryWindow>>,
        camera: Single<&Transform, With<Camera2d>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let cursor_pos = window.cursor_position().unwrap_or(camera.translation.xy());
        for event in obstacle_event.read() {
            // Components that all obstacles have in common
            let common_components = (
                GhostObstacle,
                ObstacleMarker,
                Transform::from_translation(cursor_pos.extend(0.)),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            );

            match event.obs_type {
                ObstacleType::Spike => {
                    commands
                        .spawn((
                            common_components,
                            CollisionEventsEnabled,
                            Sensor,
                            Collider::triangle(vec2(-20.0, 0.0), vec2(20.0, 0.0), vec2(0.0, 40.0)),
                            Mesh2d(meshes.add(Triangle2d::new(
                                vec2(-20.0, 0.0),
                                vec2(20.0, 0.0),
                                vec2(0.0, 40.0),
                            ))),
                        ))
                        .observe(
                            |trigger: Trigger<OnCollisionStart>,
                             player_query: Query<(), With<Player>>,
                             mut death_writer: EventWriter<PlayerDeath>,
                             ghost_query: Query<&GhostObstacle>| {
                                let spike = trigger.target();
                                // If we're still placing the spike
                                if ghost_query.contains(spike) {
                                    return;
                                }

                                if player_query.contains(trigger.collider) {
                                    death_writer.write(PlayerDeath);
                                }
                            },
                        );
                }
            }
        }
    }
    /// Make the ghost obstacle follow the mouse.
    /// Similar to the camera tracking to the player, it lags a bit behind.
    fn ghost_obstacle_follow_mouse(
        window: Single<&Window, With<PrimaryWindow>>,
        mut ghost_obs: Single<&mut Transform, With<GhostObstacle>>,
        camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    ) {
        let target_translation = window
            .cursor_position()
            .map_or(ghost_obs.translation, |pos| {
                let (camera, camera_transform) = camera.into_inner();
                // Convert screen to world pos: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
                camera
                    .viewport_to_world_2d(camera_transform, pos)
                    .unwrap()
                    .extend(ghost_obs.translation.z)
            });

        let diff = target_translation - ghost_obs.translation;
        let diff_length = diff.length();
        if diff_length <= 0.1 {
            return;
        }

        let dir = diff.clone().normalize();

        ghost_obs.translation += dir * f32::min(50., diff_length);
    }

    fn place_ghost_obs(
        mut commands: Commands,
        ghost_obs: Single<Entity, With<GhostObstacle>>,
        mut state: ResMut<NextState<GameMode>>,
        previous_last_obstacle: Option<Single<Entity, With<LastInsertedObstacle>>>,
    ) {
        info!("Placing the ghost obstacle");
        if let Some(obs) = previous_last_obstacle {
            commands
                .entity(obs.into_inner())
                .remove::<LastInsertedObstacle>();
        }

        let mut obs_entity = commands.entity(ghost_obs.into_inner());
        obs_entity.remove::<GhostObstacle>();
        obs_entity.insert(LastInsertedObstacle);
        // Doing this straight after placing the object for now, but we probably want to allow them to
        // change the placement and start a replay by pressing space or something
        state.set(GameMode::Replay)
    }
}
