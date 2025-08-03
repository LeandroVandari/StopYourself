use crate::{
    GameState,
    modes::GameMode,
    player::{Player, PlayerDeath, record_position::RecordedPositions},
};
use avian2d::prelude::*;
use bevy::{
    diagnostic::FrameCount, ecs::entity_disabling::Disabled,
    input::common_conditions::input_just_pressed, prelude::*, sprite::AlphaMode2d,
    window::PrimaryWindow,
};

#[derive(Debug, Component)]
pub enum ObstacleType {
    Spike,
    Laser,
}

/// Marker component for obstacles
#[derive(Debug, Component)]
pub struct ObstacleMarker;

/// Marker component for the shadow of a laser
#[derive(Debug, Component)]
pub struct FakeLaser;

/// mark the last inserted obstacle to allow the player to move it
#[derive(Debug, Component)]
pub struct LastInsertedObstacle;

/// Ghost obstacle, for an obstacle that's not placed yet.
#[derive(Debug, Component)]
pub struct GhostObstacle;

#[derive(Debug, Component)]
pub struct Flicker {
    /// How many frames between each appearance start, in frames
    period: u32,
    /// How long until this appears for the first time, in frames
    delay: u32,
    /// How long this appears for, in frames
    duration: u32,
}

#[derive(Debug, Event)]
pub struct SpawnGhostObstacleEvent {
    obs_type: ObstacleType,
}

#[derive(Debug, Event)]
pub struct EmitLaserPositionEvent {
    position: Vec2,
}

