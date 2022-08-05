use crate::{Ring, Field};
use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign};
use std::iter::zip;

#[derive(Debug, Clone)]
pub struct Matrix<T: Ring> {
    r: usize,
    c: usize,
    dat: Vec<T>
}

impl<T: Ring> std::ops::Index<(usize,usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.dat[index.0 * self.c + index.1]
    }
}

impl<T: Ring> std::ops::IndexMut<(usize,usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.dat[index.0 * self.c + index.1]
    }
}

impl<'a, 'b, T: Ring> Add<&'a Matrix<T>> for &'b Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: &'a Matrix<T>) -> Matrix<T> {
        Matrix::<T> {
            r: self.r,
            c: self.c,
            dat: zip(self.dat.iter(), other.dat.iter()).map(|(x, y)| x.clone() + y).collect()
        }
    }
}

impl<'a, T: Ring> AddAssign<&'a Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, other: &'a Matrix<T>) {
        zip(self.dat.iter_mut(), other.dat.iter()).for_each(|(x, y)| *x += y);
    }
}

impl<'a, 'b, T: Ring> Sub<&'a Matrix<T>> for &'b Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: &'a Matrix<T>) -> Matrix<T> {
        Matrix::<T> {
            r: self.r,
            c: self.c,
            dat: zip(self.dat.iter(), other.dat.iter()).map(|(x, y)| x.clone() - y).collect()
        }
    }
}

impl<'a, T: Ring> SubAssign<&'a Matrix<T>> for Matrix<T> {
    fn sub_assign(&mut self, other: &'a Matrix<T>) {
        zip(self.dat.iter_mut(), other.dat.iter()).for_each(|(x, y)| *x -= y);
    }
}

impl<'a, T: Ring> Neg for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn neg(self) -> Matrix<T> {
        Matrix::<T> {
            r: self.r,
            c: self.c,
            dat: self.dat.iter().map(|x| -x.clone()).collect()
        }
    }
}

impl<'a, 'b, T: Ring> Mul<&'a Matrix<T>> for &'b Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: &'a Matrix<T>) -> Matrix<T> {
        let mut res = Matrix::<T>::new(self.r, other.c);
        for i in 0..self.r {
            for k in 0..self.c {
                for j in 0..other.c {
                    res[(i, j)] += self[(i, k)].clone() * &other[(k, j)];
                }
            }
        }
        res
    }
}

impl<'a, T: Ring> MulAssign<&'a Matrix<T>> for Matrix<T> {
    fn mul_assign(&mut self, other: &'a Matrix<T>) {
        *self = &*self * other;
    }
}

forward_from_ref_ring! { impl Ring for Matrix<T> where T: Ring + Copy }

impl<T: Ring> Matrix<T> {
    pub fn new(r: usize, c: usize) -> Self {
        Self {
            r,
            c,
            dat: vec![T::zero(); r * c]
        }
    }
}

impl<T: Field> Matrix<T> {
    // returns reduced matrix, determinant and rank
    fn rref(&self) -> (Self, T, usize) {
        let (mut mat, mut det, mut rank) = (self.clone(), T::one(), 0);
        let mut ri = 0;
        for ci in 0..self.c {
            let mut k = ri;
            while k < self.r && mat[(k, ci)].is_zero() {
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
            det *= &mat[(ri, ri)];
            rank += 1;
            let d = mat[(ri, ci)].clone();
            for i in 0..self.c {
                mat[(ri, i)] /= &d;
            }
            for i in 0..self.r {
                let piv = mat[(i, ci)].clone();
                if i != ri && !piv.is_zero() {
                    for j in 0..self.c {
                        let z = piv.clone() * &mat[(ri, j)];
                        mat[(i, j)] -= z;
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
                aug[(i, j)] = self[(i, j)].clone();
            }
            aug[(i, i + self.c)] = T::one();
        }
        let (reduced, _det, rank) = aug.rref();
        if rank != self.c {
            return None;
        }
        let mut res = Self::new(self.r, self.c);
        for i in 0..self.r {
            for j in 0..self.c {
                res[(i, j)] = reduced[(i, j + self.c)].clone();
            }
        }
        Some(res)
    }
    pub fn solve(&self, targ: &[T]) -> Option<Vec<T>> {
        if targ.len() != self.r {
            return None;
        }
        let mut aug = Self::new(self.r, self.c + 1);
        for i in 0..self.r {
            for j in 0..self.c {
                aug[(i, j)] = self[(i, j)].clone();
            }
            aug[(i, self.c)] = targ[i].clone();
        }
        let (reduced, _det, _rank) = aug.rref();
        let mut res = vec![T::zero(); self.c];
        for i in (0..self.r).rev() {
            let mut piv = 0;
            while piv < self.c && reduced[(i, piv)].is_zero() {
                piv += 1;
            }
            if piv == self.c {
                if !reduced[(i, piv)].is_zero() {
                    return None;
                } else {
                    continue;
                }
            }
            let mut sm = reduced[(i, self.c)].clone();
            for j in piv+1..self.c {
                sm -= reduced[(i, j)].clone() * &res[j];
            }
            res[piv] = sm;
        }
        Some(res)
    }
}
