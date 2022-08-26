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
    fn set_accuracy(&mut self, acc: usize) {
        self.expand_to(acc);
        self.limit_accuracy(acc);
    }

    #[inline]
    fn zeroes(acc: usize) -> Self {
        let mut res = Self::zero();
        res.set_accuracy(acc);
        res
    }

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
    fn identity(acc: usize) -> Self {
        let mut res = Self::zeroes(acc);
        res[1] = Self::Coeff::one();
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
        res.set_accuracy(self.accuracy());
        let mut b = if p < 0 {
            res.clone() / self
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
        r.set_accuracy(self.accuracy());
        for _i in 0..self.accuracy() {
            let mut r2 = Self::promote(Self::Coeff::from(q));
            r2.set_accuracy(self.accuracy());
            let q = (self.clone() - r.clone() * &r).tail_term() / (r2 * &r).tail_term();
            if q.is_zero() {
                return r;
            }
            r += q;
        }
        r.pow(p)
    }

    // Compositional inverse, multiplicative inverse is done through Div
    #[inline]
    fn inverse(&self) -> Self {
        assert!(self[0].is_zero());
        let mut sm = Self::zeroes(self.accuracy());
        let mut res = sm.clone();
        res[1] = Self::Coeff::one() / &self[1];
        let mut fpow = self.clone();
        for i in 0..self.accuracy() {
            sm[i] += res[1].clone() * &fpow[i];
        }
        for i in 2..self.accuracy() {
            fpow *= self;
            res[i] = -sm[i].clone() / &fpow[i];
            for j in 0..self.accuracy() {
                sm[j] += res[i].clone() * &fpow[j];
            }
        }
        res
    }

    #[inline]
    fn compose(&self, other: &Self) -> Self {
        assert!(other[0].is_zero());
        let sig = min(self.accuracy(), other.accuracy());
        let mut res = Self::zeroes(sig);
        for i in (0..sig).rev() {
            res *= other;
            res[0] += &self[i];
        }
        res
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
        let lap1 = self.laplace_inv();
        let lap2 = other.laplace_inv();
        (lap1 * lap2).laplace()
    }

    #[inline]
    fn dirichlet(&self, other: &Self) -> Self {
        let acc = min(self.accuracy(), other.accuracy());
        let mut res = Self::zeroes(acc);
        for i in 1..acc {
            for j in 1..acc/i {
                res[i * j] += self[j].clone() * &other[i];
            }
        }
        res
    }

    // Known series
    
    #[inline]
    fn sin(acc: usize) -> Self {
        let mut res = Self::identity(acc);
        for i in (3..acc).step_by(2) {
            res[i] = -res[i - 2].clone() / Self::Coeff::from(i as u32) / Self::Coeff::from((i - 1) as u32);
        }
        res
    }

    #[inline]
    fn cos(acc: usize) -> Self {
        let mut res = Self::one();
        res.set_accuracy(acc);
        for i in (2..acc).step_by(2) {
            res[i] = -res[i - 2].clone() / Self::Coeff::from(i as u32) / Self::Coeff::from((i - 1) as u32);
        }
        res
    }

    #[inline]
    fn expx(acc: usize) -> Self {
        let mut res = Self::one();
        res.set_accuracy(acc);
        for i in 1..acc {
            res[i] = res[i - 1].clone() / Self::Coeff::from(i as u32);
        }
        res
    }

    // log(1 + x)
    #[inline]
    fn log1px(acc: usize) -> Self {
        let mut res = Self::identity(acc);
        for i in 2..acc {
            let denom = if i % 2 == 0 { -Self::Coeff::one() } else { Self::Coeff::one() };
            res[i] = denom / Self::Coeff::from(i as u32);
        }
        res
    }


    // Transforms
    
    #[inline]
    fn partial_sums(&self) -> Self {
        let mut res = Self::zeroes(self.accuracy());
        let mut sm = Self::Coeff::zero();
        for i in 0..res.accuracy() {
            sm += &self[i];
            res[i] = sm.clone();
        }
        res
    }

    #[inline]
    fn partial_products(&self) -> Self {
        let mut res = Self::zeroes(self.accuracy());
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
        let mut res = Self::zeroes(self.accuracy());
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
        let mut res = Self::zeroes(self.accuracy());
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
        let mut one = Self::one();
        one.set_accuracy(self.accuracy());
        let updown = (one + Self::sin(n)) / Self::cos(n);
        (self.laplace_inv() * updown).laplace()
    }

    #[inline]
    fn bous_inv(&self) -> Self {
        let n = self.accuracy();
        let mut one = Self::one();
        one.set_accuracy(self.accuracy());
        let updown = (one + Self::sin(n)) / Self::cos(n);
        (self.laplace() / updown).laplace_inv()
    }

    #[inline]
    fn mobius(&self) -> Self {
        let mut mob = vec![Self::Coeff::zero(); self.accuracy()];
        mob[1] = Self::Coeff::one();
        for i in 1..self.accuracy() {
            let lst = mob[i].clone();
            for j in 2..(self.accuracy()+i-1)/i {
                mob[i * j] -= &lst;
            }
        }
        let mut res = Self::zeroes(self.accuracy());
        for i in 1..self.accuracy() {
            for j in 1..(self.accuracy()+i-1)/i {
                res[i * j] += mob[i].clone() * &self[j];
            }
        }
        res
    }

    #[inline]
    fn mobius_inv(&self) -> Self {
        let mut res = Self::zeroes(self.accuracy());
        for i in 1..self.accuracy() {
            for j in 1..(self.accuracy()+i-1)/i {
                res[i * j] += &self[j];
            }
        }
        res
    }

    #[inline]
    fn stirling(&self) -> Self {
        let mut stirl = vec![vec![Self::Coeff::zero(); self.accuracy()]; self.accuracy()];
        stirl[0][0] = Self::Coeff::one();
        for i in 1..self.accuracy() {
            for j in 1..i+1 {
                let nw = Self::Coeff::from(j as u32) * &stirl[i - 1][j] + &stirl[i - 1][j - 1];
                stirl[i][j] += nw;
            }
        }
        let mut res = Self::zeroes(self.accuracy());
        for i in 0..self.accuracy() {
            for j in 0..i+1 {
                res[i] += stirl[i][j].clone() * &self[j];
            }
        }
        res
    }

    #[inline]
    fn stirling_inv(&self) -> Self {
        let mut stirl = vec![vec![Self::Coeff::zero(); self.accuracy()]; self.accuracy()];
        stirl[0][0] = Self::Coeff::one();
        for i in 1..self.accuracy() {
            for j in 1..i+1 {
                let nw = -Self::Coeff::from((i-1) as u32) * &stirl[i - 1][j] + &stirl[i - 1][j - 1];
                stirl[i][j] += nw;
            }
        }
        let mut res = Self::zeroes(self.accuracy());
        for i in 0..self.accuracy() {
            for j in 0..i+1 {
                res[i] += stirl[i][j].clone() * &self[j];
            }
        }
        res
    }

    #[inline]
    fn euler(&self) -> Self {
        let da = self.point();
        let c = da.mobius_inv();
        let mut res = Self::zeroes(self.accuracy());
        res[0] += &self[0];
        for i in 1..self.accuracy() {
            res[i] += &c[i];
            for j in 1..i {
                let cpy = res[i - j].clone();
                res[i] += cpy * &c[j];
            }
            res[i] /= Self::Coeff::from(i as u32);
        }
        res
    }

    #[inline]
    fn euler_inv(&self) -> Self {
        let mut res = Self::zeroes(self.accuracy());
        for i in 1..self.accuracy() {
            res[i] = Self::Coeff::from(i as u32) * &self[i];
            for j in 1..i {
                let cpy = res[j].clone();
                res[i] -= cpy * &self[i - j];
            }
        }
        res = res.mobius();
        res[0] += &self[0];
        for i in 1..self.accuracy() {
            res[i] /= Self::Coeff::from(i as u32);
        }
        res
    }

    #[inline]
    fn lah(&self) -> Self {
        let mut fac = vec![];
        let mut mul = Self::Coeff::one();
        fac.push(mul.clone());
        for i in 1..self.accuracy() {
            mul *= Self::Coeff::from(i as u32);
            fac.push(mul.clone());
        }
        let mut res = Self::zeroes(self.accuracy());
        res[0] = self[0].clone();
        for i in 1..self.accuracy() {
            for j in 1..i+1 {
                let coeff = fac[i].clone() * &fac[i - 1] / &fac[j] / &fac[j - 1] / &fac[i - j];
                res[i] += coeff * &self[j];
            }
        }
        res
    }

    #[inline]
    fn lah_inv(&self) -> Self {
        let mut fac = vec![];
        let mut mul = Self::Coeff::one();
        fac.push(mul.clone());
        for i in 1..self.accuracy() {
            mul *= Self::Coeff::from(i as u32);
            fac.push(mul.clone());
        }
        let mut res = Self::zeroes(self.accuracy());
        res[0] = self[0].clone();
        for i in 1..self.accuracy() {
            for j in 1..i+1 {
                let coeff = fac[i].clone() * &fac[i - 1] / &fac[j] / &fac[j - 1] / &fac[i - j];
                if (i - j) % 2 == 0 {
                    res[i] += coeff * &self[j];
                } else {
                    res[i] -= coeff * &self[j];
                }
            }
        }
        res
    }

    #[inline]
    fn powerset(&self) -> Self {
        let mut res = Self::zero();
        res.set_accuracy(self.accuracy());
        for i in 1..self.accuracy() {
            for j in 1..self.accuracy()/i+1 {
                let coeff = self[i].clone() / Self::Coeff::from(j as u32);
                if j % 2 == 1 {
                    res[i * j] += coeff;
                } else {
                    res[i * j] -= coeff;
                }
            }
        }
        res.exp()
    }
}


