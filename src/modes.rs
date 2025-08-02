use bevy::{diagnostic::FrameCount, prelude::*};

use crate::{
    environment::ResetEnvironment,
    obstacles::{GhostObstacle, LastInsertedObstacle, SpawnGhostObstacleEvent},
    player::{
        Player, PositionEvent, record_movement::RecordedMovements,
        record_position::RecordedPositions,
    },
};

/// The two modes for the game
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode {
    /// Get to the goal bypassing the traps
    #[default]
    Survive,
    /// Place defenses to stop your replay from getting to the goal
    Defend,
    /// Watch the replay go against the defenses
    Replay,
}

#[derive(Debug, Event)]
pub struct GoalReached;

pub struct ModesManagement;

impl Plugin for ModesManagement {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::change_to_defend
                .run_if(on_event::<GoalReached>.and(in_state(GameMode::Survive))),),
        )
        .add_systems(
            FixedUpdate,
            (Self::draw_player_ghost.run_if(on_event::<GoalReached>),),
        )
        .add_systems(OnEnter(GameMode::Replay), Self::replay)
        .init_state::<GameMode>()
        .add_event::<GoalReached>();
    }
}

impl ModesManagement {
    fn draw_player_ghost(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        recorded_positions: Res<RecordedPositions>,
    ) {
        for (_, PositionEvent::Position(recorded_position)) in &recorded_positions.positions {
            commands.spawn((
                // Appearance
                Mesh2d(meshes.add(Rectangle {
                    half_size: vec2(20., 40.),
                })),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
                // Movement
                Transform::from_translation(recorded_position.extend(1.)),
            ));
        }
    }
    fn change_to_defend(
        mut state: ResMut<NextState<GameMode>>,
        mut reset_environment: EventWriter<ResetEnvironment>,
        mut spawn_obstacle_writer: EventWriter<SpawnGhostObstacleEvent>,
    ) {
        reset_environment.write(ResetEnvironment);
        spawn_obstacle_writer.write(SpawnGhostObstacleEvent::random());
        state.set(GameMode::Defend);
    }

    fn replay(mut recorded_movements: ResMut<RecordedMovements>, frame: Res<FrameCount>) {
        recorded_movements.frame_start = frame.0
    }
}
