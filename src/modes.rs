use avian2d::prelude::*;
use bevy::{diagnostic::FrameCount, prelude::*};

use crate::{
    environment::ResetEnvironment,
    obstacles::{GhostObstacle, LastInsertedObstacle, SpawnGhostObstacleEvent},
    player::record_movement::RecordedMovements,
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
            FixedUpdate,
            (Self::handle_goal_reached.run_if(on_event::<GoalReached>),),
        )
        .add_systems(OnEnter(GameMode::Replay), Self::replay)
        .init_state::<GameMode>()
        .add_event::<GoalReached>();
    }
}

impl ModesManagement {
    pub fn handle_goal_reached(
        mut commands: Commands,
        curr_state: Res<State<GameMode>>,
        mut state: ResMut<NextState<GameMode>>,
        mut reset_environment: EventWriter<ResetEnvironment>,
        mut spawn_obstacle_writer: EventWriter<SpawnGhostObstacleEvent>,

        last_placed_object: Option<Single<Entity, With<LastInsertedObstacle>>>,
    ) {
        reset_environment.write(ResetEnvironment);
        state.set(GameMode::Defend);

        match curr_state.get() {
            GameMode::Survive => {
                spawn_obstacle_writer.write(SpawnGhostObstacleEvent::random());
            }
            GameMode::Replay => {
                commands
                    .entity(
                        last_placed_object
                            .expect("If we're in replay mode an obstacle was already placed.")
                            .into_inner(),
                    )
                    .insert(GhostObstacle)
                    .remove::<CollisionEventsEnabled>();
            }
            GameMode::Defend => {
                warn!("Shouldn't reach goal in the defend game mode...")
            }
        }
    }

    fn replay(mut recorded_movements: ResMut<RecordedMovements>, frame: Res<FrameCount>) {
        recorded_movements.frame_start = frame.0
    }
}
