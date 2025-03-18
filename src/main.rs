use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use blocks3d::blocks::{
    BlockTextures, BlockType, MeshRegistry, batch_set_block, set_block, setup_block_materials,
};
use blocks3d::debug::debug_wireframe;
use blocks3d::player::{PlayerPlugin /*highlight_block*/};
use blocks3d::ui::GameUiPlugin;
use blocks3d::worldgen::{get_block_at, get_surface_and_y};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // PhysicsPlugins::default(),
            PlayerPlugin,
            GameUiPlugin,
        ))
        .add_systems(Startup, (setup_block_materials, setup).chain())
        .add_systems(Update, (grab_mouse, debug_wireframe /*highlight_block*/))
        .run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    btex_reg: Res<BlockTextures>,
    mesh_reg: Res<MeshRegistry>,
    // mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let mut batch: Vec<(BlockType, (f32, f32, f32))> = vec![];
    // for x in -35..=35 {
    //     for z in -35..=35 {
    //         let (x, z) = (x as f32, z as f32);
    //         let (block_type, y) = get_surface_and_y(x, z);
    //         batch.push((block_type, (x, y, z)));
    //     }
    // }
    // batch_set_block(batch, &btex_reg, &mesh_reg, &mut commands);
    for x in -20..=20 {
        for z in -20..=20 {
            for y in 80..=100 {
                let (x, y, z) = (x as f32, y as f32, z as f32);
                let block_type = get_block_at(x, y, z);
                if block_type != BlockType::Air {
                    batch.push((block_type, (x, y, z)));
                }
            }
        }
    }
    batch_set_block(batch, &btex_reg, &mesh_reg, &mut commands);

    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::cuboid(100.0, 100.0, 100.0),
    //     BlockType::Stone,
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     Mesh3d(meshes.add(Cuboid::new(400.0, 100.0, 400.0))),
    //     MeshMaterial3d(mats.add(Color::srgb(0.3, 0.3, 0.3))),
    // ));

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
