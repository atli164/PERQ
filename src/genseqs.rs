use std::ops::{Add, Sub, Mul, Div};

// Z maps into any ring, so From<i64> makes sense here
pub trait Ring: Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + From<i64> {}
impl<T> Ring for T where T: Clone + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + From<i64> {}
pub trait Field: Clone + Ring + Div<Output = Self> {}
impl<T> Field for T where T: Clone + Ring + Div<Output = Self> {}

pub struct Series<T> where T: Ring {
    coeff: Vec<T>
}

impl<T: Ring> From<i64> for Series<T> {
    fn from(x: i64) -> Series<T> {
        Series {
            coeff: vec![T::from(x)]
        }
    }
}

impl<T: Ring> Add for Series<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            coeff: self.coeff.iter()
                .zip(other.coeff.iter())
                .map(|(x, y)| x.clone() + y.clone()).collect()
        }
    }
}

impl<T: Ring> Sub for Series<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            coeff: self.coeff.iter()
                .zip(other.coeff.iter())
                .map(|(x, y)| x.clone() - y.clone()).collect()
        }
    }
}

pub trait GeneratingSeries<T> where T: Ring {
    fn coeff(&self, i: usize) -> T;
    fn into_bounded(&self, len: usize) -> Series<T> {
        let mut res = Vec::with_capacity(len);
        for i in 0..len {
            res.push(self.coeff(i));
        }
        Series {
            coeff: res
        }
    }
}

// Due to length, Generating Series is shortened as GS going forward

struct PureFuncPrimitiveGS<T: Ring> {
    f: fn(usize) -> T
}

impl<T: Ring> GeneratingSeries<T> for PureFuncPrimitiveGS<T> {
    fn coeff(&self, i: usize) -> T {
        (self.f)(i)
    }
}

// Make sure these functions use at most linear-ish memory
// For example storing all nCk values for k <= n would be far
// too much memory for large sequence values, blotting out any
// speed gains

pub fn a000045<T: Ring>(i: usize) -> T {
    match i {
        0 => T::from(0),
        1 => T::from(1),
        _ => a000045::<T>(i - 1) + a000045::<T>(i - 2)
    }
}

pub fn a001477<T: Ring>(i: usize) -> T {
    T::from(i as i64)
}

pub fn oeis<T: Ring + 'static>(a_ind: u32) -> Option<Box<dyn GeneratingSeries<T>>> {
    match a_ind {
        000045 => Some(Box::new(PureFuncPrimitiveGS {
            f: a000045
        })),
        001477 => Some(Box::new(PureFuncPrimitiveGS {
            f: a001477
        })),
        _ => None
    }
}
