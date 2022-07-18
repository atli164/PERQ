use std::ops::{Add, Sub, Mul, Neg, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::mathtypes::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ModIntP32 {
    x: u32
}
impl ModIntP32 {
    const MOD: u32 = 65521;
    #[inline]
    fn sq(self) -> Self {
        self * self
    }
    #[inline]
    fn inv(self) -> Self {
        let x2 = self.sq();
        let x4 = x2.sq();
        let x8 = x4.sq();
        let x9 = self * x8;
        let x11 = x2 * x9;
        let x22 = x11.sq();
        let y1 = x9 * x22; // 2^5 - 1
        let y2 = y1.sq().sq().sq().sq().sq() * y1; // 2^10 - 1
        let y3 = y2.sq() * self; // 2^11 - 1
        let y4 = y3.sq().sq().sq().sq().sq(); // 2^16 - 32
        y4 * x11 * x4 // 2^16 - 17 = p - 2
    }
}

impl Zero for ModIntP32 {
    fn zero() -> ModIntP32 {
        ModIntP32 {
            x: 0
        }
    }
    fn is_zero(&self) -> bool {
        self.x == 0
    }
}

impl One for ModIntP32 {
    fn one() -> ModIntP32 {
        ModIntP32 {
            x: 1
        }
    }
    fn is_one(&self) -> bool {
        self.x == 1
    }
}

impl<'a, 'b> Add<&'a ModIntP32> for &'b ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn add(self, other: &'a ModIntP32) -> ModIntP32 {
        let res = self.x + other.x;
        ModIntP32 {
            x: if res >= ModIntP32::MOD { res - ModIntP32::MOD } else { res }
        }
    }
}

forward_binop_impl! { impl Add, add for ModIntP32 }

impl<'a> AddAssign<&'a ModIntP32> for ModIntP32 {
    #[inline]
    fn add_assign(&mut self, other: &'a ModIntP32) {
        self.x += other.x;
        if self.x >= ModIntP32::MOD {
            self.x -= ModIntP32::MOD;
        }
    }
}

forward_assign_impl! { impl AddAssign, add_assign for ModIntP32 }

impl<'a, 'b> Sub<&'a ModIntP32> for &'b ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn sub(self, other: &'a ModIntP32) -> ModIntP32 {
        let res = ModIntP32::MOD + self.x - other.x;
        ModIntP32 {
            x: if res >= ModIntP32::MOD { res - ModIntP32::MOD } else { res }
        }
    }
}

forward_binop_impl! { impl Sub, sub for ModIntP32 }

impl<'a> SubAssign<&'a ModIntP32> for ModIntP32 {
    #[inline]
    fn sub_assign(&mut self, other: &'a ModIntP32) {
        self.x += ModIntP32::MOD - other.x;
        if self.x >= ModIntP32::MOD {
            self.x -= ModIntP32::MOD;
        }
    }
}

forward_assign_impl! { impl SubAssign, sub_assign for ModIntP32 }

impl<'a> Neg for &'a ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn neg(self) -> ModIntP32 {
        ModIntP32 {
            x: ModIntP32::MOD - self.x
        }
    }
}

forward_unop_impl! { impl Neg, neg for ModIntP32 }

impl<'a, 'b> Mul<&'a ModIntP32> for &'b ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn mul(self, other: &'a ModIntP32) -> ModIntP32 {
        ModIntP32 {
            x: (self.x * other.x) % ModIntP32::MOD
        }
    }
}

forward_binop_impl! { impl Mul, mul for ModIntP32 }

impl<'a> MulAssign<&'a ModIntP32> for ModIntP32 {
    #[inline]
    fn mul_assign(&mut self, other: &'a ModIntP32) {
        self.x *= other.x;
        self.x %= ModIntP32::MOD;
    }
}

forward_assign_impl! { impl MulAssign, mul_assign for ModIntP32 }

impl<'a, 'b> Div<&'a ModIntP32> for &'b ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn div(self, other: &'a ModIntP32) -> ModIntP32 {
        self * other.inv()
    }
}

forward_binop_impl! { impl Div, div for ModIntP32 }

