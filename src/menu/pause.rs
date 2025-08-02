use bevy::{diagnostic::FrameCount, input::common_conditions::input_just_pressed, prelude::*};

use crate::{GameState, player::record_position::RecordedPositions};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::handle_pause.run_if(input_just_pressed(KeyCode::Escape)),
        );
    }
}

impl PausePlugin {
    fn handle_pause(
        mut set_state: ResMut<NextState<GameState>>,
        get_state: Res<State<GameState>>,
        mut time: ResMut<Time<Virtual>>,
        mut positions: ResMut<RecordedPositions>,
        frame: Res<FrameCount>,
    ) {
        match get_state.get() {
            GameState::Game => {
                set_state.set(GameState::Paused {
                    frame_paused: frame.0,
                });
                time.pause();
            }
            GameState::Paused { frame_paused } => {
                let frames_since_pause = frame.0 - frame_paused;
                positions.frame_start += frames_since_pause;
                set_state.set(GameState::Game);
                time.unpause();
            }
            _ => (),
        }
    }
}
