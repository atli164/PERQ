use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::iter::{zip, once};
use crate::{PowerSeries};
use crate::mathtypes::{Zero, One};
use rug::{Rational, Complete};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Series {
    pub seq: Vec<Rational>
}

impl Zero for Series {
    #[inline]
    fn zero() -> Self {
        Self {
            seq: vec![Rational::new()]
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        if self.seq.is_empty() {
            return false;
        }
        self.seq.iter().all(|x| x.cmp0() == std::cmp::Ordering::Equal)
    }
}

impl One for Series {
    #[inline]
    fn one() -> Self {
        Self {
            seq: vec![Rational::from(1)]
        }
    }
    #[inline]
    fn is_one(&self) -> bool {
        if self.seq.is_empty() {
            return false;
        }
        if self.seq[0].cmp(&1.into()) != std::cmp::Ordering::Equal {
            return false;
        }
        self.seq.iter().skip(1).all(|x| x.cmp0() == std::cmp::Ordering::Equal)
    }
}

impl From<u32> for Series {
    #[inline]
    fn from(x: u32) -> Series {
        Series::promote(Rational::from(x))
    }
}

impl<'a, 'b> Add<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn add(self, other: &'a Series) -> Series {
        Series {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(x, y)| x + y).map(|z| z.complete()).collect()
        }
    }
}

forward_binop_impl! { impl Add, add for Series }

impl<'a> AddAssign<&'a Series> for Series {
    #[inline]
    fn add_assign(&mut self, other: &'a Series) {
        zip(self.seq.iter_mut(), other.seq.iter()).for_each(|(x, y)| *x += y);
    }
}

forward_assign_impl! { impl AddAssign, add_assign for Series }

impl<'a> Neg for &'a Series {
    type Output = Series;

    #[inline]
    fn neg(self) -> Series {
        Series {
            seq: self.seq.iter().map(|x| (-x).complete()).collect()
        }
    }
}

forward_unop_impl! { impl Neg, neg for Series }

impl<'a, 'b> Sub<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn sub(self, other: &'a Series) -> Series {
        Series {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(x, y)| x - y).map(|z| z.complete()).collect()
        }
    }
}

forward_binop_impl! { impl Sub, sub for Series }

impl<'a> SubAssign<&'a Series> for Series {
    #[inline]
    fn sub_assign(&mut self, other: &'a Series) {
        zip(self.seq.iter_mut(), other.seq.iter()).for_each(|(x, y)| *x -= y);
    }
}

forward_assign_impl! { impl SubAssign, sub_assign for Series }

impl<'a, 'b> Mul<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn mul(self, other: &'a Series) -> Series {
        let n = std::cmp::min(self.seq.len(), other.seq.len());
        let mut seq = vec![Default::default(); n];
        for i in 0..n {
            for j in 0..n-i {
                let prod = (&self.seq[i] * &other.seq[j]).complete();
                seq[i + j] += prod;
            }
        }
        Series {
            seq: seq
        }
    }
}

forward_binop_impl! { impl Mul, mul for Series }

impl<'a> MulAssign<&'a Series> for Series {
    #[inline]
    fn mul_assign(&mut self, other: &'a Series) {
        *self = &*self * other;
    }
}

forward_assign_impl! { impl MulAssign, mul_assign for Series }

impl<'a> DivAssign<&'a Series> for Series {
    #[inline]
    fn div_assign(&mut self, other: &'a Series) {
        let n = std::cmp::min(self.seq.len(), other.seq.len());
        for i in 0..n {
            self.seq[i] /= &other.seq[0];
            for j in (i+1)..n {
                let prod = (&self.seq[i] * &other.seq[j - i]).complete();
                self.seq[j] -= prod;
            }
        }
    }
}

forward_assign_impl! { impl DivAssign, div_assign for Series }

impl<'a, 'b> Div<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn div(self, other: &'a Series) -> Series {
        let mut cpy = self.clone();
        cpy /= other;
        cpy
    }
}

forward_binop_impl! { impl Div, div for Series }

impl PowerSeries for Series {
    type Coeff = Rational;

    #[inline]
    fn promote(x: Rational) -> Self {
        let mut res: Self = Default::default();
        res.seq.push(x);
        res
    }

    #[inline]
    fn coefficient(&self, i: usize) -> Self::Coeff {
        self.seq[i].clone()
    }

    #[inline]
    fn identity() -> Self {
        let mut res: Self = Default::default();
        res.seq.push(Rational::from(1));
        res
    }

    #[inline]
    fn derive(&self) -> Self {
        Self {
            seq: self.seq.iter().enumerate().skip(1).map(|(i, x)| Rational::from(i as u32) * x).collect()
        }
    }

    #[inline]
    fn integrate(&self) -> Self {
        Self {
            seq: once(Default::default()).chain(self.seq.iter().enumerate().map(|(i, x)| x / Rational::from((i + 1) as u32))).collect()
        }
    }

    #[inline]
    fn compose(&self, other: &Self) -> Self {
        assert_eq!(other.seq[0], Rational::from(0));
        if self.seq.len() == 1 { return self.clone(); }
        let reccomp = self.lshift().compose(&other);
        let mut tail = (other.lshift() * reccomp).rshift();
        tail.seq[0] += &self.seq[0];
        return tail;
    }

    #[inline]
    fn inverse(&self) -> Self {
        assert_eq!(self.seq[0], Rational::from(0));
        let mut r = Self {
            seq: vec![Default::default(); self.seq.len()]
        };
        let comp = self.lshift();
        for _i in 0..self.seq.len() {
            r = (Self::promote(Rational::from(1)) / comp.compose(&r)).rshift();
        }
        r
    }

    #[inline]
    fn hadamard(&self, other: &Self) -> Self {
        Self {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(x, y)| x * y).map(|z| z.complete()).collect()
        }
    }

    #[inline]
    fn sqrt(&self) -> Self {
        // for now, tonnelli-shanks later
        assert!(self.seq[0] == Rational::from(1));
        let mut r = Self::promote(Rational::from(1));
        for _i in 0..self.seq.len() {
            let q = (self.clone() - r.clone() * r.clone()).tail_term() / (Self::promote(Rational::from(2)) * r.clone()).tail_term();
            if q == Self::promote(Rational::from(0)) {
                return r;
            }
            r = r + q;
        }
        r
    }

    #[inline]
    fn ratpow(self, _p: i64, _q: i64) -> Self {
        unimplemented!()
    }

    #[inline]
    fn lshift(&self) -> Self {
        Self {
            seq: self.seq.iter().skip(1).cloned().collect()
        }
    }

    #[inline]
    fn rshift(&self) -> Self {
        Self {
            seq: once(Default::default()).chain(self.seq.iter().cloned()).collect()
        }
    }
}

impl Series {
    fn new(vec: Vec<Rational>) -> Self {
        Self {
            seq: vec
        }
    }
    fn new_u32(vec: Vec<u32>) -> Self {
        Self {
            seq: vec.iter().map(|&x| Rational::from(x)).collect()
        }
    }
    #[inline]
    fn tail_term(&self) -> Self {
        let mut found = false;
        Self {
            seq: self.seq.iter().map(|x| 
                if found { 
                    x.clone()
                } else { 
                    if x.cmp0() == std::cmp::Ordering::Equal { 
                        x.clone()
                    } else { 
                        found = true; 
                        Rational::from(0) 
                    } 
                }
            ).collect()
        }
    }
    #[inline]
    fn pop(&self) -> Self {
        Self {
            seq: self.seq.iter().take(self.seq.len() - 1).cloned().collect()
        }
    }
}
