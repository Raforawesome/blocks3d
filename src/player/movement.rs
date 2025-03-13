use avian3d::prelude::*;
use bevy::prelude::*;

use super::Player;

#[derive(Component, Default)]
#[allow(unused)]
pub struct PlayerMovementController {
    direction: Vec3,
    velocity: f32,
    ground_height: f32,
}

/// # Ground height updater
/// Reads player's shapecaster to find the "ground" relative to player's position
pub fn player_height_update(
    mut caster: Query<(&ShapeHits, &mut PlayerMovementController), With<Player>>,
    mut commands: Commands,
) {
    let (hits, mut ctrl) = caster.single_mut();
    let Some(ground_data) = hits.iter().next() else {
        // character is floating
        println!("no hits");
        return;
    };
    ctrl.ground_height = ground_data.point1.y;
}
