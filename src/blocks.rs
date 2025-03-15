use avian3d::prelude::*;
use bevy::prelude::*;

pub fn set_block(
    block_type: BlockType,
    pos: (f32, f32, f32),
    meshes: &mut ResMut<Assets<Mesh>>,
    btex_reg: &Res<BlockTextures>,
    commands: &mut Commands,
) {
    let (x, y, z) = pos;
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        block_type,
        Transform::from_xyz(x, y, z),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(block_type.texture(btex_reg).clone()),
    ));
}

#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub enum BlockType {
    // Air, // do i need an air type?
    Grass,
    Log,
    Leaves,
}

impl BlockType {
    pub fn texture<'a>(&self, btex_reg: &'a Res<BlockTextures>) -> &'a Handle<StandardMaterial> {
        match *self {
            BlockType::Grass => &btex_reg.grass,
            BlockType::Log => &btex_reg.log,
            BlockType::Leaves => &btex_reg.leaves,
        }
    }
}

#[derive(Resource)]
pub struct BlockTextures {
    grass: Handle<StandardMaterial>,
    log: Handle<StandardMaterial>,
    leaves: Handle<StandardMaterial>,
}

pub fn setup_block_materials(mut mats: ResMut<Assets<StandardMaterial>>, mut cmd: Commands) {
    let textures = BlockTextures {
        grass: mats.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.3, 0.0),
            reflectance: 0.2,
            ..default()
        }),
        log: mats.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.09, 0.0),
            reflectance: 0.0,
            ..default()
        }),
        leaves: mats.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.5, 0.1),
            reflectance: 0.1,
            // alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };
    cmd.insert_resource(textures);
}
