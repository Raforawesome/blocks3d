use avian3d::prelude::*;
use bevy::prelude::*;

pub fn batch_set_block(
    block_datas: impl IntoIterator<Item = (BlockType, (f32, f32, f32))>,
    btex_reg: &Res<BlockTextures>,
    mesh_reg: &Res<MeshRegistry>,
    commands: &mut Commands,
) {
    let block_datas: Vec<_> = block_datas
        .into_iter()
        .map(|(block_type, (x, y, z))| {
            (
                RigidBody::Static,
                Collider::cuboid(1.0, 1.0, 1.0),
                block_type,
                Transform::from_xyz(x, y, z),
                Mesh3d(mesh_reg.block.clone()),
                MeshMaterial3d(block_type.texture(btex_reg).clone()),
            )
        })
        .collect();

    commands.spawn_batch(block_datas);
}

pub fn set_block(
    block_type: BlockType,
    pos: (f32, f32, f32),
    btex_reg: &Res<BlockTextures>,
    mesh_reg: &Res<MeshRegistry>,
    commands: &mut Commands,
) {
    let (x, y, z) = pos;
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        block_type,
        Transform::from_xyz(x, y, z),
        Mesh3d(mesh_reg.block.clone()),
        MeshMaterial3d(block_type.texture(btex_reg).clone()),
    ));
}

#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub enum BlockType {
    // Air, // do i need an air type?
    Grass,
    Stone,
    Log,
    Leaves,
    Water,
}

impl BlockType {
    pub fn texture<'a>(&self, btex_reg: &'a Res<BlockTextures>) -> &'a Handle<StandardMaterial> {
        match *self {
            BlockType::Grass => &btex_reg.grass,
            BlockType::Stone => &btex_reg.stone,
            BlockType::Log => &btex_reg.log,
            BlockType::Leaves => &btex_reg.leaves,
            BlockType::Water => &btex_reg.water,
        }
    }
}

#[derive(Resource)]
pub struct BlockTextures {
    grass: Handle<StandardMaterial>,
    stone: Handle<StandardMaterial>,
    log: Handle<StandardMaterial>,
    leaves: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct MeshRegistry {
    block: Handle<Mesh>,
}

pub fn setup_block_materials(
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmd: Commands,
) {
    let textures = BlockTextures {
        grass: mats.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.3, 0.0),
            reflectance: 0.2,
            ..default()
        }),
        stone: mats.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
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
        water: mats.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.3, 0.8),
            reflectance: 0.5,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };
    cmd.insert_resource(textures);
    cmd.insert_resource(MeshRegistry {
        block: meshes.add(Cuboid::from_length(1.0)),
    });
}
