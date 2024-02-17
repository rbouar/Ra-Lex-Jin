use std::ops::Sub;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

use crate::dungeon::{CollisionTile, Wall};

use super::*;

// pub enum DamageType {
//     Physical,
//     Magical,
//     True,
// }

#[derive(Component)]
pub struct Attack {
    // pub damage_type: DamageType,
    pub amount: i32,
}

impl Attack {
    pub fn new(amount: i32) -> Self {
        Attack { amount }
    }
}

#[derive(Component)]
pub struct Projectile;

pub fn player_attack(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    player_query: Query<(&Transform, &LinearVelocity), With<Player>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    asset_server: Res<AssetServer>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        if let Some(cursor_position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let (player_transform, player_linear_velocity) = player_query.single();
            let player_translation = player_transform.translation;
            let player_2d_translation = player_translation.truncate();
            let direction = match (cursor_position - player_2d_translation).try_normalize() {
                Some(direction) => direction,
                None => Vec2::X,
            };
            let velocity = LinearVelocity {
                0: 100. * direction + player_linear_velocity.0,
            };

            let mut fireball = commands.spawn_empty();

            fireball
                .insert((
                    Attack::new(10),
                    Collider::ball(10.),
                    Sensor,
                    RigidBody::Dynamic,
                    velocity,
                    LockedAxes::ROTATION_LOCKED,
                    SpriteBundle {
                        texture: asset_server.load("fireball.png"),
                        ..default()
                    },
                    Projectile,
                ))
                .insert(TransformBundle::from_transform(*player_transform));
        }
    }
}

pub fn fireball_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    wall_query: Query<With<Wall>>,
    projectile_query: Query<With<Projectile>>,
) {
    for &CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        for (entity1, entity2) in [(entity1, entity2), (entity2, entity1)] {
            if projectile_query.contains(entity1) && wall_query.contains(entity2) {
                commands.entity(entity1).despawn_recursive();
            }
        }
    }
}
