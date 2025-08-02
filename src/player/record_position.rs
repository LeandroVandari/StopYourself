use bevy::{
    app::{App, FixedUpdate, Plugin},
    diagnostic::FrameCount,
    ecs::resource::Resource,
    prelude::*,
};

use crate::{modes::GameMode, player::Player};

pub struct RecordPositionPlugin;

#[derive(Debug, Resource)]
pub struct RecordedPositions {
    /// Which frame the positions started being recorded in
    pub(crate) frame_start: u32,
    pub(crate) positions: Vec<(u32, Vec3)>, // (Frame of the position, position)
    pub(crate) last_played_frame: usize,
    pub(crate) locked: bool,
}

impl Plugin for RecordPositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RecordedPositions {
            frame_start: 0,
            positions: Vec::with_capacity(256),
            last_played_frame: 0,
            locked: false,
        })
        .add_systems(
            FixedUpdate,
            Self::record_position.run_if(in_state(GameMode::Survive)),
        );
    }
}

impl RecordPositionPlugin {
    pub fn record_position(
        position: Single<&Transform, With<Player>>,
        mut recorded_positions: ResMut<RecordedPositions>,
        frame_counter: Res<FrameCount>,
    ) {
        if recorded_positions.locked {
            return;
        }
        // info!("Recorded position! `{:?}`", position);
        if recorded_positions.positions.is_empty() {
            recorded_positions.frame_start = frame_counter.0;
            // recorded_positions.last_played_frame = 0;
        }
        let frame_from_start = frame_counter.0 - recorded_positions.frame_start;
        recorded_positions
            .positions
            .push((frame_from_start, position.translation))
    }

    pub fn play_recorded_position(
        mut recorded_positions: ResMut<RecordedPositions>,
        frame_counter: Res<FrameCount>,
        mut player: Single<&mut Transform, With<Player>>,
    ) {
        let start_frame = recorded_positions.last_played_frame;
        let positions = recorded_positions.positions.clone();
        for position in positions
            .iter()
            .skip_while(|pos| pos.0 < start_frame as u32)
        {
            if (frame_counter.0 - recorded_positions.frame_start) < position.0 {
                break;
            }
            player.translation = position.1;

            recorded_positions.last_played_frame = position.0 as usize;
        }
    }
}
