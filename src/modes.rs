use bevy::{diagnostic::FrameCount, input::common_conditions::input_pressed, prelude::*};

use crate::{
    environment::ResetEnvironment,
    obstacles::{GhostObstacle, LastInsertedObstacle, SpawnGhostObstacleEvent},
    player::record_position::{RecordPositionPlugin, RecordedPositions},
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
            FixedPreUpdate,
            (
                (
                    Self::handle_flag_reached,
                    RecordPositionPlugin::record_position.run_if(in_state(GameMode::Survive)),
                )
                    .run_if(on_event::<GoalReached>),
                Self::handle_replay
                    .run_if(input_pressed(KeyCode::Space).and(in_state(GameMode::Defend))),
            )
                .before(crate::update_state),
        )
        .add_systems(OnEnter(GameMode::Replay), Self::replay)
        .init_state::<GameMode>()
        .add_event::<GoalReached>();
    }
}

impl ModesManagement {
    fn handle_flag_reached(
        mut commands: Commands,
        mut state: ResMut<NextState<GameMode>>,
        mut reset_environment: EventWriter<ResetEnvironment>,
        mut spawn_obstacle_writer: EventWriter<SpawnGhostObstacleEvent>,
        mode: Res<State<GameMode>>,

        last_placed_obstacle: Option<Single<Entity, With<LastInsertedObstacle>>>,
    ) {
        match mode.get() {
            GameMode::Survive => {
                info!("flag reached in survive mode");
                reset_environment.write(ResetEnvironment);
                spawn_obstacle_writer.write(SpawnGhostObstacleEvent::random());
                state.set(GameMode::Defend);
            }
            GameMode::Replay => {
                info!("flag reached in replay mode");
                reset_environment.write(ResetEnvironment);
                commands
                    .entity(last_placed_obstacle.unwrap().into_inner())
                    .insert(GhostObstacle)
                    .remove::<LastInsertedObstacle>();
                state.set(GameMode::Defend)
            }
            GameMode::Defend => {
                error!("Reached the flag in defend mode");
            }
        }
    }

    fn handle_replay(
        ghost_obs_query: Option<Single<&GhostObstacle>>,
        mut state: ResMut<NextState<GameMode>>,
    ) {
        if ghost_obs_query.is_some() {
            return;
        }

        state.set(GameMode::Replay);
    }

    fn replay(mut recorded_positions: ResMut<RecordedPositions>, frame: Res<FrameCount>) {
        info!("resetting frame_start");
        recorded_positions.frame_start = frame.0;
        recorded_positions.last_played_frame = 0;
    }
}
