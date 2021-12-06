use std::ops::{Add, Sub, Mul, Div};
use std::sync::Mutex;

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

pub trait UnboundedSeries<T> where T: Ring {
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

// Due to length, Unbounded Series is shortened as US going forward

struct NoMemoryPrimitiveUS<T> where T: Ring {
    f: fn(usize) -> T
}

impl<T: Ring> UnboundedSeries<T> for NoMemoryPrimitiveUS<T> {
    fn coeff(&self, i: usize) -> T {
        (self.f)(i)
    }
}

struct VectorMemoryPrimitiveUS<T> where T: Ring {
    next: fn(&Vec<T>) -> T,
    memo: Mutex<Vec<T>>
}

impl<T: Ring> UnboundedSeries<T> for VectorMemoryPrimitiveUS<T> {
    fn coeff(&self, i: usize) -> T {
        let vec = &mut self.memo.lock().unwrap();
        while vec.len() <= i {
            let val = (self.next)(&vec);
            vec.push(val);
        }
        vec[i].clone()
    }
}

pub fn oeis<T: Ring + 'static>(a_ind: u32) -> Option<Box<dyn UnboundedSeries<T>>> {
    match a_ind {
        000045 => Some(Box::new(VectorMemoryPrimitiveUS {
            memo: Mutex::new(vec![T::from(0), T::from(1)]),
            next: |v: &Vec<T>| {
                v[v.len() - 2].clone() + v[v.len() - 1].clone()
            }
        })),
        001477 => Some(Box::new(NoMemoryPrimitiveUS {
            f: |x: usize| {
                T::from(x as i64)
            }
        })),
        _ => None
    }
}
