use avian3d::prelude::*;
use bevy::{math::vec3, prelude::*};
use blocks3d::NumExt;

use super::Player;

/// The maximum distance between the "ground position" (as measured by raycasting)
/// and the player. If a player is within this threshold, they are considered grounded.
pub const GRAVITY_EPSILON: f32 = 1e-4;
pub const GRAVITY_RATE: f32 = -9.8; // how fast gravity accelerates the player downwards
pub const PLAYER_ACCEL: f32 = 20.0; // player directional speed, measured in units per second
pub const MAX_PLAYER_VEL: f32 = 5.0; // threshold to stop accelerating after
pub const DAMPING_FACTOR: f32 = 0.9;

pub const TERMINAL_VELOCITY: f32 = 15.0; // TODO: replace this with real term vel calcs later

#[derive(Component, Default)]
#[allow(unused)]
pub struct KbMovementController {
    moving: bool,
    ground_height: f32,
}

/// # Ground height updater
/// Reads player's shapecaster to find the "ground" relative to player's position
pub fn player_height_update(
    mut caster: Query<(&ShapeHits, &mut KbMovementController), With<Player>>,
) {
    let (hits, mut mctrl) = caster.single_mut();
    let Some(ground_data) = hits.iter().next() else {
        // character is floating
        return;
    };
    mctrl.ground_height = ground_data.point1.y;
}

pub fn gravity_frame_step(
    mut player_query: Query<(&mut LinearVelocity, &Transform, &KbMovementController), With<Player>>,
    time: Res<Time>,
) {
    let (mut vel, plr_trn, mctrl) = player_query.single_mut();

    if plr_trn.translation.y - mctrl.ground_height > GRAVITY_EPSILON + 1.0 {
        if vel.y < TERMINAL_VELOCITY {
            vel.y = (vel.y + GRAVITY_RATE * time.delta_secs()).clamp(-TERMINAL_VELOCITY, 0.0);
        }
    } else {
        vel.y = 0.0;
    }
}

/// Handles WASD input and dampens velocity when not moving.
pub fn update_player_velocity(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut LinearVelocity, &Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut vel, plr_trn) = player.single_mut();

    let mut forward: f32 = 0.0;
    let mut sideways: f32 = 0.0;
    if input.pressed(KeyCode::KeyW) {
        forward += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        sideways -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        forward -= 1.0
    }
    if input.pressed(KeyCode::KeyD) {
        sideways += 1.0;
    }

    if (sideways != 0.0 || forward != 0.0)
        && vel.x.is_between(-MAX_PLAYER_VEL, MAX_PLAYER_VEL)
        && vel.z.is_between(-MAX_PLAYER_VEL, MAX_PLAYER_VEL)
    {
        let dt = time.delta_secs();
        let factor = PLAYER_ACCEL * dt;

        let fvec = vec3(plr_trn.forward().x, 0.0, plr_trn.forward().z); // skip calculations on y
        let rvec = vec3(plr_trn.right().x, 0.0, plr_trn.right().z); // skip calculations on y

        let mut movement = fvec * forward + rvec * sideways;
        movement = movement.normalize_or_zero();

        if movement != Vec3::ZERO {
            vel.x += movement.x * factor;
            vel.z += movement.z * factor;
        }
    } else {
        // if player isnt moving then start dampening velocity
        if vel.x != 0.0 {
            vel.x *= DAMPING_FACTOR;
        }
        if vel.z != 0.0 {
            vel.z *= DAMPING_FACTOR;
        }
    }
}
