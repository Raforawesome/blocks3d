mod movement;

use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::math::vec2;
use bevy::prelude::*;
use movement::{
    KbMovementController, gravity_frame_step, player_height_update, update_player_velocity,
};

#[derive(Component, Deref)]
/// Editable camera sensitivity settings stored in the player struct
/// TODO: Expand into full settings later (make a settings menu first)
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        // it's often nicer to have a faster horizontal sensitivity than vertical
        Self(vec2(0.003, 0.002))
    }
}

/// Marker for the player entity
#[derive(Component)]
#[require(
    CameraSensitivity, KbMovementController,
    RigidBody(|| RigidBody::Kinematic),
    Collider(|| Collider::capsule(0.4, 1.0)),     // player hitbox
    Transform(|| Transform::from_xyz(0.0, 30.0, 0.0)), // spawn point
)]
pub struct Player;

pub fn update_player_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let delta = mouse_motion.delta;

    if delta != Vec2::ZERO {
        let (mut plr_trn, sens) = player.single_mut();

        let delta_yaw = -delta.x * sens.x; // negative to make yaw go clockwise
        let delta_pitch = -delta.y * sens.y; // negative to make pitch go upwards

        let (yaw, pitch, roll) = plr_trn.rotation.to_euler(EulerRot::YXZ);
        let new_yaw: f32 = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01; // avoid gimbal lock
        let new_pitch: f32 = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        plr_trn.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, roll);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let plr: Entity = commands.spawn(Player).id();
    commands
        .entity(plr)
        .insert(Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))))
        .insert(MeshMaterial3d(mats.add(Color::BLACK)))
        .insert(ShapeCaster::new(
            Collider::capsule(0.4, 1.0),
            Vec3::ZERO,
            Quat::default(),
            Dir3::NEG_Y,
        ))
        .with_children(|parent| {
            parent.spawn((
                // spawn player camera
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Projection::from(PerspectiveProjection {
                    // fov: 80.0_f32.to_radians(),
                    ..default()
                }),
            ));
        });
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                update_player_look,
                update_player_velocity,
                player_height_update,
                gravity_frame_step,
            ),
        );
    }
}
