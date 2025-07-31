use bevy::{diagnostic::FrameCount, prelude::*};

use crate::player::movement::{self, MovementAction};

pub struct RecordMovementPlugin;

#[derive(Debug, Resource)]
pub struct RecordedMovements {
    /// Which frame the movements started being recorded in
    frame_start: u32,
    movements: Vec<(u32, MovementAction)>, // (Frame of the action, action)
}

impl Plugin for RecordMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RecordedMovements {
            frame_start: 0,
            movements: Vec::with_capacity(256),
        })
        .add_systems(
            FixedUpdate,
            Self::record_input.after(movement::PlayerMovementPlugin::keyboard_input),
        );
    }
}
impl RecordMovementPlugin {
    fn record_input(
        mut movement_event_reader: EventReader<MovementAction>,
        mut recorded_movements: ResMut<RecordedMovements>,
        frame_counter: Res<FrameCount>,
    ) {
        for movement in movement_event_reader.read() {
            recorded_movements
                .movements
                .push((frame_counter.0, *movement))
        }
    }
}