impl SpawnGhostObstacleEvent {
    // TODO: make it actually random
    pub fn random() -> Self {
        let random = rand::random_range(0.0..1.0);
        Self {
            obs_type: if random < 0.4 {
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
            .add_event::<EmitLaserPositionEvent>()
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
            )
            .add_systems(
                FixedUpdate,
                Self::force_disable.run_if(
                    (in_state(GameMode::Defend).or(on_event::<PlayerDeath>))
                        .and(in_state(GameState::Game)),
                ),
            )
            .add_systems(FixedPreUpdate, Self::emit_real_laser_pos)
            .add_systems(
                FixedUpdate,
                Self::follow_real_laser_pos.run_if(on_event::<EmitLaserPositionEvent>),
            );
    }
}

impl ObstaclePlugin {
    fn force_disable(
        mut commands: Commands,
        query: Query<(Entity, &mut Visibility), With<Flicker>>,
    ) {
        for (entity, mut visibility) in query {
            commands.entity(entity).insert(ColliderDisabled);
            *visibility = Visibility::Hidden;
        }
    }
    fn flicker_on_frames(
        mut commands: Commands,
        frame_counter: Res<FrameCount>,
        recorded_positions: Res<RecordedPositions>,
        query: Query<(
            Entity,
            &mut Transform,
            &mut Flicker,
            &mut Visibility,
            Has<ColliderDisabled>,
        )>,
        asset_server: Res<AssetServer>,
    ) {
        let start_frame = recorded_positions.frame_start;
        for (entity, transform, mut flicker, mut visibility, is_disabled) in query {
            let frame_for_flicker = frame_counter.0 - start_frame;
            if frame_counter.0 < start_frame || frame_for_flicker < flicker.delay {
                commands.entity(entity).insert(ColliderDisabled);
                *visibility = Visibility::Hidden;
                return;
            }

            if ((frame_for_flicker + flicker.delay) % flicker.period) < flicker.duration {
                if !is_disabled {
                    continue;
                }
                commands.spawn(AudioPlayer::new(asset_server.load("sounds/laser.wav")));
                commands.entity(entity).remove::<ColliderDisabled>();
                *visibility = Visibility::Visible;
            } else {
                if !is_disabled {
                    commands.entity(entity).insert(ColliderDisabled);
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    fn emit_real_laser_pos(
        query: Query<(&Transform, &ObstacleType)>,
        mut laser_pos_event: EventWriter<EmitLaserPositionEvent>,
    ) {
        for (transform, obstacle_type) in query {
            match obstacle_type {
                ObstacleType::Laser => {
                    laser_pos_event.write(EmitLaserPositionEvent {
                        position: transform.translation.truncate(),
                    });
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn follow_real_laser_pos(
        mut laser_pos_event: EventReader<EmitLaserPositionEvent>,
        query: Query<&mut Transform, With<FakeLaser>>,
    ) {
        for mut transform in query {
            transform.translation = laser_pos_event.read().next().map_or(
                transform.translation,
                |EmitLaserPositionEvent { position }| position.extend(0.),
            );
        }
    }

    fn spawn_obstacle_ghost(
        mut commands: Commands,
        mut obstacle_event: EventReader<SpawnGhostObstacleEvent>,
        window: Single<&Window, With<PrimaryWindow>>,
        camera: Single<(&Camera, &GlobalTransform, &Transform), With<Camera2d>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        positions: ResMut<RecordedPositions>,
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
                             ghost_query: Query<&GhostObstacle>,
                             previous_last_obstacle: Option<
                                Single<Entity, With<LastInsertedObstacle>>,
                            >,
                             game_mode: Res<State<GameMode>>| {
                                let spike = trigger.target();
                                // If we're still placing the spike
                                if ghost_query.contains(spike) {
                                    return;
                                }

                                match game_mode.get() {
                                    GameMode::Replay => {
                                        // If we're hitting someone that wasn't the previous last obstacle,
                                        // we should ignore that collision
                                        if !(previous_last_obstacle.map_or(true, |obs| {
                                            obs.into_inner() == trigger.target()
                                        })) {
                                            return;
                                        }
                                    }

                                    _ => {
                                        // do nothing
                                    }
                                }

                                if player_query.contains(trigger.collider) {
                                    death_writer.write(PlayerDeath);
                                }
                            },
                        );
                }
                ObstacleType::Laser => {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle {
                            half_size: vec2(40., 1000.),
                        })),
                        MeshMaterial2d(materials.add(ColorMaterial {
                            color: Color::srgb(1.0, 0.2, 0.3).with_alpha(0.2),
                            alpha_mode: AlphaMode2d::Blend,
                            ..default()
                        })),
                        Transform::from_translation(cursor_pos.extend(0.)),
                        FakeLaser,
                    ));
                    commands
                        .spawn((
                            common_components,
                            ObstacleType::Laser,
                            CollisionEventsEnabled,
                            Sensor,
                            Collider::rectangle(60.0, 1000.0),
                            Mesh2d(meshes.add(Rectangle {
                                half_size: vec2(40., 10_000.),
                            })),
                            Flicker {
                                period: 120,
                                delay: 120,
                                duration: 20,
                            },
                        ))
                        .observe(
                            |trigger: Trigger<OnCollisionStart>,
                             player_query: Query<(), With<Player>>,
                             mut death_writer: EventWriter<PlayerDeath>,
                             ghost_query: Query<&GhostObstacle>,
                             previous_last_obstacle: Option<
                                Single<Entity, With<LastInsertedObstacle>>,
                            >,
                             game_mode: Res<State<GameMode>>| {
                                let laser = trigger.target();
                                // If we're still placing the laser
                                if ghost_query.contains(laser) {
                                    return;
                                }

                                match game_mode.get() {
                                    GameMode::Replay => {
                                        // If we're hitting someone that wasn't the previous last obstacle,
                                        // we should ignore that collision
                                        if !(previous_last_obstacle.map_or(true, |obs| {
                                            obs.into_inner() == trigger.target()
                                        })) {
                                            return;
                                        }
                                    }

                                    _ => {
                                        // do nothing
                                    }
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
        asset_server: Res<AssetServer>,
        ghost_obs: Single<Entity, With<GhostObstacle>>,
        previous_last_obstacle: Option<Single<Entity, With<LastInsertedObstacle>>>,

        mut flicker_query: Query<(&mut Flicker, &Position, &Rotation, &Collider)>,

        positions: Res<RecordedPositions>,
    ) {
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/click.wav")));
        info!("Placing the ghost obstacle");
        if let Some(obs) = previous_last_obstacle {
            commands
                .entity(obs.into_inner())
                .remove::<LastInsertedObstacle>();
        }
        let entity = ghost_obs.into_inner();
        let mut obs_entity = commands.entity(entity);

        if let Ok((mut flicker, pos, rot, col)) = flicker_query.get_mut(entity) {
            let position_where_player_is_in_laser = positions
                .positions
                .iter()
                .filter(|(_, p, _)| col.contains_point(*pos, *rot, p.truncate()))
                .collect::<Vec<_>>();
            let period = flicker.period;
            let delay = position_where_player_is_in_laser
                .get(rand::random_range(
                    0..position_where_player_is_in_laser.len(),
                ))
                .map_or(120, |(frame, _p, _)| {
                    info!("Will strike player in frame {frame}, when they're in position {_p}");
                    if *frame > period {
                        frame.next_multiple_of(period) - frame
                    } else {
                        *frame
                    }
                });
            flicker.delay = delay;
        }
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
