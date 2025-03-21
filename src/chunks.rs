use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::{FaceDir, blocks::BlockType};

pub const CHUNK_LENGTH: usize = 16;

pub struct Chunk {
    pub blocks: [[[BlockType; CHUNK_LENGTH]; CHUNK_LENGTH]; CHUNK_LENGTH],
    world_pos: IVec3,
}

impl Chunk {
    /// Determines whether a face needs to be drawn a the meeting point of current and
    /// neighbour. Neighbour should only be [None] at a chunk border.
    pub fn needs_face(&self, current: BlockType, neighbour: Option<BlockType>) -> bool {
        current.is_solid()
            && match neighbour {
                Some(other) => other.is_air(),
                None => true,
            }
    }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> Option<BlockType> {
        if x < CHUNK_LENGTH && y < CHUNK_LENGTH && z < CHUNK_LENGTH {
            Some(self.blocks[x][y][z])
        } else {
            None
        }
    }

    pub fn generate_mesh(&self) -> Mesh {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for dir in FaceDir::all() {
            self.greedy_mesh_direction(
                self,
                dir,
                &mut positions,
                &mut normals,
                &mut uvs,
                &mut indices,
            );
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }

    fn greedy_mesh_direction(
        &self,
        arg: &Chunk,
        dir: FaceDir,
        positions: &[[f32; 3]],
        normals: &[[f32; 3]],
        uvs: &[[f32; 2]],
        indices: &[u32],
    ) -> _ {
        todo!()
    }
}
