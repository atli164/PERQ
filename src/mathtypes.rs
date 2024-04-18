use std::ops::{Add, Sub, Mul, Neg, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::str::FromStr;
use std::fmt::Debug;
use rug::{Integer, Rational};

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

pub trait Group: PartialEq + Clone + Debug + Sized + Zero + FromStr + 
    GroupOps<Self, Self> +
    for<'r> GroupOps<&'r Self, Self> + 
    GroupAssign<Self> + 
    for<'r> GroupAssign<&'r Self> {}

impl<T> Group for T where T: PartialEq + Clone + Debug + Sized + Zero + FromStr +  
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
    for<'r> FieldAssign<&'r Self> +
    From<Rational> {}

impl<T> Field for T where T: Ring +
    FieldOps<Self, Self> +
    for<'r> FieldOps<&'r Self, Self> +
    FieldAssign<Self> +
    for<'r> FieldAssign<&'r Self> +
    From<Rational> {}

impl_zero_one_for_eq! { impl Zero, One for Integer, Integer::from(0), Integer::from(1) }
impl_zero_one_for_eq! { impl Zero, One for Rational, Rational::from((0u32, 1u32)), Rational::from((1u32, 1u32)) }
impl_zero_one_for_eq! { impl Zero, One for f32, 0.0, 1.0 }
impl_zero_one_for_eq! { impl Zero, One for f64, 0.0, 1.0 }

macro_rules! zero_one_impl_ints { 
    ($($t:ty)*) => ($(
        impl_zero_one_for_eq! { impl Zero, One for $t, 0, 1 }
    )*)
}

zero_one_impl_ints! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
