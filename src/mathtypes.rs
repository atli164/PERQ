use std::ops::{Add, Sub, Mul, Neg, Div};

pub trait Ring: PartialEq + Default + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> + From<u32> {}
impl<T> Ring for T where T: PartialEq + Default + Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> + From<u32> {}

pub trait Field: Ring + Div<Output = Self> { }
impl<T> Field for T where T: Ring + Div<Output = Self> { }

pub trait PowerSeries: Field {
    type Coeff: Field;

    // Can only have one impl, so promote needs to exist rather than
    // just having impl<T> From<T> for Powerseries<T>
    fn promote(coeff: Self::Coeff) -> Self;
    // Helper function to return polynomial identity function
    fn identity() -> Self;
    fn compose(self, other: Self) -> Self;
    fn coefficient(self, i: usize) -> Self::Coeff;
    fn derive(self) -> Self;
    fn integrate(self) -> Self;
    // Compositional inverse, multiplicative inverse is done through Div
    fn inverse(self) -> Self;
}