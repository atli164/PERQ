use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
use std::cmp::min;
use std::iter::{zip, once};
use crate::{PowerSeries};
use crate::mathtypes::{Zero, One};
use rug::{Rational, Complete};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Series {
    pub seq: Vec<Rational>,
    pub acc: usize
}

impl Zero for Series {
    #[inline]
    fn zero() -> Self {
        Self {
            seq: vec![],
            acc: usize::MAX
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        if self.acc == 0 {
            return false;
        }
        self.seq.iter().all(|x| x.cmp0() == std::cmp::Ordering::Equal)
    }
}

impl One for Series {
    #[inline]
    fn one() -> Self {
        Self {
            seq: vec![Rational::from(1)],
            acc: usize::MAX
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
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(x, y)| x + y).map(|z| z.complete()).collect(),
            acc: min(self.acc, other.acc)
        }
    }
}

impl<'a> AddAssign<&'a Series> for Series {
    #[inline]
    fn add_assign(&mut self, other: &'a Series) {
        zip(self.seq.iter_mut(), other.seq.iter()).for_each(|(x, y)| *x += y);
    }
}

impl<'a> Neg for &'a Series {
    type Output = Series;

    #[inline]
    fn neg(self) -> Series {
        Series {
            seq: self.seq.iter().map(|x| (-x).complete()).collect(),
            acc: self.acc
        }
    }
}

impl<'a, 'b> Sub<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn sub(self, other: &'a Series) -> Series {
        Series {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(x, y)| x - y).map(|z| z.complete()).collect(),
            acc: min(self.acc, other.acc)
        }
    }
}

impl<'a> SubAssign<&'a Series> for Series {
    #[inline]
    fn sub_assign(&mut self, other: &'a Series) {
        zip(self.seq.iter_mut(), other.seq.iter()).for_each(|(x, y)| *x -= y);
    }
}

impl<'a, 'b> Mul<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn mul(self, other: &'a Series) -> Series {
        let n = min(self.seq.len(), other.seq.len());
        let mut seq = vec![Default::default(); n];
        for i in 0..n {
            for j in 0..n-i {
                let prod = (&self.seq[i] * &other.seq[j]).complete();
                seq[i + j] += prod;
            }
        }
        // more fanciful acc later
        Series {
            seq,
            acc: min(self.acc, other.acc)
        }
    }
}

impl<'a> MulAssign<&'a Series> for Series {
    #[inline]
    fn mul_assign(&mut self, other: &'a Series) {
        *self = &*self * other;
    }
}

impl<'a> DivAssign<&'a Series> for Series {
    #[inline]
    fn div_assign(&mut self, other: &'a Series) {
        let n = min(self.seq.len(), other.seq.len());
        for i in 0..n {
            self.seq[i] /= &other.seq[0];
            for j in (i+1)..n {
                let prod = (&self.seq[i] * &other.seq[j - i]).complete();
                self.seq[j] -= prod;
            }
        }
    }
}

impl<'a, 'b> Div<&'a Series> for &'b Series {
    type Output = Series;

    #[inline]
    fn div(self, other: &'a Series) -> Series {
        let mut cpy = self.clone();
        cpy /= other;
        cpy
    }
}

forward_from_ref_field! { impl Field for Series }

impl Index<usize> for Series {
    type Output = Rational;

    fn index(&self, index: usize) -> &Rational {
        &self.seq[index]
    }
}

impl IndexMut<usize> for Series {
    fn index_mut(&mut self, index: usize) -> &mut Rational {
        &mut self.seq[index]
    }
}

impl PowerSeries for Series {
    type Coeff = Rational;

    #[inline]
    fn expand_to(&mut self, l: usize) {
        if l > self.seq.len() {
            self.seq.resize(l, Self::Coeff::zero());
        }
    }

    #[inline]
    fn accuracy(&self) -> usize {
        self.seq.len()
    }

    #[inline]
    fn nonzero_num(&self) -> usize {
        let mut ans = self.seq.len();
        while ans > 0 && self[ans - 1].is_zero() {
            ans -= 1;
        }
        ans
    }

    #[inline]
    fn limit_accuracy(&mut self, l: usize) {
        self.acc = min(self.acc, l);
        self.seq.truncate(l);
    }

    #[inline]
    fn lshift(&self) -> Self {
        Self::new(
            self.seq.iter().skip(1).cloned().collect()
        )
    }

    #[inline]
    fn rshift(&self) -> Self {
        Self::new(
            once(Default::default()).chain(self.seq.iter().cloned()).collect()
        )
    }
}

impl Series {
    fn new(vec: Vec<Rational>) -> Self {
        let acc = vec.len();
        Self {
            seq: vec,
            acc
        }
    }
    fn new_u32(vec: Vec<u32>) -> Self {
        Self::new(
            vec.iter().map(|&x| Rational::from(x)).collect()
        )
    }
    #[inline]
    fn pop(&self) -> Self {
        Self::new(
            self.seq.iter().take(self.seq.len() - 1).cloned().collect()
        )
    }
}
