use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_xpbd_2d::{
    components::{Collider, LinearVelocity, LockedAxes, Restitution, RigidBody},
    math::{AdjustPrecision, Scalar, Vector2},
};

use crate::player::{PLAYER_ACCELERATION, PLAYER_DAMPING};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>().add_systems(
            Update,
            (
                keyboard_input,
                gamepad_input,
                apply_deferred,
                movement,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Default)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle, Default)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    rotation_constraints: LockedAxes,
    movement: MovementBundle,
    restitution: Restitution,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(10000.0, 0.95)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            movement: MovementBundle::default(),
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            restitution: Restitution::ZERO,
        }
    }

    pub fn with_movement(mut self, acceleration: Scalar, damping: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration, damping);
        self
    }
}

impl From<&EntityInstance> for CharacterControllerBundle {
    fn from(entity_instance: &EntityInstance) -> CharacterControllerBundle {
        let width = entity_instance.width as f32;
        let height = entity_instance.height as f32;

        let collider = Collider::cuboid(width, height);

        CharacterControllerBundle::new(collider).with_movement(PLAYER_ACCELERATION, PLAYER_DAMPING)
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::S, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::Z, KeyCode::Down]);
    let left = keyboard_input.any_pressed([KeyCode::Q, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0);

    if direction != Vector2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let axis_ly = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            movement_event_writer.send(MovementAction::Move(
                Vector2::new(x as Scalar, y as Scalar).clamp_length_max(1.0),
            ));
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(&MovementAcceleration, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, mut linear_velocity) in &mut controllers {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.y -= direction.y * movement_acceleration.0 * delta_time;
                }
            }
        }
    }
}

/// Slows down movement.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.y *= damping_factor.0;
    }
}
