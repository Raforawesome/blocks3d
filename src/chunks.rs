use bevy::prelude::*;

use crate::blocks::BlockType;

const WIDTH: usize = 16;
const LENGTH: usize = 16;
const HEIGHT: usize = 16;

pub struct Chunk {
    pub blocks: [[[BlockType; WIDTH]; HEIGHT]; LENGTH],
    world_pos: IVec3,
}
