use std::ops::{Add, Sub, Mul, Neg, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::str::FromStr;
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

pub trait GroupOps<Rhs, Output>:
    Add<Rhs, Output = Output> + 
    Sub<Rhs, Output = Output> +
    Neg<Output = Output> {}

impl<T, Rhs, Output> GroupOps<Rhs, Output> for T where T:
    Add<Rhs, Output = Output> +
    Sub<Rhs, Output = Output> +
    Neg<Output = Output> {}

pub trait GroupAssign<Rhs>:
    AddAssign<Rhs> +
    SubAssign<Rhs> {}

impl<T, Rhs> GroupAssign<Rhs> for T where T:
    AddAssign<Rhs> +
    SubAssign<Rhs> {}

pub trait Group: PartialEq + Eq + Clone + Debug + Sized + Zero + FromStr + 
    GroupOps<Self, Self> +
    for<'r> GroupOps<&'r Self, Self> + 
    GroupAssign<Self> + 
    for<'r> GroupAssign<&'r Self> {}

impl<T> Group for T where T: PartialEq + Eq + Clone + Debug + Sized + Zero + FromStr +  
    GroupOps<Self, Self> +
    for<'r> GroupOps<&'r Self, Self> + 
    GroupAssign<Self> + 
    for<'r> GroupAssign<&'r Self> {}

pub trait RingOps<Rhs, Output>: Mul<Rhs, Output = Output> {}

impl<T, Rhs, Output> RingOps<Rhs, Output> for T where T: Mul<Rhs, Output = Output> {}

pub trait RingAssign<Rhs>: MulAssign<Rhs> {}

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

pub trait FieldOps<Rhs, Output>: Div<Rhs, Output = Output> {}

impl<T, Rhs, Output> FieldOps<Rhs, Output> for T where T: Div<Rhs, Output = Output> {}

pub trait FieldAssign<Rhs>: DivAssign<Rhs> {}

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
