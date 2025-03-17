use std::sync::atomic::{AtomicBool, Ordering};

use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::blocks::BlockType;

pub fn debug_wireframe(
    input: Res<ButtonInput<KeyCode>>,
    blocks: Query<&Transform, With<BlockType>>,
    mut gizmos: Gizmos,
) {
    static OVERLAY_ON: AtomicBool = AtomicBool::new(false);

    if input.just_pressed(KeyCode::F4) {
        let cur: bool = OVERLAY_ON.load(Ordering::Relaxed);
        OVERLAY_ON.swap(!cur, Ordering::Relaxed);
    }

    if OVERLAY_ON.load(Ordering::Relaxed) {
        for block_trn in blocks.iter() {
            gizmos.cuboid(*block_trn, WHITE);
        }
    }
}
