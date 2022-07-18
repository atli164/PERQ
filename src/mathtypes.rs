use std::ops::{Add, Sub, Mul, Neg, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt::Debug;
use rug::Rational;

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

trait GroupOps<Rhs, Output>:
    Add<Rhs, Output = Output> + 
    Sub<Rhs, Output = Output> +
    Neg<Output = Output> {}

impl<T, Rhs, Output> GroupOps<Rhs, Output> for T where T:
    Add<Rhs, Output = Output> +
    Sub<Rhs, Output = Output> +
    Neg<Output = Output> {}

trait GroupAssign<Rhs>:
    AddAssign<Rhs> +
    SubAssign<Rhs> {}

impl<T, Rhs> GroupAssign<Rhs> for T where T:
    AddAssign<Rhs> +
    SubAssign<Rhs> {}

pub trait Group: PartialEq + Eq + Zero + Clone + Debug +
    GroupOps<Self, Self> +
    for<'r> GroupOps<&'r Self, Self> + 
    GroupAssign<Self> + 
    for<'r> GroupAssign<&'r Self> {}

impl<T> Group for T where T: PartialEq + Eq + Zero + Clone + Debug + 
    GroupOps<Self, Self> +
    for<'r> GroupOps<&'r Self, Self> + 
    GroupAssign<Self> + 
    for<'r> GroupAssign<&'r Self> {}

trait RingOps<Rhs, Output>: Mul<Rhs, Output = Output> {}
impl<T, Rhs, Output> RingOps<Rhs, Output> for T where T: Mul<Rhs, Output = Output> {}

trait RingAssign<Rhs>: MulAssign<Rhs> {}
impl<T, Rhs> RingAssign<Rhs> for T where T: MulAssign<Rhs> {}

pub trait Ring: Group + One + std::convert::From<u32> + 
    RingOps<Self, Self> +
    for<'r> RingOps<&'r Self, Self> +
    RingAssign<Self> +
    for<'r> RingAssign<&'r Self> {}

impl<T> Ring for T where T: Group + One + std::convert::From<u32> +
    RingOps<Self, Self> +
    for<'r> RingOps<&'r Self, Self> +
    RingAssign<Self> +
    for<'r> RingAssign<&'r Self> {}

trait FieldOps<Rhs, Output>: Div<Rhs, Output = Output> {}
impl<T, Rhs, Output> FieldOps<Rhs, Output> for T where T: Div<Rhs, Output = Output> {}

trait FieldAssign<Rhs>: DivAssign<Rhs> {}
impl<T, Rhs> FieldAssign<Rhs> for T where T: DivAssign<Rhs> {}

pub trait Field: Ring +
    FieldOps<Self, Self> +
    for<'r> FieldOps<&'r Self, Self> +
    FieldAssign<Self> +
    for<'r> FieldAssign<&'r Self> {}

impl<T> Field for T where T: Ring +
    FieldOps<Self, Self> +
    for<'r> FieldOps<&'r Self, Self> +
    FieldAssign<Self> +
    for<'r> FieldAssign<&'r Self> {}

pub trait PowerSeries: Field {
    type Coeff: Field;

    // Can only have one impl, so promote needs to exist rather than
    // just having impl<T> From<T> for Powerseries<T>
    fn promote(coeff: Self::Coeff) -> Self;
    // Helper function to return polynomial identity function
    fn identity() -> Self;
    fn coefficient(&self, i: usize) -> Self::Coeff;
    fn derive(&self) -> Self;
    fn integrate(&self) -> Self;
    // Compositional inverse, multiplicative inverse is done through Div
    fn inverse(&self) -> Self;
    fn compose(&self, other: &Self) -> Self;
    fn hadamard(&self, other: &Self) -> Self;
    fn sqrt(&self) -> Self;
    // Rational power
    fn ratpow(self, p: i64, q: i64) -> Self;
    // Tail operation
    fn lshift(&self) -> Self;
    // Multiply by x
    fn rshift(&self) -> Self;
}

impl Zero for Rational {
    fn zero() -> Rational {
        Rational::new()
    }
    fn is_zero(&self) -> bool {
        self.cmp0() == std::cmp::Ordering::Equal
    }
}

impl One for Rational {
    fn one() -> Rational {
        Rational::from((1u32, 1u32))
    }
    fn is_one(&self) -> bool {
        self == &Rational::from((1u32, 1u32))
    }
}
