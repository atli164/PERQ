use std::ops::{Add, Sub};
use crate::Ring;

pub struct Series<T: Ring + Copy> {
    coeff: Vec<T>
}

impl<T: Ring + Copy> From<u32> for Series<T> {
    fn from(x: u32) -> Series<T> {
        Series {
            coeff: vec![T::from(x)]
        }
    }
}

impl<T: Ring + Copy> Add for Series<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            coeff: self.coeff.iter()
                .zip(other.coeff.iter())
                .map(|(x, y)| x.clone() + y.clone()).collect()
        }
    }
}

impl<T: Ring + Copy> Sub for Series<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            coeff: self.coeff.iter()
                .zip(other.coeff.iter())
                .map(|(x, y)| x.clone() - y.clone()).collect()
        }
    }
}

// Due to length, Generating Series is shortened as GS going forward

pub trait GS<T> where T: Ring + Copy {
    fn coeff(&mut self, i: usize) -> T;
    fn into_bounded(&mut self, len: usize) -> Series<T> {
        let mut res = Vec::with_capacity(len);
        for i in 0..len {
            res.push(self.coeff(i));
        }
        Series {
            coeff: res
        }
    }
}

struct PureFuncGS<T: Ring + Copy> {
    f: fn(usize, &Vec<T>) -> T,
    cache: Vec<T>
}

impl<T: Ring + Copy> GS<T> for PureFuncGS<T> {
    fn coeff(&mut self, i: usize) -> T {
        while self.cache.len() <= i {
            let nw = (self.f)(self.cache.len(), &self.cache);
            self.cache.push(nw);
        }
        self.cache[i]
    }
}

impl<T: Ring + Copy> PureFuncGS<T> {
    fn new(f: fn(usize, &Vec<T>) -> T) -> PureFuncGS<T> {
        PureFuncGS {
            f: f,
            cache: Default::default()
        }
    }
}

struct LinRecGS<T: Ring + Copy> {
    cache: Vec<T>,
    coeff: Vec<T>,
    const_coeff: T
}

impl<T: Ring + Copy> GS<T> for LinRecGS<T> {
    fn coeff(&mut self, i: usize) -> T {
        while self.cache.len() <= i {
            self.calc_next();
        }
        self.cache[i]
    }
}

impl<T: Ring + Copy> LinRecGS<T> {
    fn new(coeff: Vec<T>, initial: Vec<T>, const_coeff: T) -> LinRecGS<T> {
        LinRecGS {
            cache: initial,
            coeff: coeff,
            const_coeff: const_coeff
        }
    }
    fn calc_next(&mut self) {
        let mut sm = self.const_coeff;
        for (i, x) in self.coeff.iter().enumerate() {
            sm = sm + *x * self.cache[self.cache.len() - i - 1];
        }
        self.cache.push(sm);
    }
}

struct CompoundGS<T: Ring + Copy> {
    sub_expr: Vec<Box<dyn GS<T>>>,
    cache: Vec<T>,
    calc_next: fn(usize, &Vec<T>, &Vec<Box<dyn GS<T>>>) -> T
}

// Make sure these functions use at most linear-ish memory
// For example storing all nCk values for k <= n would be far
// too much memory for large sequence values, blotting out any
// speed gains

pub fn a001477<T: Ring + Copy>(i: usize, _cache: &Vec<T>) -> T {
    T::from(i as u32)
}

pub fn oeis<T: Ring + Copy + 'static>(a_ind: u32) -> Option<Box<dyn GS<T>>> {
    match a_ind {
        000045 => Some(Box::new(LinRecGS::new(vec![T::from(1), T::from(1)], vec![T::from(0), T::from(1)], T::from(0)))),
        001477 => Some(Box::new(PureFuncGS::new(a001477))),
        _ => None
    }
}
