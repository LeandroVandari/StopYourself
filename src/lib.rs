use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::modes::GameMode;

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
    Paused,
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (Self::create_level_dimensions, Self::start_background_music),
        )
        .add_systems(Update, Self::update_level_dimensions)
        .add_systems(FixedPreUpdate, update_state)
        .init_state::<GameState>()
        .init_state::<GameMode>()
        .init_asset::<AudioSource>();
    }
}

// Anything that can mutate state should run before this.
fn update_state(world: &mut World) {
    world.try_run_schedule(StateTransition).unwrap();
}

#[derive(Debug, Resource)]
pub struct LevelDimensions {
    start: Vec2,
    tile_size: f32,
    // length in tiles
    level_length: u32,
}

impl SetupPlugin {
    fn create_level_dimensions(
        mut commands: Commands,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        let level_length = 75;
        commands.insert_resource(LevelDimensions {
            start: window.size().map(|dimension| -dimension / 2.),
            tile_size: window.size().x / level_length as f32,
            level_length: level_length,
        });
    }

    fn update_level_dimensions(
        mut resize_reader: EventReader<WindowResized>,
        mut level_dimensions: ResMut<LevelDimensions>,
    ) {
        for new_size in resize_reader.read() {
            level_dimensions.start = vec2(-new_size.width / 2., -new_size.height / 2.);
            level_dimensions.tile_size = new_size.width / level_dimensions.level_length as f32
        }
    }
    fn start_background_music(mut asset_server: Res<AssetServer>, mut commands: Commands) {
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/bg_music.wav")),
            PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::Linear(0.5)),
        ));
    }
}

impl LevelDimensions {
    /// Uses top-left anchor (because the physics engine doesn't work with anchors, we need to do this manually).
    /// If you want a center anchor, just pass Vec2::ZERO as the object_size.
    pub fn grid_pos_to_pixels(&self, pos: (i32, i32), object_size: Vec2) -> Vec2 {
        self.start
            + vec2(
                pos.0 as f32 * self.tile_size + object_size.x / 2.,
                pos.1 as f32 * self.tile_size + object_size.y / 2.,
            )
    }
}
