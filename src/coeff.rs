use std::ops::{Add, Sub, Mul, Neg, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::mathtypes::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
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
    #[inline]
    fn zero() -> ModIntP32 {
        ModIntP32 {
            x: 0
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.x == 0
    }
}

impl One for ModIntP32 {
    #[inline]
    fn one() -> ModIntP32 {
        ModIntP32 {
            x: 1
        }
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.x == 1
    }
}

impl Add<ModIntP32> for ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn add(self, other: ModIntP32) -> ModIntP32 {
        let res = self.x + other.x;
        ModIntP32 {
            x: if res >= ModIntP32::MOD { res - ModIntP32::MOD } else { res }
        }
    }
}

impl AddAssign<ModIntP32> for ModIntP32 {
    #[inline]
    fn add_assign(&mut self, other: ModIntP32) {
        self.x += other.x;
        if self.x >= ModIntP32::MOD {
            self.x -= ModIntP32::MOD;
        }
    }
}

impl Sub<ModIntP32> for ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn sub(self, other: ModIntP32) -> ModIntP32 {
        let res = ModIntP32::MOD + self.x - other.x;
        ModIntP32 {
            x: if res >= ModIntP32::MOD { res - ModIntP32::MOD } else { res }
        }
    }
}

impl SubAssign<ModIntP32> for ModIntP32 {
    #[inline]
    fn sub_assign(&mut self, other: ModIntP32) {
        self.x += ModIntP32::MOD - other.x;
        if self.x >= ModIntP32::MOD {
            self.x -= ModIntP32::MOD;
        }
    }
}

impl Neg for ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn neg(self) -> ModIntP32 {
        ModIntP32 {
            x: ModIntP32::MOD - self.x
        }
    }
}

impl Mul<ModIntP32> for ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn mul(self, other: ModIntP32) -> ModIntP32 {
        ModIntP32 {
            x: (self.x * other.x) % ModIntP32::MOD
        }
    }
}

impl MulAssign<ModIntP32> for ModIntP32 {
    #[inline]
    fn mul_assign(&mut self, other: ModIntP32) {
        self.x *= other.x;
        self.x %= ModIntP32::MOD;
    }
}

impl Div<ModIntP32> for ModIntP32 {
    type Output = ModIntP32;

    #[inline]
    fn div(self, other: ModIntP32) -> ModIntP32 {
        self * other.inv()
    }
}

impl DivAssign<ModIntP32> for ModIntP32 {
    #[inline]
    fn div_assign(&mut self, other: ModIntP32) {
        *self *= other.inv();
    }
}

impl From<u32> for ModIntP32 {
    #[inline]
    fn from(x: u32) -> ModIntP32 {
        ModIntP32 {
            x: if x < ModIntP32::MOD { x } else { x % ModIntP32::MOD }
        }
    }
}

impl std::str::FromStr for ModIntP32 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let mut res = ModIntP32::from(0u32);
        let mut neg = false;
        for (i, c) in s.chars().enumerate() {
            if i == 0 && c == '-' {
                neg = true;
                continue;
            }
            res *= ModIntP32::from(10u32);
            match c.to_digit(10) {
                Some(x) => res += ModIntP32::from(x),
                None => return Err(())
            }
        }
        if neg {
            res = -res;
        }
        Ok(res)
    }
}

