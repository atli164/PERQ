use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::{Field, PowerSeries};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShortSeq<T: Field + Copy> {
    pub seq: [T; 16],
    pub cnt: u8
}

impl<T: Field + Copy> Default for ShortSeq<T> {
    #[inline]
    fn default() -> Self {
        Self {
            seq: Default::default(),
            cnt: 16
        }
    }
}

impl<T: Field + Copy> From<u32> for ShortSeq<T> {
    #[inline]
    fn from(x: u32) -> ShortSeq<T> {
        ShortSeq::promote(T::from(x))
    }
}

impl<T: Field + Copy> Add for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        // This seems to get automatically unrolled
        // Spelling it out explicitly makes no performance difference
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] + other.seq[i]
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy> Neg for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = -self.seq[i]
        }
        Self {
            seq: seq,
            cnt: self.cnt
        }
    }
}

impl<T: Field + Copy> Sub for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] - other.seq[i]
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}


impl<T: Field + Copy> Mul for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for i in 0..16 {
            for j in 0..16-i {
                seq[i + j] = seq[i + j] + (self.seq[i] * other.seq[j]);
            }
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy> Div for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let mut seq: [T; 16] = self.seq.clone();
        for i in 0..16 {
            for j in 0..i {
                seq[i] = seq[i] - other.seq[j] * seq[i - j - 1];
            }
            seq[i] = seq[i] / other.seq[0];
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy> PowerSeries for ShortSeq<T> {
    type Coeff = T;

    #[inline]
    fn promote(x: T) -> Self {
        let mut res: Self = Default::default();
        res.seq[0] = x;
        res
    }

    #[inline]
    fn coefficient(self, i: usize) -> Self::Coeff {
        self.seq[i]
    }

    #[inline]
    fn identity() -> Self {
        let mut res: ShortSeq<T> = Default::default();
        res.seq[1] = T::from(1);
        res
    }

    #[inline]
    fn derive(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        // This seems to get automatically unrolled
        // Spelling it out explicitly makes no performance difference
        for i in 1..16 {
            seq[i - 1] = self.seq[i] * T::from(i as u32);
        }
        Self {
            seq: seq,
            cnt: self.cnt.saturating_sub(1)
        }
    }

    #[inline]
    fn integrate(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        // This seems to get automatically unrolled
        // Spelling it out explicitly makes no performance difference
        for i in 1..16 {
            seq[i] = self.seq[i - 1] / T::from(i as u32);
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(16, self.cnt + 1)
        }
    }

    #[inline]
    fn compose(self, _other: Self) -> Self {
        // TODO
        Self {
            seq: Default::default(),
            cnt: self.cnt
        }
    }

    #[inline]
    fn inverse(self) -> Self {
        // TODO
        Self {
            seq: Default::default(),
            cnt: self.cnt
        }
    }
}
