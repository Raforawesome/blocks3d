mod movement;

use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::math::vec2;
use bevy::prelude::*;
use movement::{
    KbMovementController,
    update_player_velocity,
    // gravity_frame_step, player_height_update,
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
    LockedAxes(|| LockedAxes::new().lock_rotation_x().lock_rotation_z()),
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::capsule(0.5, 2.0)),     // player hitbox
    Transform(|| Transform::from_xyz(0.0, 30.0, 0.0)), // spawn point
)]
pub struct Player;

pub fn update_player_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
    mut cam: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let delta = mouse_motion.delta;
    let (mut plr_trn, sens) = player.single_mut();
    let mut cam = cam.single_mut();
    cam.translation = plr_trn.translation;

    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01; // avoid gimbal lock
    let (cy, cp, cr) = cam.rotation.to_euler(EulerRot::YXZ); // cam pitch

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * sens.x; // negative to make yaw go clockwise
        let delta_pitch = -delta.y * sens.y; // negative to make pitch go upwards
        let new_yaw: f32 = cy + delta_yaw;
        let new_pitch: f32 = (cp + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        cam.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, cr);
    }

    plr_trn.rotation = Quat::from_euler(EulerRot::YXZ, cam.rotation.y, 0.0, 0.0);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let plr: Entity = commands.spawn(Player).id();
    commands.entity(plr).insert((
        Mesh3d(meshes.add(Capsule3d::new(0.5, 2.0))),
        MeshMaterial3d(mats.add(Color::BLACK)),
        // ShapeCaster::new(
        //     Collider::capsule(0.3, 0.1),
        //     Vec3::ZERO,
        //     Quat::default(),
        //     Dir3::NEG_Y,
        // ),
    ));
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            // fov: 80.0_f32.to_radians(),
            ..default()
        }),
    ));
    // .with_children(|parent| {
    //     parent.spawn((
    //         // spawn player camera
    //         Camera3d::default(),
    //         Transform::from_xyz(0.0, 1.0, 10.0),
    //         Projection::from(PerspectiveProjection {
    //             // fov: 80.0_f32.to_radians(),
    //             ..default()
    //         }),
    //     ));
    // });
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                (update_player_look, update_player_velocity).chain(),
                // player_height_update,
                // gravity_frame_step,
            ),
        );
    }
}
