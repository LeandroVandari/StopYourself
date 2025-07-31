use bevy::prelude::*;

/// The two modes for the game
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMode {
    /// Get to the goal bypassing the traps
    #[default]
    Survive,
    /// Place defenses to stop your replay from getting to the goal
    Defend
}