pub trait NumExt {
    fn is_between(&self, lower: Self, upper: Self) -> bool;
}

impl<T: PartialOrd> NumExt for T {
    fn is_between(&self, lower: Self, upper: Self) -> bool {
        *self >= lower && *self <= upper
    }
}
