use crate::Field;
use crate::mathtypes::{Zero, One};
use std::ops::IndexMut;
use std::cmp::min;

pub trait PowerSeries: IndexMut<usize, Output = Self::Coeff> + Field {
    type Coeff: Field;

    fn accuracy(&self) -> usize;
    fn nonzero_num(&self) -> usize;
    fn expand_to(&mut self, l: usize);
    fn limit_accuracy(&mut self, l: usize);
    // Tail operation
    fn lshift(&self) -> Self;
    // Multiply by x
    fn rshift(&self) -> Self;


    #[inline]
    fn promote(coeff: Self::Coeff) -> Self {
        let mut res = Self::zero();
        res.expand_to(1);
        res[0] = coeff;
        res
    }

    #[inline]
    fn identity() -> Self {
        let mut res = Self::zero();
        res.expand_to(1);
        res[0] = Self::Coeff::one();
        res
    }

    #[inline]
    fn derive(&self) -> Self {
        let mut res = self.lshift();
        for i in 0..res.nonzero_num() {
            res[i] *= Self::Coeff::from((i+1) as u32);
        }
        res
    }


    #[inline]
    fn integrate(&self) -> Self {
        let mut res = self.rshift();
        for i in 1..self.nonzero_num()+1 {
            res[i] /= Self::Coeff::from(i as u32);
        }
        res
    }

    #[inline]
    fn hadamard(&self, other: &Self) -> Self {
        let mut res = self.clone();
        let len = min(self.nonzero_num(), other.nonzero_num());
        for i in 0..len {
            res[i] *= &other[i];
        }
        res.limit_accuracy(len);
        res
    }

    #[inline]
    fn pow(&self, p: i32) -> Self {
        let mut res = Self::one();
        let mut b = if p < 0 {
            Self::one() / self
        } else {
            self.clone()
        };
        let mut e = p.abs() as u32;
        while e > 0 {
            if e & 1 > 0 {
                res *= &b;
            }
            b *= b.clone();
            e /= 2;
        }
        res
    }

    #[inline]
    fn ratpow(&self, p: i32, q: u32) -> Self {
        // for now, check for x^q and such later
        assert!(self[0].is_one());
        let mut r = Self::one();
        for _i in 0..self.nonzero_num() {
            let q = (self.clone() - r.clone() * &r).tail_term() / (Self::promote(Self::Coeff::from(q)) * &r).tail_term();
            if q.is_zero() {
                return r;
            }
            r += q;
        }
        r.pow(p)
    }

    #[inline]
    // Compositional inverse, multiplicative inverse is done through Div
    fn inverse(&self) -> Self {
        assert!(self[0].is_zero());
        let mut r = Self::zero();
        let comp = self.lshift();
        r.limit_accuracy(self.accuracy());
        for _i in 0..self.nonzero_num() {
            r = (Self::one() / comp.compose(&r)).rshift();
        }
        r
    }

    #[inline]
    fn compose(&self, other: &Self) -> Self {
        assert!(other[0].is_zero());
        if self.is_constant() { return self.clone(); }
        let reccomp = self.lshift().compose(other);
        let mut tail = (other.lshift() * reccomp).rshift();
        tail[0] += self[0].clone();
        tail
    }

    #[inline]
    fn sqrt(&self) -> Self {
        self.ratpow(1, 2)
    }

    #[inline]
    fn is_constant(&self) -> bool {
        self.lshift().is_zero()
    }

    #[inline]
    fn tail_term(self) -> Self {
        let mut found = false;
        let mut res = self.clone();
        for i in 0..self.accuracy() {
            if res[i].is_zero() {
                continue;
            }
            if found {
                res[i] = Self::Coeff::zero();
            } else {
                found = true;
            }
        }
        res
    }
}

