use crate::{Ring, Field};
use std::ops::{Add, Sub, Mul, Neg};
use std::iter::zip;

#[derive(Debug, Clone)]
pub struct Matrix<T: Ring + Copy> {
    r: usize,
    c: usize,
    dat: Vec<T>
}

impl<T: Ring + Copy> std::ops::Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.dat[index.0 * self.c + index.1]
    }
}

impl<T: Ring + Copy> std::ops::IndexMut<(usize,usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.dat[index.0 * self.c + index.1]
    }
}

impl<T: Ring + Copy> Add for Matrix<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r,
            c: self.c,
            dat: zip(self.dat.iter(), other.dat.iter()).map(|(&x, &y)| x + y).collect()
        }
    }
}

impl<T: Ring + Copy> Sub for Matrix<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r,
            c: self.c,
            dat: zip(self.dat.iter(), other.dat.iter()).map(|(&x, &y)| x - y).collect()
        }
    }
}

impl<T: Ring + Copy> Neg for Matrix<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            r: self.r,
            c: self.c,
            dat: self.dat.iter().map(|&x| -x).collect()
        }
    }
}

impl<T: Ring + Copy> Mul for Matrix<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut res = Self::new(self.r, other.c);
        for i in 0..self.r {
            for k in 0..self.c {
                for j in 0..other.c {
                    res[(i, j)] = res[(i, j)] + self[(i, k)] * other[(k, j)];
                }
            }
        }
        res
    }
}

impl<T: Ring + Copy> Matrix<T> {
    pub fn new(r: usize, c: usize) -> Self {
        Self {
            r: r,
            c: c,
            dat: vec![Default::default(); r * c]
        }
    }
}

impl<T: Field + Copy> Matrix<T> {
    // returns reduced matrix, determinant and rank
    fn rref(&self) -> (Self, T, usize) {
        let (mut mat, mut det, mut rank) = (self.clone(), T::from(1u32), 0);
        let mut ri = 0;
        for ci in 0..self.c {
            let mut k = ri;
            while k < self.r && mat[(k, ci)] == T::from(0u32) {
                k += 1;
            }
            if k == self.r {
                continue;
            }
            if k != ri {
                det = -det;
                for i in 0..self.c {
                    mat.dat.swap(k * self.c + i, ri * self.c + i);
                }
            }
            det = det * mat[(ri, ri)];
            rank += 1;
            let d = mat[(ri, ci)];
            for i in 0..self.c {
                mat[(ri, i)] = mat[(ri, i)] / d;
            }
            for i in 0..self.r {
                let piv = mat[(i, ci)];
                if i != ri && piv != T::from(0u32) {
                    for j in 0..self.c {
                        mat[(i, j)] = mat[(i, j)] - piv * mat[(ri, j)];
                    }
                }
            }
            ri += 1;
        }
        (mat, det, rank)
    }
    pub fn inverse(&self) -> Option<Self> {
        if self.c != self.r {
            return None;
        }
        let mut aug = Self::new(self.r, 2 * self.c);
        for i in 0..self.r {
            for j in 0..self.c {
                aug[(i, j)] = self[(i, j)];
            }
            aug[(i, i + self.c)] = T::from(1u32);
        }
        let (reduced, det, rank) = aug.rref();
        if rank != self.c {
            return None;
        }
        let mut res = Self::new(self.r, self.c);
        for i in 0..self.r {
            for j in 0..self.c {
                res[(i, j)] = reduced[(i, j + self.c)];
            }
        }
        Some(res)
    }
    pub fn solve(&self, targ: &Vec<T>) -> Option<Vec<T>> {
        if targ.len() != self.r {
            return None;
        }
        let mut aug = Self::new(self.r, self.c + 1);
        for i in 0..self.r {
            for j in 0..self.c {
                aug[(i, j)] = self[(i, j)];
            }
            aug[(i, self.c)] = targ[i];
        }
        let (reduced, det, rank) = aug.rref();
        let mut res = vec![Default::default(); self.c];
        for i in (0..self.r).rev() {
            let mut piv = 0;
            while piv < self.c && reduced[(i, piv)] == T::from(0u32) {
                piv += 1;
            }
            if piv == self.c {
                if reduced[(i, piv)] != T::from(0u32) {
                    return None;
                } else {
                    continue;
                }
            }
            let mut sm = reduced[(i, self.c)];
            for j in piv+1..self.c {
                sm = sm - reduced[(i, j)] * res[j];
            }
            res[piv] = sm;
        }
        Some(res)
    }
}
