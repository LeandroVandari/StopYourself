use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    LevelDimensions,
    environment::ResetEnvironment,
    modes::GameMode,
    player::{movement::CharacterControllerBundle, record_position::RecordedPositions},
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
        .add_systems(Startup, Self::spawn_player)
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
                half_size: vec2(20., 40.),
            })),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
            // Movement
            CharacterControllerBundle::new(Collider::rectangle(40., 80.))
                .with_movement(1250., 0.92, 800.),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(1.5),
            Transform::from_translation(
                level_dimensions
                    .grid_pos_to_pixels((1, 3), vec2(40., 80.))
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
            .grid_pos_to_pixels((1, 3), vec2(40., 80.))
            .extend(1.);

        velocity.0 = Vec2::ZERO;
        recorded_positions.locked = false;
    }

    fn handle_death(
        mut commands: Commands,
        game_mode: Res<State<GameMode>>,
        mut state: ResMut<NextState<GameMode>>,
        mut reset_environment: EventWriter<ResetEnvironment>,

        mut recorded_positions: ResMut<RecordedPositions>,

        asset_server: Res<AssetServer>,
    ) {
        match game_mode.get() {
            GameMode::Survive => {
                info!("Player died in survive mode. Restarting mode.");
                recorded_positions.positions.clear();
                recorded_positions.locked = true;
            }
            GameMode::Replay => {
                info!("Player died in replay mode. Moving on to survive.");
                commands.spawn((
                    Text::new("Congrats!\nYou Killed Yourself"),
                    TextFont {
                        // This font is just a placeholder, feel free to change :)
                        font: asset_server.load("fonts/Yeti Sighting.ttf"),
                        font_size: 67.,
                        ..Default::default()
                    },
                    TextLayout::new_with_justify(JustifyText::Center),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(0.5),
                        top: Val::Percent(0.2),
                        right: Val::Percent(0.5),
                        ..Default::default()
                    },
                ));
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
