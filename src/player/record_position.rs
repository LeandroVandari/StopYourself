use std::collections::VecDeque;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    diagnostic::FrameCount,
    ecs::resource::Resource,
    prelude::*,
};

use crate::player::PositionEvent;

pub struct RecordPositionPlugin;

#[derive(Debug, Resource)]
pub struct RecordedPositions {
    /// Which frame the positions started being recorded in
    pub(crate) frame_start: u32,
    pub(crate) positions: VecDeque<(u32, PositionEvent)>, // (Frame of the position, position)
}

impl Plugin for RecordPositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RecordedPositions {
            frame_start: 0,
            positions: VecDeque::with_capacity(256),
        })
        .add_systems(FixedUpdate, (Self::record_position));
    }
}

impl RecordPositionPlugin {
    fn record_position(
        mut position_event_reader: EventReader<PositionEvent>,
        mut recorded_positions: ResMut<RecordedPositions>,
        frame_counter: Res<FrameCount>,
    ) {
        for position in position_event_reader.read() {
            // info!("Recorded position! `{:?}`", position);
            if recorded_positions.positions.is_empty() {
                recorded_positions.frame_start = frame_counter.0;
            }
            let frame_from_start = frame_counter.0 - recorded_positions.frame_start;
            recorded_positions
                .positions
                .push_back((frame_from_start, *position))
        }
    }
}
