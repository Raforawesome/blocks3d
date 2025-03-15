use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use blocks3d::blocks::{BlockTextures, BlockType, set_block, setup_block_materials};
use blocks3d::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), PlayerPlugin))
        .add_systems(Startup, (setup_block_materials, setup).chain())
        .add_systems(Update, grab_mouse)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    btex_reg: Res<BlockTextures>,
    // mut mats: ResMut<Assets<StandardMaterial>>,
) {
    // create baseplate
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::cuboid(20.0, 1.0, 20.0),
    //     Mesh3d(meshes.add(Cuboid::new(20.0, 1.0, 20.0))),
    //     MeshMaterial3d(mats.add(Color::srgb(0.5, 0.55, 0.55))),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));

    for x in -5..=5 {
        for z in -5..=5 {
            set_block(
                BlockType::Grass,
                (x as f32, 0.0, z as f32),
                &mut meshes,
                &btex_reg,
                &mut commands,
            );
        }
    }

    // Create a tree with a trunk and leaves
    // Trunk
    for y in 1..5 {
        set_block(
            BlockType::Log,
            (3.0, y as f32, 3.0),
            &mut meshes,
            &btex_reg,
            &mut commands,
        );
    }

    // Leaves
    for x in 2..=4 {
        for y in 5..7 {
            for z in 2..=4 {
                set_block(
                    BlockType::Leaves,
                    (x as f32, y as f32, z as f32),
                    &mut meshes,
                    &btex_reg,
                    &mut commands,
                );
            }
        }
    }

    // Add a wider leaf layer at bottom of canopy
    for x in 1..=5 {
        for z in 1..=5 {
            set_block(
                BlockType::Leaves,
                (x as f32, 4.0, z as f32),
                &mut meshes,
                &btex_reg,
                &mut commands,
            );
        }
    }

    // light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.95, 0.85),
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
