use bevy::{diagnostic::FrameCount, prelude::*};

use crate::player::{ResetPlayer, record_movement::RecordedMovements};

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
            (Self::change_to_defend.run_if(on_event::<GoalReached>),),
        )
        .add_systems(OnEnter(GameMode::Replay), Self::replay)
        .init_state::<GameMode>()
        .add_event::<GoalReached>();
    }
}

impl ModesManagement {
    fn change_to_defend(
        mut state: ResMut<NextState<GameMode>>,
        mut reset_player: EventWriter<ResetPlayer>,
    ) {
        reset_player.write(ResetPlayer);
        // TODO: Change this to defend when implementing it
        state.set(GameMode::Replay);
    }

    fn replay(mut recorded_movements: ResMut<RecordedMovements>, frame: Res<FrameCount>) {
        recorded_movements.frame_start = frame.0
    }
}
