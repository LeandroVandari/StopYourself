//! Handle the player movement.
//! Loosely based on https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/

use avian2d::{
    math::{AdjustPrecision, Scalar, Vector},
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    GameState,
    modes::GameMode,
    player::{Player, record_position::RecordPositionPlugin},
};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                ((
                    Self::keyboard_input, // Only get the input if we're in the survive mode
                    Self::update_grounded,
                    Self::movement,
                    Self::apply_movement_damping,
                )
                    .run_if(in_state(GameMode::Survive)))
                .chain(),
                RecordPositionPlugin::play_recorded_position.run_if(in_state(GameMode::Replay)),
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_event::<MovementAction>()
        .add_event::<ActualJump>();
    }
}

#[derive(Debug, Event, Clone, Copy)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
}

/// Indicates whether the entity is grounded.
#[derive(Debug, Component)]
pub struct Grounded;

/// The acceleration used for character movement.
#[derive(Component, Debug)]
pub struct MovementAcceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Debug)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component, Debug)]
pub struct JumpImpulse(pub Scalar);

#[derive(Debug, Bundle)]
pub struct CharacterControllerBundle {
    player: Player,
    movement: MovementBundle,
    body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    ground_caster: ShapeCaster,
}

#[derive(Debug, Bundle)]
struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
}

#[derive(Event)]
pub struct ActualJump;

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar, jump_impulse: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30., 0.9, 0.7)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            player: Player,
            body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(1.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse);
        self
    }
}

impl PlayerMovementPlugin {
    pub(crate) fn keyboard_input(
        mut movement_event_writer: EventWriter<MovementAction>,
        keyboard: Res<ButtonInput<KeyCode>>,
    ) {
        let left = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

        let horizontal = right as i8 - left as i8;
        let direction = horizontal as Scalar;

        if direction != 0.0 {
            movement_event_writer.write(MovementAction::Move(direction));
        }

        if keyboard.pressed(KeyCode::Space) {
            movement_event_writer.write(MovementAction::Jump);
        }
    }

    /// Updates the [`Grounded`] status for character controllers.
    fn update_grounded(
        mut commands: Commands,
        mut query: Query<(Entity, &ShapeHits), With<Player>>,
    ) {
        for (entity, hits) in &mut query {
            // The character is grounded if the shape caster has a hit
            let is_grounded = hits.iter().next().is_some();

            if is_grounded {
                commands.entity(entity).insert(Grounded);
            } else {
                commands.entity(entity).remove::<Grounded>();
            }
        }
    }

    fn movement(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut jump_writer: EventWriter<ActualJump>,

        time: Res<Time>,
        mut movement_event_reader: EventReader<MovementAction>,
        mut controllers: Query<(
            &MovementAcceleration,
            &JumpImpulse,
            &mut LinearVelocity,
            Has<Grounded>,
        )>,
    ) {
        // Precision is adjusted so that the example works with
        // both the `f32` and `f64` features. Otherwise you don't need this.
        let delta_time = time.delta_secs_f64().adjust_precision();

        for event in movement_event_reader.read() {
            for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
                &mut controllers
            {
                match event {
                    MovementAction::Move(direction) => {
                        linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                    }
                    MovementAction::Jump => {
                        if is_grounded {
                            linear_velocity.y = jump_impulse.0;
                            commands
                                .spawn((AudioPlayer::new(asset_server.load("sounds/jump.wav")),));
                            jump_writer.write(ActualJump);
                        } else if linear_velocity.y > 0.0 {
                            linear_velocity.y += jump_impulse.0 * 0.05;
                        }
                    }
                }
            }
        }
    }

    /// Slows down movement in the X direction.
    fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
        for (damping_factor, mut linear_velocity) in &mut query {
            // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
            linear_velocity.x *= damping_factor.0;
        }
    }
}
