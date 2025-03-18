pub mod blocks;
pub mod chunks;
pub mod debug;
pub mod player;
pub mod ui;
pub mod worldgen;

pub type Coords = (f32, f32, f32);

#[derive(Clone, Copy)]
pub enum FaceDir {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl FaceDir {
    pub fn all() -> [FaceDir; 6] {
        [
            FaceDir::PosX,
            FaceDir::NegX,
            FaceDir::PosY,
            FaceDir::NegY,
            FaceDir::PosZ,
            FaceDir::NegZ,
        ]
    }

    pub fn normal(&self) -> [f32; 3] {
        match self {
            FaceDir::PosX => [1.0, 0.0, 0.0],
            FaceDir::NegX => [-1.0, 0.0, 0.0],
            FaceDir::PosY => [0.0, 1.0, 0.0],
            FaceDir::NegY => [0.0, -1.0, 0.0],
            FaceDir::PosZ => [0.0, 0.0, 1.0],
            FaceDir::NegZ => [0.0, 0.0, -1.0],
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            FaceDir::PosX | FaceDir::NegX => Axis::X,
            FaceDir::PosY | FaceDir::NegY => Axis::Y,
            FaceDir::PosZ | FaceDir::NegZ => Axis::Z,
        }
    }

    pub fn perp_axes(&self) -> (Axis, Axis) {
        match self {
            FaceDir::PosX | FaceDir::NegX => (Axis::Y, Axis::Z),
            FaceDir::PosY | FaceDir::NegY => (Axis::X, Axis::Z),
            FaceDir::PosZ | FaceDir::NegZ => (Axis::X, Axis::Y),
        }
    }

    pub fn positive(&self) -> bool {
        match self {
            FaceDir::PosX | FaceDir::PosY | FaceDir::PosZ => true,
            FaceDir::NegX | FaceDir::NegY | FaceDir::NegZ => false,
        }
    }

    pub fn negative(&self) -> bool {
        match self {
            FaceDir::PosX | FaceDir::PosY | FaceDir::PosZ => false,
            FaceDir::NegX | FaceDir::NegY | FaceDir::NegZ => true,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            FaceDir::PosX => FaceDir::NegX,
            FaceDir::NegX => FaceDir::PosX,
            FaceDir::PosY => FaceDir::NegY,
            FaceDir::NegY => FaceDir::PosY,
            FaceDir::PosZ => FaceDir::NegZ,
            FaceDir::NegZ => FaceDir::PosZ,
        }
    }
}

pub trait CmpExt {
    fn is_between(&self, lower: Self, upper: Self) -> bool;
}

impl<T: PartialOrd> CmpExt for T {
    fn is_between(&self, lower: Self, upper: Self) -> bool {
        *self >= lower && *self <= upper
    }
}