impl<'a> DivAssign<&'a ModIntP32> for ModIntP32 {
    #[inline]
    fn div_assign(&mut self, other: &'a ModIntP32) {
        *self *= other.inv();
    }
}

forward_assign_impl! { impl DivAssign, div_assign for ModIntP32 }

impl From<u32> for ModIntP32 {
    #[inline]
    fn from(x: u32) -> ModIntP32 {
        ModIntP32 {
            x: x % ModIntP32::MOD
        }
    }
}

impl std::str::FromStr for ModIntP32 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(());
        }
        let mut res = ModIntP32::from(0u32);
        let mut neg = false;
        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                neg = true;
                continue;
            }
            res = res * ModIntP32::from(10u32);
            match c.to_digit(10) {
                Some(x) => res = res + ModIntP32::from(x),
                None => return Err(())
            }
        }
        if neg {
            res = -res;
        }
        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MersP31 {
    x: u32
}
impl MersP31 {
    const MOD: u32 = 2147483647u32;
    #[inline]
    fn reduced(self) -> MersP31 {
        // According to Godbolt this should emit a CMOV
        Self {
            x: if self.x >= MersP31::MOD {
                self.x - MersP31::MOD
            } else {
                self.x
            }
        }
    }
    #[inline]
     fn inv(self) -> Self {
        let mut t = (0u32, 1u32);
        let mut r = (MersP31::MOD, self.x);
        while r.1 != 0 {
            let q = r.0 / r.1;
            t = (t.1, t.0.wrapping_sub(q * t.1));
            r = (r.1, r.0 - q * r.1);
        }
        Self { x: if t.0 >= MersP31::MOD { t.0.wrapping_add(MersP31::MOD) } else { t.0 } }
    }
}
impl Add for MersP31 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        (Self { x: self.x + other.x }).reduced()
    }
}
impl Sub for MersP31 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}
impl Neg for MersP31 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        (Self { x: MersP31::MOD - self.x }).reduced()
    }
}
impl Mul for MersP31 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let (r, k) = self.x.widening_mul(other.x);
        let res = (r & MersP31::MOD) + (r >> 31) + (k << 1);
        (Self { x: res }).reduced()
    }
}
impl Div for MersP31 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl From<u32> for MersP31 {
    #[inline]
    fn from(x: u32) -> MersP31 {
        let res = (x & MersP31::MOD) + (x >> 31);
        (Self { x: res}).reduced()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MersP61 {
    x: u64
}
impl MersP61 {
    const MOD: u64 = 2305843009213693951u64;
    #[inline]
    fn reduced(self) -> MersP61 {
        Self {
            x: if self.x >= MersP61::MOD {
                self.x - MersP61::MOD
            } else {
                self.x
            }
        }
    }
    #[inline]
    fn inv(self) -> Self {
        let mut t = (0u64, 1u64);
        let mut r = (MersP61::MOD, self.x);
        while r.1 != 0 {
            let q = r.0 / r.1;
            t = (t.1, t.0.wrapping_sub(q * t.1));
            r = (r.1, r.0 - q * r.1);
        }
        Self { x: if t.0 >= MersP61::MOD { t.0.wrapping_add(MersP61::MOD) } else { t.0 } }
    }
}
impl Add for MersP61 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        (Self { x: self.x + other.x }).reduced()
    }
}
impl Sub for MersP61 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}
impl Neg for MersP61 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        (Self { x: MersP61::MOD - self.x }).reduced()
    }
}
impl Mul for MersP61 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let (r, k) = self.x.widening_mul(other.x);
        let res = (r & MersP61::MOD) + (r >> 61) + (k << 3);
        (Self { x: res }).reduced()
    }
}
impl Div for MersP61 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl From<u32> for MersP61 {
    #[inline]
    fn from(x: u32) -> MersP61 {
        Self::from(x as u64)
    }
}
impl From<u64> for MersP61 {
    #[inline]
    fn from(x: u64) -> MersP61 {
        let xp = x + 1;
        let z = ((xp >> 61) + xp) >> 61;
        MersP61 {
            x: (x + z) & MersP61::MOD
        }
    }
}
