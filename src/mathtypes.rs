use std::ops::{Add, Sub, Mul, Neg, Div};

pub trait BaseType: PartialEq + Default + Clone + std::fmt::Debug {}
impl<T> BaseType for T where T: PartialEq + Default + Clone + std::fmt::Debug {}

pub trait Ring: BaseType + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> + From<u32> {}
impl<T> Ring for T where T: BaseType + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Neg<Output = Self> + From<u32> {}

pub trait Field: Ring + Div<Output = Self> { }
impl<T> Field for T where T: Ring + Div<Output = Self> { }

pub trait PowerSeries: Field {
    type Coeff: Field;

    // Can only have one impl, so promote needs to exist rather than
    // just having impl<T> From<T> for Powerseries<T>
    fn promote(coeff: Self::Coeff) -> Self;
    // Helper function to return polynomial identity function
    fn identity() -> Self;
    fn coefficient(self, i: usize) -> Self::Coeff;
    fn derive(self) -> Self;
    fn integrate(self) -> Self;
    // Compositional inverse, multiplicative inverse is done through Div
    fn inverse(self) -> Self;
    fn compose(self, other: Self) -> Self;
    fn hadamard(self, other: Self) -> Self;
    fn sqrt(self) -> Self;
    // Skip for now
    // fn ratpow(self, p: i64, q: i64) -> Self;
    // Tail operation
    fn lshift(self) -> Self;
    // Multiply by x
    fn rshift(self) -> Self;
}