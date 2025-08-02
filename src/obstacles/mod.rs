use avian2d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

use crate::player::{Player, PlayerDeath};

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
        app.add_event::<SpawnGhostObstacleEvent>()
            .add_systems(
                Update,
                (
                    Self::spawn_obstacle_ghost.run_if(on_event::<SpawnGhostObstacleEvent>),
                    Self::ghost_obstacle_follow_mouse,
                ),
            )
            .add_systems(
                FixedPreUpdate,
                Self::place_ghost_obs
                    .run_if(input_just_pressed(MouseButton::Left))
                    .before(crate::update_state),
            );
    }
}

impl ObstaclePlugin {
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
        let (camera, camera_transform) = camera.into_inner();

        let target_translation =
            get_cursor_world_pos(window.into_inner(), camera, camera_transform)
                .map_or(ghost_obs.translation, |pos| {
                    pos.extend(ghost_obs.translation.z)
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
