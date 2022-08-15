use crate::Field;
use crate::mathtypes::{Zero, One};
use std::ops::IndexMut;
use std::cmp::min;

pub trait PowerSeries: IndexMut<usize, Output = Self::Coeff> + FromIterator<Self::Coeff> + Field {
    type Coeff: Field;

    fn accuracy(&self) -> usize;
    fn expand_to(&mut self, l: usize);
    fn limit_accuracy(&mut self, l: usize);
    // Tail operation
    fn lshift(&self) -> Self;
    // Multiply by x
    fn rshift(&self) -> Self;

    #[inline]
    fn matches(&self, other: &Self) -> bool {
        let acc = min(self.accuracy(), other.accuracy());
        (0..acc).all(|i| self[i] == other[i])
    }

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
        for i in 0..res.accuracy() {
            res[i] *= Self::Coeff::from((i+1) as u32);
        }
        res
    }

    #[inline]
    fn point(&self) -> Self {
        let mut res = self.clone();
        for i in 0..res.accuracy() {
            res[i] *= Self::Coeff::from(i as u32);
        }
        res
    }

    #[inline]
    fn log_derive(&self) -> Self {
        self.derive() / self
    }


    #[inline]
    fn integrate(&self) -> Self {
        let mut res = self.rshift();
        for i in 1..self.accuracy() {
            res[i] /= Self::Coeff::from(i as u32);
        }
        res
    }

    #[inline]
    fn hadamard(&self, other: &Self) -> Self {
        let mut res = self.clone();
        let len = min(self.accuracy(), other.accuracy());
        for i in 0..len {
            res[i] *= &other[i];
        }
        res.limit_accuracy(len);
        res
    }

    #[inline]
    fn pow(&self, p: i32) -> Self {
        let mut res = Self::one();
        res.expand_to(self.accuracy());
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
        r.expand_to(self.accuracy());
        for _i in 0..self.accuracy() {
            let mut r2 = Self::promote(Self::Coeff::from(q));
            r2.expand_to(self.accuracy());
            let q = (self.clone() - r.clone() * &r).tail_term() / (r2 * &r).tail_term();
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
        r.expand_to(self.accuracy());
        let comp = self.lshift();
        for _i in 0..self.accuracy() {
            let mut one = Self::one();
            one.expand_to(self.accuracy());
            r = (one / comp.compose(&r)).rshift();
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

    #[inline]
    fn exp_mul(&self, other: &Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn dirichlet(&self, other: &Self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn comp_sqrt(&self) -> Self {
        unimplemented!()
    }

    // Known series
    
    #[inline]
    fn sin(acc: usize) -> Self {
        let mut res = Self::identity();
        res.expand_to(acc);
        res.limit_accuracy(acc);
        for i in (3..acc).step_by(2) {
            res[i] = -res[i - 2].clone() / Self::Coeff::from(i as u32) / Self::Coeff::from((i - 1) as u32);
        }
        res
    }

    #[inline]
    fn cos(acc: usize) -> Self {
        let mut res = Self::one();
        res.expand_to(acc);
        res.limit_accuracy(acc);
        for i in (3..acc).step_by(2) {
            res[i] = -res[i - 2].clone() / Self::Coeff::from(i as u32) / Self::Coeff::from((i - 1) as u32);
        }
        res
    }

    #[inline]
    fn expx(acc: usize) -> Self {
        let mut res = Self::one();
        res.expand_to(acc);
        res.limit_accuracy(acc);
        for i in 1..acc {
            res[i] = res[i - 1].clone() / Self::Coeff::from(i as u32);
        }
        res
    }

    #[inline]
    // log(1 + x)
    fn log1px(acc: usize) -> Self {
        let mut res = Self::identity();
        res.expand_to(acc);
        res.limit_accuracy(acc);
        for i in 2..acc {
            let denom = if i % 2 == 0 { -Self::Coeff::one() } else { Self::Coeff::one() };
            res[i] = denom / Self::Coeff::from(i as u32);
        }
        res
    }


    // Transforms
    
    #[inline]
    fn partial_sums(&self) -> Self {
        let mut res = Self::zero();
        res.expand_to(self.accuracy());
        let mut sm = Self::Coeff::zero();
        for i in 0..res.accuracy() {
            sm += &self[i];
            res[i] = sm.clone();
        }
        res
    }

    #[inline]
    fn partial_products(&self) -> Self {
        let mut res = Self::zero();
        res.expand_to(self.accuracy());
        let mut pr = Self::Coeff::one();
        for i in 0..res.accuracy() {
            pr *= &self[i];
            res[i] = pr.clone();
        }
        res
    }

    #[inline]
    fn delta(&self) -> Self {
        let mut res = self.lshift();
        for i in 0..res.accuracy() {
            res[i] -= &self[i];
        }
        res
    }

    #[inline]
    fn binomial(&self) -> Self {
        // compositional definition much slower in practice
        // calculating binomial coefficients is also slower
        let mut res = Self::zero();
        res.expand_to(self.accuracy());
        let mut summer = self.clone();
        res[0] = summer[0].clone();
        for i in 1..self.accuracy() {
            for j in 0..summer.accuracy()-1 {
                let nxt = summer[j + 1].clone();
                summer[j] += nxt;
            }
            summer.limit_accuracy(summer.accuracy() - 1);
            res[i] = summer[0].clone();
        }
        res
    }

    #[inline]
    fn binomial_inv(&self) -> Self {
        let mut res = Self::zero();
        res.expand_to(self.accuracy());
        let mut summer = self.clone();
        res[0] = summer[0].clone();
        for i in 1..self.accuracy() {
            for j in 0..summer.accuracy()-1 {
                let nxt = summer[j + 1].clone();
                summer[j] = nxt - &summer[j];
            }
            summer.limit_accuracy(summer.accuracy() - 1);
            res[i] = summer[0].clone();
        }
        res
    }

    #[inline]
    fn t019(&self) -> Self {
        let mut res = self.lshift().lshift();
        for i in 0..res.accuracy() {
            res[i] += &self[i];
        }
        for i in 0..res.accuracy() {
            res[i] -= &self[i + 1];
            res[i] -= &self[i + 1];
        }
        res
    }

    #[inline]
    fn exp(&self) -> Self {
        let mut res = Self::expx(self.accuracy()).compose(self);
        res[0] -= Self::Coeff::one();
        res
    }

    #[inline]
    fn log(&self) -> Self {
        Self::log1px(self.accuracy()).compose(self)
    }
    
    #[inline]
    fn laplace(&self) -> Self {
        let mut fac = Self::Coeff::one();
        let mut res = self.clone();
        for i in 2..self.accuracy() {
            fac *= Self::Coeff::from(i as u32);
            res[i] *= &fac;
        }
        res
    }

    #[inline]
    fn laplace_inv(&self) -> Self {
        let mut fac = Self::Coeff::one();
        let mut res = self.clone();
        for i in 2..self.accuracy() {
            fac *= Self::Coeff::from(i as u32);
            res[i] /= &fac;
        }
        res
    }

    #[inline]
    fn bous(&self) -> Self {
        let n = self.accuracy();
        let updown = (Self::one() + Self::sin(n)) / Self::cos(n);
        (self.laplace_inv() * updown).laplace()
    }

    #[inline]
    fn bous_inv(&self) -> Self {
        let n = self.accuracy();
        let updown = (Self::one() + Self::sin(n)) / Self::cos(n);
        (self.laplace() / updown).laplace_inv()
    }

    #[inline]
    fn mobius(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn mobius_inv(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn weigh(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn stirling(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn stirling_inv(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn partition(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn euler(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn euler_inv(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn multiset(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn powerset(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cycle(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn lah(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn lah_inv(&self) -> Self {
        unimplemented!()
    }

    #[inline]
    fn cameron(&self) -> Self {
        unimplemented!()
    }
}