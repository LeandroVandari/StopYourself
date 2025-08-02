use bevy::{
    diagnostic::FrameCount, input::common_conditions::input_just_pressed, prelude::*,
    window::PrimaryWindow,
};

use crate::{modes::GameMode, player::record_position::RecordedPositions};

pub mod camera;
pub mod environment;
pub mod menu;
pub mod modes;
pub mod obstacles;
pub mod player;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
    Paused {
        frame_paused: u32,
    },
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::level_dimensions)
            .add_systems(FixedPreUpdate, update_state)
            .add_systems(
                Update,
                handle_pause.run_if(input_just_pressed(KeyCode::Escape)),
            )
            .init_state::<GameState>()
            .init_state::<GameMode>();
    }
}

// Anything that can mutate state should run before this.
fn update_state(world: &mut World) {
    world.try_run_schedule(StateTransition).unwrap();
}

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

#[derive(Debug, Resource)]
pub struct LevelDimensions {
    start: Vec2,
    tile_size: f32,
    // length in tiles
    level_length: u32,
}

impl SetupPlugin {
    fn level_dimensions(mut commands: Commands, window: Single<&Window, With<PrimaryWindow>>) {
        commands.insert_resource(LevelDimensions {
            start: window.size().map(|dimension| -dimension / 2.),
            tile_size: 20.,
            level_length: 75,
        });
    }
}

impl LevelDimensions {
    /// Uses top-left anchor (because the physics engine doesn't work with anchors, we need to do this manually).
    /// If you want a center anchor, just pass Vec2::ZERO as the object_size.
    pub fn grid_pos_to_pixels(&self, pos: (u32, u32), object_size: Vec2) -> Vec2 {
        self.start
            + vec2(
                pos.0 as f32 * self.tile_size + object_size.x / 2.,
                pos.1 as f32 * self.tile_size + object_size.y / 2.,
            )
    }
}
