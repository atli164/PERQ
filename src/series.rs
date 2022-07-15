use std::ops::{Add, Sub, Neg, Mul, Div};
use std::iter::{zip, once};
use crate::{Field, PowerSeries};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Series<T: Field> {
    pub seq: Vec<T>
}

impl<T: Field> From<u32> for Series<T> {
    #[inline]
    fn from(x: u32) -> Series<T> {
        Series::promote(T::from(x))
    }
}

macro_rules! forward_binop_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a, T: Field> $imp<$t> for &'a $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(self, &other)
            }
        }

        impl<'a, T: Field> $imp<&'a $t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: &'a $t) -> $t {
                $imp::$method(&self, other)
            }
        }

        impl<T: Field> $imp<$t> for $t {
            type Output = $t;

            #[inline]
            fn $method(self, other: $t) -> $t {
                $imp::$method(&self, &other)
            }
        }
    }
}

macro_rules! forward_unop_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<T: Field> $imp for $t {
            type Output = $t;

            #[inline]
            fn $method(self) -> $t {
                $imp::$method(&self)
            }
        }
    }
}

impl<'a, 'b, T: Field> Add<&'a Series<T>> for &'b Series<T> {
    type Output = Series<T>;

    #[inline]
    fn add(self, other: &'a Series<T>) -> Series<T> {
        Series::<T> {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(&x, &y)| x + y).collect()
        }
    }
}

forward_binop_impl! { impl Add, add for Series<T> }

impl<'a, T: Field> Neg for &'a Series<T> {
    type Output = Series<T>;

    #[inline]
    fn neg(self) -> Series<T> {
        Series::<T> {
            seq: self.seq.iter().map(|&x| -x).collect()
        }
    }
}

forward_unop_impl! { impl Neg, neg for Series<T> }

impl<'a, 'b, T: Field> Sub<&'a Series<T>> for &'b Series<T> {
    type Output = Series<T>;

    #[inline]
    fn sub(self, other: &'a Series<T>) -> Series<T> {
        Series::<T> {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(&x, &y)| x - y).collect()
        }
    }
}

forward_binop_impl! { impl Sub, sub for Series<T> }

impl<'a, 'b, T: Field> Mul<&'a Series<T>> for &'b Series<T> {
    type Output = Series<T>;

    #[inline]
    fn mul(self, other: &'a Series<T>) -> Series<T> {
        let n = std::cmp::min(self.seq.len(), other.seq.len());
        let mut seq = vec![Default::default(); n];
        for i in 0..n {
            for j in 0..n-i {
                seq[i + j] = seq[i + j] + (self.seq[i] * other.seq[j]);
            }
        }
        Series::<T> {
            seq: seq
        }
    }
}

forward_binop_impl! { impl Mul, mul for Series<T> }

impl<'a, 'b, T: Field> Div<&'a Series<T>> for &'b Series<T> {
    type Output = Series<T>;

    #[inline]
    fn div(self, other: &'a Series<T>) -> Series<T> {
        let n = std::cmp::min(self.seq.len(), other.seq.len());
        let mut seq = self.seq.clone();
        for i in 0..n {
            seq[i] = seq[i] / other.seq[0];
            for j in (i+1)..n {
                seq[j] = seq[j] - seq[i] * other.seq[j - i];
            }
        }
        Series::<T> {
            seq: seq
        }
    }
}

forward_binop_impl! { impl Div, div for Series<T> }

impl<T: Field> PowerSeries for Series<T> {
    type Coeff = T;

    #[inline]
    fn promote(x: T) -> Self {
        let mut res: Self = Default::default();
        res.seq.push(x);
        res
    }

    #[inline]
    fn coefficient(&self, i: usize) -> Self::Coeff {
        self.seq[i]
    }

    #[inline]
    fn identity() -> Self {
        let mut res: Self = Default::default();
        res.seq.push(T::from(1));
        res
    }

    #[inline]
    fn derive(&self) -> Self {
        Self {
            seq: self.seq.iter().enumerate().skip(1).map(|(i, &x)| T::from(i as u32) * x).collect()
        }
    }

    #[inline]
    fn integrate(&self) -> Self {
        Self {
            seq: once(Default::default()).chain(self.seq.iter().enumerate().map(|(i, &x)| x / T::from((i + 1) as u32))).collect()
        }
    }

    #[inline]
    fn compose(&self, other: &Self) -> Self {
        assert_eq!(other.seq[0], T::from(0));
        if self.seq.len() == 1 { return self.clone(); }
        let reccomp = self.lshift().compose(&other);
        let mut tail = (other.lshift() * reccomp).rshift();
        tail.seq[0] = tail.seq[0] + self.seq[0];
        return tail;
    }

    #[inline]
    fn inverse(&self) -> Self {
        assert_eq!(self.seq[0], T::from(0));
        let mut r = Self {
            seq: vec![Default::default(); self.seq.len()]
        };
        let comp = self.lshift();
        for _i in 0..self.seq.len() {
            r = (Self::promote(T::from(1)) / comp.compose(&r)).rshift();
        }
        r
    }

    #[inline]
    fn hadamard(&self, other: &Self) -> Self {
        Self {
            seq: zip(self.seq.iter(), other.seq.iter()).map(|(&x, &y)| x * y).collect()
        }
    }

    #[inline]
    fn sqrt(&self) -> Self {
        // for now, tonnelli-shanks later
        assert!(self.seq[0] == T::from(1));
        let mut r = Self::promote(T::from(1));
        for _i in 0..self.seq.len() {
            let q = (self.clone() - r.clone() * r.clone()).tail_term() / (Self::promote(T::from(2)) * r.clone()).tail_term();
            if q == Self::promote(T::from(0)) {
                return r;
            }
            r = r + q;
        }
        r
    }

    //#[inline]
    //fn ratpow(self, p: i64, q: i64) -> Self {
    //    unimplemented!()
    //}

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

impl<T: Field> Series<T> {
    fn new(vec: Vec<T>) -> Self {
        Self {
            seq: vec
        }
    }
    fn new_u32(vec: Vec<u32>) -> Self {
        Self {
            seq: vec.iter().map(|&x| T::from(x)).collect()
        }
    }
    #[inline]
    fn tail_term(&self) -> Self {
        let mut found = false;
        Self {
            seq: self.seq.iter().map(|&x| if found { x } else { if x == T::from(0) { x } else { found = true; T::from(0) } }).collect()
        }
    }
    fn pop(&self) -> Self {
        Self {
            seq: self.seq.iter().take(self.seq.len() - 1).cloned().collect()
        }
    }
}
