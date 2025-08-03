use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameState, LevelDimensions,
    environment::ResetEnvironment,
    modes::GameMode,
    player::{
        movement::{CharacterControllerBundle, MovementAction},
        record_position::RecordedPositions,
    },
};

/// Player died
#[derive(Debug, Event)]
pub struct PlayerDeath;

mod movement;
//pub mod record_movement;
pub mod record_position;
/// Player spawning and movement handling.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::PlayerMovementPlugin,
            record_position::RecordPositionPlugin,
        ))
        .add_event::<ResetEnvironment>()
        .add_event::<PlayerDeath>()
        .add_systems(OnExit(GameState::Menu), Self::spawn_player)
        .add_systems(
            FixedUpdate,
            Self::move_to_start_pos.run_if(on_event::<ResetEnvironment>),
        )
        .add_systems(
            FixedPreUpdate,
            (Self::handle_death)
                .run_if(on_event::<PlayerDeath>)
                .before(crate::update_state),
        );
    }
}
/// Marker for the player character.
#[derive(Debug, Component)]
pub struct Player;

impl PlayerPlugin {
    fn spawn_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        level_dimensions: Res<LevelDimensions>,
    ) {
        commands.spawn((
            // Appearance
            Mesh2d(meshes.add(Rectangle {
                half_size: vec2(20., 20.),
            })),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            // Movement
            CharacterControllerBundle::new(Collider::rectangle(40., 40.))
                .with_movement(6250., 0.82, 1600.),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(8.0),
            Transform::from_translation(
                level_dimensions
                    .grid_pos_to_pixels((1, 3), vec2(40., 40.))
                    .extend(1.),
            ),
        ));
    }

    fn move_to_start_pos(
        player: Single<(&mut Transform, &mut LinearVelocity), With<Player>>,
        level_dimensions: Res<LevelDimensions>,
        mut recorded_positions: ResMut<RecordedPositions>,
    ) {
        let (mut transform, mut velocity) = player.into_inner();
        transform.translation = level_dimensions
            .grid_pos_to_pixels((1, 3), vec2(40., 40.))
            .extend(1.);

        velocity.0 = Vec2::ZERO;
        recorded_positions.locked = false;
    }

    fn handle_death(
        game_mode: Res<State<GameMode>>,
        mut state: ResMut<NextState<GameMode>>,
        mut reset_environment: EventWriter<ResetEnvironment>,

        mut recorded_positions: ResMut<RecordedPositions>,

        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/death.wav")));
        match game_mode.get() {
            GameMode::Survive => {
                info!("Player died in survive mode. Restarting mode.");
                recorded_positions.positions.clear();
                recorded_positions.locked = true;
            }
            GameMode::Replay => {
                info!("Player died in replay mode. Moving on to survive.");
                state.set(GameMode::Survive);
                recorded_positions.positions.clear();
                recorded_positions.locked = true;
            }
            GameMode::Defend => {
                warn!(
                    "Player should not die in the defend game mode. This probably happened because they touched the flag and something killed them at the same time."
                );
                return;
            }
        }
        reset_environment.write(ResetEnvironment);
    }
}
