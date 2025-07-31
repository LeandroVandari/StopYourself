use bevy::prelude::*;

use crate::player::ResetPlayer;

/// The two modes for the game
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode {
    /// Get to the goal bypassing the traps
    #[default]
    Survive,
    /// Place defenses to stop your replay from getting to the goal
    Defend,
}

#[derive(Debug, Event)]
pub struct GoalReached;

pub struct ModesManagement;

impl Plugin for ModesManagement {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::change_to_defend.run_if(on_event::<GoalReached>),
        )
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
        state.set(GameMode::Defend);
    }
}
