use std::ops::{Add, Sub, Mul, Neg, Div};

pub trait Ring: PartialEq + Default + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> + From<u32> {}
impl<T> Ring for T where T: PartialEq + Default + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> + From<u32> {}

pub trait Field: Ring + Div<Output = Self> { }
impl<T> Field for T where T: Ring + Div<Output = Self> { }
