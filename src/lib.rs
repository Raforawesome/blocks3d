pub mod blocks;
pub mod chunks;
pub mod debug;
pub mod player;
pub mod ui;
pub mod worldgen;

pub type Coords = (f32, f32, f32);

pub trait CmpExt {
    fn is_between(&self, lower: Self, upper: Self) -> bool;
}

impl<T: PartialOrd> CmpExt for T {
    fn is_between(&self, lower: Self, upper: Self) -> bool {
        *self >= lower && *self <= upper
    }
}
