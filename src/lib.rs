use bevy::prelude::*;

pub mod environment;
pub mod player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