forward_into_ref_field! { impl Field for ModIntP32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MersP31 {
    x: u32
}
impl MersP31 {
    const MOD: u32 = 2147483647u32;
    #[inline]
    fn reduce(&mut self) {
        if self.x >= MersP31::MOD {
            self.x -= MersP31::MOD;
        }
    }
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

impl Zero for MersP31 {
    #[inline]
    fn zero() -> MersP31 {
        MersP31 {
            x: 0
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.x == 0
    }
}

impl One for MersP31 {
    #[inline]
    fn one() -> MersP31 {
        MersP31 {
            x: 1
        }
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.x == 1
    }
}

impl Add<MersP31> for MersP31 {
    type Output = MersP31;

    #[inline]
    fn add(self, other: MersP31) -> MersP31 {
        (MersP31 { x: self.x + other.x }).reduced()
    }
}

impl AddAssign<MersP31> for MersP31 {
    #[inline]
    fn add_assign(&mut self, other: MersP31) {
        self.x += other.x;
        self.reduce();
    }
}

impl Sub<MersP31> for MersP31 {
    type Output = MersP31;

    #[inline]
    fn sub(self, other: MersP31) -> MersP31 {
        self + other.neg()
    }
}

impl SubAssign<MersP31> for MersP31 {
    #[inline]
    fn sub_assign(&mut self, other: MersP31) {
        *self += other.neg();
    }
}

impl Neg for MersP31 {
    type Output = MersP31;

    #[inline]
    fn neg(self) -> MersP31 {
        (MersP31 { x: MersP31::MOD - self.x }).reduced()
    }
}

impl Mul<MersP31> for MersP31 {
    type Output = MersP31;

    #[inline]
    fn mul(self, other: MersP31) -> MersP31 {
        let (r, k) = self.x.widening_mul(other.x);
        let res = (r & MersP31::MOD) + (r >> 31) + (k << 1);
        (MersP31 { x: res }).reduced()
    }
}

impl MulAssign<MersP31> for MersP31 {
    #[inline]
    fn mul_assign(&mut self, other: MersP31) {
        *self = *self * other;
    }
}

impl Div<MersP31> for MersP31 {
    type Output = MersP31;

    #[inline]
    fn div(self, other: MersP31) -> MersP31 {
        self * other.inv()
    }
}

impl DivAssign<MersP31> for MersP31 {
    #[inline]
    fn div_assign(&mut self, other: MersP31) {
        *self *= other.inv();
    }
}

impl From<u32> for MersP31 {
    #[inline]
    fn from(x: u32) -> MersP31 {
        let res = (x & MersP31::MOD) + (x >> 31);
        (Self { x: res}).reduced()
    }
}

forward_into_ref_field! { impl Field for MersP31 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MersP61 {
    x: u64
}
impl MersP61 {
    const MOD: u64 = 2305843009213693951u64;
    #[inline]
    fn reduce(&mut self) {
        if self.x >= MersP61::MOD {
            self.x -= MersP61::MOD;
        }
    }
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

impl Zero for MersP61 {
    #[inline]
    fn zero() -> MersP61 {
        MersP61 {
            x: 0
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.x == 0
    }
}

impl One for MersP61 {
    #[inline]
    fn one() -> MersP61 {
        MersP61 {
            x: 1
        }
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.x == 1
    }
}

impl Add<MersP61> for MersP61 {
    type Output = MersP61;

    #[inline]
    fn add(self, other: MersP61) -> MersP61 {
        (MersP61 { x: self.x + other.x }).reduced()
    }
}

impl AddAssign<MersP61> for MersP61 {
    #[inline]
    fn add_assign(&mut self, other: MersP61) {
        self.x += other.x;
        self.reduce();
    }
}

impl Sub<MersP61> for MersP61 {
    type Output = MersP61;

    #[inline]
    fn sub(self, other: MersP61) -> MersP61 {
        self + other.neg()
    }
}

impl SubAssign<MersP61> for MersP61 {
    #[inline]
    fn sub_assign(&mut self, other: MersP61) {
        *self += other.neg();
    }
}

impl Neg for MersP61 {
    type Output = MersP61;

    #[inline]
    fn neg(self) -> MersP61 {
        (MersP61 { x: MersP61::MOD - self.x }).reduced()
    }
}

impl Mul<MersP61> for MersP61 {
    type Output = MersP61;

    #[inline]
    fn mul(self, other: MersP61) -> MersP61 {
        let (r, k) = self.x.widening_mul(other.x);
        let res = (r & MersP61::MOD) + (r >> 61) + (k << 3);
        (MersP61 { x: res }).reduced()
    }
}

impl MulAssign<MersP61> for MersP61 {
    #[inline]
    fn mul_assign(&mut self, other: MersP61) {
        *self = *self * other;
    }
}

impl Div<MersP61> for MersP61 {
    type Output = MersP61;

    #[inline]
    fn div(self, other: MersP61) -> MersP61 {
        self * other.inv()
    }
}

impl DivAssign<MersP61> for MersP61 {
    #[inline]
    fn div_assign(&mut self, other: MersP61) {
        *self *= other.inv();
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

forward_into_ref_field! { impl Field for MersP61 }
