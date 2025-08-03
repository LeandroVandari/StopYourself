use avian2d::prelude::*;
use bevy::{
    diagnostic::FrameCount, input::common_conditions::input_just_pressed, prelude::*,
    window::PrimaryWindow,
};
use rand::prelude::*;

use crate::{
    GameState,
    modes::GameMode,
    player::{Player, PlayerDeath, record_position::RecordedPositions},
};

#[derive(Debug, Component)]
pub enum ObstacleType {
    Spike,
    Laser,
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

#[derive(Debug, Component)]
pub struct Flicker {
    // How many frames between each appearance start, in frames
    period: u32,
    // How long until this appears for the first time, in frames
    delay: u32,
    // How long this appears for, in frames
    duration: u32,
    // the original position of the object
    // (we flicker objects by physically placing them far away)
    original_position: Vec3,
}

#[derive(Debug, Event)]
pub struct SpawnGhostObstacleEvent {
    obs_type: ObstacleType,
}

impl SpawnGhostObstacleEvent {
    // TODO: make it actually random
    pub fn random() -> Self {
        Self {
            obs_type: if rand::random::<bool>() {
                ObstacleType::Laser
            } else {
                ObstacleType::Spike
            },
        }
    }
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnGhostObstacleEvent>()
            .add_systems(
                Update,
                (
                    Self::spawn_obstacle_ghost.run_if(on_event::<SpawnGhostObstacleEvent>),
                    Self::ghost_obstacle_follow_mouse,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                FixedPreUpdate,
                Self::place_ghost_obs
                    .run_if(
                        input_just_pressed(MouseButton::Left)
                            .and(in_state(GameMode::Defend).and(in_state(GameState::Game))),
                    )
                    .before(crate::update_state),
            )
            .add_systems(
                FixedUpdate,
                Self::flicker_on_frames.run_if(
                    (in_state(GameMode::Replay).or(in_state(GameMode::Survive)))
                        .and(in_state(GameState::Game)),
                ),
            );
    }
}

impl ObstaclePlugin {
    fn flicker_on_frames(
        frame_counter: Res<FrameCount>,
        recorded_positions: Res<RecordedPositions>,
        query: Query<(&mut Transform, &mut Flicker)>,
    ) {
        let start_frame = recorded_positions.frame_start;
        if frame_counter.0 < start_frame {
            return;
        }
        for (mut transform, mut flicker) in query {
            let frame_for_flicker =
                (frame_counter.0 - start_frame + flicker.delay) % flicker.period;
            if frame_for_flicker < flicker.duration {
                if transform.translation != Vec3::X * 10_000. {
                    flicker.original_position = transform.translation;
                }
                transform.translation = flicker.original_position;
            } else {
                if transform.translation != Vec3::X * 10_000. {
                    flicker.original_position = transform.translation;
                    transform.translation = Vec3::X * 10_000.;
                }
            }
        }
    }

    fn spawn_obstacle_ghost(
        mut commands: Commands,
        mut obstacle_event: EventReader<SpawnGhostObstacleEvent>,
        window: Single<&Window, With<PrimaryWindow>>,
        camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera2d>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let (camera, camera_global_transform, camera_transform) = camera.into_inner();

        let cursor_pos = get_cursor_world_pos(window.into_inner(), camera, camera_global_transform)
            .unwrap_or(camera_transform.translation.xy());
        for event in obstacle_event.read() {
            // Components that all obstacles have in common
            let common_components = (
                GhostObstacle,
                ObstacleMarker,
                Transform::from_translation(cursor_pos.extend(0.)),
                MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.2, 0.3))),
                ),
            );

            match event.obs_type {
                ObstacleType::Spike => {
                    commands
                        .spawn((
                            common_components,
                            CollisionEventsEnabled,
                            Sensor,
                            Collider::triangle(vec2(-14.0, 0.0), vec2(14.0, 0.0), vec2(0.0, 28.0)),
                            Mesh2d(meshes.add(Triangle2d::new(
                                vec2(-20.0, 0.0),
                                vec2(20.0, 0.0),
                                vec2(0.0, 40.0),
                            ))),
                            ObstacleType::Spike,
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
                ObstacleType::Laser => {
                    commands
                        .spawn((
                            common_components,
                            ObstacleType::Laser,
                            CollisionEventsEnabled,
                            Sensor,
                            Collider::rectangle(60.0, 1000.0),
                            Mesh2d(meshes.add(Rectangle {
                                half_size: vec2(40., 1000.),
                            })),
                            Flicker {
                                period: 100,
                                delay: rand::random_range(0..50),
                                duration: 40,
                                original_position: Vec3::ZERO,
                            },
                        ))
                        .observe(
                            |trigger: Trigger<OnCollisionStart>,
                             player_query: Query<(), With<Player>>,
                             mut death_writer: EventWriter<PlayerDeath>,
                             ghost_query: Query<&GhostObstacle>| {
                                let laser = trigger.target();
                                // If we're still placing the laser
                                if ghost_query.contains(laser) {
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
        ghost_obs: Single<(&mut Transform, &ObstacleType), With<GhostObstacle>>,
        camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    ) {
        let (mut obs_transform, obs_type) = ghost_obs.into_inner();
        let (camera, camera_transform) = camera.into_inner();

        let inner_window = window.into_inner();

        if obs_transform.translation == Vec3::X * 10_000. {
            obs_transform.translation =
                get_cursor_world_pos(inner_window, camera, camera_transform)
                    .map_or(obs_transform.translation, |pos| {
                        pos.extend(obs_transform.translation.z)
                    });
        }

        let mut target_translation = get_cursor_world_pos(inner_window, camera, camera_transform)
            .map_or(obs_transform.translation, |pos| {
                pos.extend(obs_transform.translation.z)
            });
        if matches!(obs_type, ObstacleType::Laser) {
            target_translation = target_translation.with_y(0.)
        }

        let diff = target_translation - obs_transform.translation;
        let diff_length = diff.length();
        if diff_length <= 0.1 {
            return;
        }

        let dir = diff.clone().normalize();

        obs_transform.translation += dir * f32::min(50., diff_length);
    }

    fn place_ghost_obs(
        mut commands: Commands,
        ghost_obs: Single<Entity, With<GhostObstacle>>,
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
    }
}

pub fn get_cursor_world_pos(
    window: &Window,

    camera: &Camera,

    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    // Convert screen to world pos: https://bevy-cheatbook.github.io/cookbook/cursor2world.html

    window
        .cursor_position()
        .map(|pos| camera.viewport_to_world_2d(camera_transform, pos).unwrap())
}
