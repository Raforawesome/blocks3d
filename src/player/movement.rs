use avian3d::prelude::*;
use bevy::prelude::*;

use super::{Player, PlayerCaster};

#[derive(Component)]
pub struct PlayerMovementController {
    direction: Vec3,
    velocity: f32,
}

pub fn player_height_update(caster: Query<&ShapeHits, With<Player>>, mut commands: Commands) {
    let hits = caster.single();
    let Some(ground_data) = hits.iter().next() else {
        // character is floating
        println!("no hits");
        return;
    };
    println!(
        "closest ground: {:?}, dist: {}, entity: {}",
        ground_data.point1, ground_data.distance, ground_data.entity
    );
    commands.entity(ground_data.entity).log_components();
}
