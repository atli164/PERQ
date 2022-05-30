use std::ops::{Add, Sub, Mul, Neg, Div};

const MODP32: u32 = 65521;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ModIntP32 {
    x: u32
}
impl ModIntP32 {
    #[inline]
    pub fn sq(self) -> Self {
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
impl Add for ModIntP32 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        let res = self.x + other.x;
        Self {
            x: if res >= MODP32 { res - MODP32 } else { res }
        }
    }
}
impl Sub for ModIntP32 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        let res = MODP32 + self.x - other.x;
        Self {
            x: if res >= MODP32 { res - MODP32 } else { res }
        }
    }
}
impl Neg for ModIntP32 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: MODP32 - self.x
        }
    }
}
impl Mul for ModIntP32 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: (self.x * other.x) % MODP32
        }
    }
}
impl Div for ModIntP32 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl From<u32> for ModIntP32 {
    #[inline]
    fn from(x: u32) -> ModIntP32 {
        ModIntP32 {
            x: x % MODP32
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
pub struct MersP31B32 {
    x: u32
}
const MERSENNEVAL32: u32 = 2147483647u32;
impl MersP31B32 {
    #[inline]
    pub fn sq(self) -> Self {
        self * self
    }
    #[inline]
    fn reduced(self) -> MersP31B32 {
        // According to Godbolt this should emit a CMOV
        Self {
            x: if self.x >= MERSENNEVAL32 {
                self.x - MERSENNEVAL32
            } else {
                self.x
            }
        }
    }
    #[inline]
     fn inv(self) -> Self {
        let mut t = (0u32, 1u32);
        let mut r = (MERSENNEVAL32, self.x);
        while r.1 != 0 {
            let q = r.0 / r.1;
            t = (t.1, t.0.wrapping_sub(q * t.1));
            r = (r.1, r.0 - q * r.1);
        }
        Self { x: if t.0 >= MERSENNEVAL32 { t.0.wrapping_add(MERSENNEVAL32) } else { t.0 } }
    }
}
impl Add for MersP31B32 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        (Self { x: self.x + other.x }).reduced()
    }
}
impl Sub for MersP31B32 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}
impl Neg for MersP31B32 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        (Self { x: MERSENNEVAL32 - self.x }).reduced()
    }
}
impl Mul for MersP31B32 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let (r, k) = self.x.widening_mul(other.x);
        let res = (r & MERSENNEVAL32) + (r >> 31) + (k << 1);
        (Self { x: res }).reduced()
    }
}
impl Div for MersP31B32 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl From<u32> for MersP31B32 {
    #[inline]
    fn from(x: u32) -> MersP31B32 {
        let res = (x & MERSENNEVAL32) + (x >> 31);
        (Self { x: res}).reduced()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MersP61B64 {
    x: u64
}
const MERSENNEVAL64: u64 = 2305843009213693951u64;
impl MersP61B64 {
    #[inline]
    pub fn sq(self) -> Self {
        self * self
    }
    #[inline]
    fn reduced(self) -> MersP61B64 {
        Self {
            x: if self.x >= MERSENNEVAL64 {
                self.x - MERSENNEVAL64
            } else {
                self.x
            }
        }
    }
    #[inline]
    fn inv(self) -> Self {
        let mut t = (0u64, 1u64);
        let mut r = (MERSENNEVAL64, self.x);
        while r.1 != 0 {
            let q = r.0 / r.1;
            t = (t.1, t.0.wrapping_sub(q * t.1));
            r = (r.1, r.0 - q * r.1);
        }
        Self { x: if t.0 >= MERSENNEVAL64 { t.0.wrapping_add(MERSENNEVAL64) } else { t.0 } }
    }
}
impl Add for MersP61B64 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        (Self { x: self.x + other.x }).reduced()
    }
}
impl Sub for MersP61B64 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}
impl Neg for MersP61B64 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        (Self { x: MERSENNEVAL64 - self.x }).reduced()
    }
}
impl Mul for MersP61B64 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let (r, k) = self.x.widening_mul(other.x);
        let res = (r & MERSENNEVAL64) + (r >> 61) + (k << 3);
        (Self { x: res }).reduced()
    }
}
impl Div for MersP61B64 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl From<u32> for MersP61B64 {
    #[inline]
    fn from(x: u32) -> MersP61B64 {
        Self::from(x as u64)
    }
}
impl From<u64> for MersP61B64 {
    #[inline]
    fn from(x: u64) -> MersP61B64 {
        let xp = x + 1;
        let z = ((xp >> 61) + xp) >> 61;
        MersP61B64 {
            x: (x + z) & MERSENNEVAL64
        }
    }
}
