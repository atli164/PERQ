use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero, pow};

// MOD assumed small enough to allow calculations
// within 64 bits, i.e. MOD < 2**32
// Also assumed to be prime
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct ModInt<const P: u64> {
    val: u64
}

impl<const P: u64> Zero for ModInt<P> {
    fn zero() -> Self {
        Self { val: 0 }
    }
    fn is_zero(&self) -> bool {
        self.val == 0
    }
}

impl<const P: u64> One for ModInt<P> {
    fn one() -> Self {
        Self { val: 1 }
    }
}

impl<const P: u64> Add for ModInt<P> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let res = self.val + other.val;
        ModInt {
            val: if res > P { res - P } else { res }
        }
    }
}

impl<const P: u64> Sub for ModInt<P> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let res = P + self.val - other.val;
        ModInt {
            val: if res > P { res - P } else { res }
        }
    }
}

impl<const P: u64> Neg for ModInt<P> {
    type Output = Self;
    fn neg(self) -> Self {
        ModInt {
            val: P - self.val
        }
    }
}

impl<const P: u64> Mul for ModInt<P> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ModInt {
            val: self.val * other.val % P
        }
    }
}

impl<const P: u64> Div for ModInt<P> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        ModInt {
            val: self.val * pow(other.val, (P - 2) as usize) % P
        }
    }
}

impl<const P: u64> From<u64> for ModInt<P> {
    fn from(x: u64) -> Self {
        ModInt { val: x % P }
    }
}