#[cfg(test)]
mod tests {
    use crate::PowerSeries;
    use crate::Series;

    #[test]
    fn test_mobius() {
        let inp: Series = "0,1,3,4,7,6,12,8,15,13,18,12,28,14,24,24".parse().unwrap();
        let ans: Series = "0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15".parse().unwrap();
        assert_eq!(inp.mobius(), ans);
        assert_eq!(inp, ans.mobius_inv());
        assert_eq!(inp, inp.mobius().mobius_inv());
        assert_eq!(inp, inp.mobius_inv().mobius());
    }

    #[test]
    fn test_stirling() {
        let inp: Series = "0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15".parse().unwrap();
        let ans: Series = "0,1,3,10,37,151,674,3263,17007,94828,562595,3535027,23430840,163254885,1192059223,9097183602".parse().unwrap();
        assert_eq!(inp.stirling(), ans);
        assert_eq!(ans.stirling_inv(), inp);
        assert_eq!(inp, inp.stirling().stirling_inv());
        assert_eq!(inp, inp.stirling_inv().stirling());
    }

    #[test]
    fn test_lah() {
        let inp: Series = "0,1,4,9,16,25,36,49,64,81,100,121,144,169,196,225".parse().unwrap();
        let ans: Series = "0,1,6,39,292,2505,24306,263431,3154824,41368977,589410910,9064804551,149641946796,2638693215769,49490245341642,983607047803815".parse().unwrap();
        assert_eq!(inp.lah(), ans);
        assert_eq!(ans.lah_inv(), inp);
        assert_eq!(inp, inp.lah().lah_inv());
        assert_eq!(inp, inp.lah_inv().lah());
    }

    #[test]
    fn test_euler() {
        let conn: Series = "1,1,1,2,6,20,99,646,5974,71885,1052805,17449299,313372298".parse().unwrap();
        let notc: Series = "1,1,2,4,11,33,142,822,6966,79853,1140916,18681008,333312451".parse().unwrap();
        assert_eq!(conn.euler(), notc);
        assert_eq!(conn, notc.euler_inv());
        assert_eq!(conn.euler().euler_inv(), conn);
        assert_eq!(conn.euler_inv().euler(), conn);
    }
}
