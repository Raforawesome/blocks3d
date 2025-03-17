use std::sync::LazyLock;

use bevy::prelude::*;
use noise::{NoiseFn as _, OpenSimplex, Perlin};

use crate::blocks::BlockType;

const OCEAN_LEVEL: f32 = 90.0; // baseline height to generate terrain at
const SCALE: f64 = 20.0; // horizontal stretch factor for heightmap

static TERRAIN_NOISE: LazyLock<OpenSimplex> = LazyLock::new(|| OpenSimplex::new(fastrand::u32(..)));

pub fn get_block_at(x: f32, y: f32, z: f32) -> Option<BlockType> {
    let surface_y =
        OCEAN_LEVEL + TERRAIN_NOISE.get([x as f64 / SCALE, z as f64 / SCALE]) as f32 * 20.0;
    if y < surface_y {
        println!("put stone at {x}, {y}, {z} (surface_y: {surface_y}");
        Some(BlockType::Stone)
    } else {
        println!("put air at {x}, {y}, {z} (surface_y: {surface_y}");
        None
    }
}

pub fn get_surface_and_y(x: f32, z: f32) -> (BlockType, f32) {
    let surface_y =
        OCEAN_LEVEL + TERRAIN_NOISE.get([x as f64 / SCALE, z as f64 / SCALE]) as f32 * 20.0;
    (BlockType::Grass, surface_y)
}
