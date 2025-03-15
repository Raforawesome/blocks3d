pub mod blocks;
pub mod player;

pub trait CmpExt {
    fn is_between(&self, lower: Self, upper: Self) -> bool;
}

impl<T: PartialOrd> CmpExt for T {
    fn is_between(&self, lower: Self, upper: Self) -> bool {
        *self >= lower && *self <= upper
    }
}
