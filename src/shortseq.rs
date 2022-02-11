use std::ops::{Add, Sub, Mul};
use crate::{Ring};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ShortSeq<T: Ring + Copy> {
    pub seq: [T; 16],
}

impl<T: Ring + Copy> Add for ShortSeq<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] + other.seq[i]
        }
        Self {
            seq
        }
    }
}

impl<T: Ring + Copy> Sub for ShortSeq<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] - other.seq[i]
        }
        Self {
            seq
        }
    }
}


impl<T: Ring + Copy> Mul for ShortSeq<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for i in 0..16 {
            for j in 0..16-i {
                seq[i + j] = seq[i + j] + (self.seq[i] * other.seq[j]);
            }
        }
        Self {
            seq
        }
    }
}
