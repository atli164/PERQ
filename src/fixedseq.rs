use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
use std::cmp::min;
use crate::{Field, PowerSeries};
use crate::mathtypes::{Zero, One};
use std::iter::zip;

#[derive(Debug, Clone, Copy)]
pub struct FixedSeq<T: Field + Copy, const N: usize> {
    pub seq: [T; N],
    pub cnt: u8
}

pub type ShortSeq<T> = FixedSeq<T, 16>;

impl<T: Field + Copy, const N: usize> PartialEq for FixedSeq<T, N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.seq[0..self.cnt as usize] == other.seq[0..other.cnt as usize] 
    }
}

impl<T: Field + Copy, const N: usize> Eq for FixedSeq<T, N> { }

impl<T: Field + Copy + Ord, const N: usize> PartialOrd for FixedSeq<T, N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Field + Copy + Ord, const N: usize> Ord for FixedSeq<T, N> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seq[0..self.cnt as usize].cmp(&other.seq[0..other.cnt as usize])
    }
}


impl<T: Field + Copy, const N: usize> One for FixedSeq<T, N> {
    #[inline]
    fn one() -> Self {
        let mut seq = [T::zero(); N];
        seq[0] = T::one();
        Self {
            seq,
            cnt: N as u8
        }
    }
    #[inline]
    fn is_one(&self) -> bool {
        if self.cnt == 0 {
            return false;
        }
        if !self.seq[0].is_one() {
            return false;
        }
        self.seq[1..].iter().all(|x| x.is_zero())
    }
}

impl<T: Field + Copy, const N: usize> Zero for FixedSeq<T, N> {
    #[inline]
    fn zero() -> Self {
        Self {
            seq: [T::zero(); N],
            cnt: N as u8
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.seq.iter().take(self.cnt as usize).all(|x| x.is_zero())
    }
}

impl<T: Field + Copy, const N: usize> From<u32> for FixedSeq<T, N> {
    #[inline]
    fn from(x: u32) -> FixedSeq<T, N> {
        FixedSeq::promote(T::from(x))
    }
}

impl<T: Field + Copy, const N: usize> AddAssign<FixedSeq<T, N>> for FixedSeq<T, N> {
    #[inline]
    fn add_assign(&mut self, other: FixedSeq<T, N>) {
        zip(self.seq.iter_mut(), other.seq.iter()).for_each(|(x, y)| *x += y);
        self.cnt = min(self.cnt, other.cnt);
    }
}

impl<T: Field + Copy, const N: usize> Add<FixedSeq<T, N>> for FixedSeq<T, N> {
    type Output = FixedSeq<T, N>;

    #[inline]
    fn add(self, other: FixedSeq<T, N>) -> FixedSeq<T, N> {
        let mut res = self;
        res += other;
        res
    }
}

impl<T: Field + Copy, const N: usize> Neg for FixedSeq<T, N> {
    type Output = FixedSeq<T, N>;

    #[inline]
    fn neg(self) -> FixedSeq<T, N> {
        let mut seq: [T; N] = [T::zero(); N];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = -self.seq[i]
        }
        FixedSeq::<T, N> {
            seq,
            cnt: self.cnt
        }
    }
}

impl<T: Field + Copy, const N: usize> SubAssign<FixedSeq<T, N>> for FixedSeq<T, N> {
    #[inline]
    fn sub_assign(&mut self, other: FixedSeq<T, N>) {
        zip(self.seq.iter_mut(), other.seq.iter()).for_each(|(x, y)| *x -= y);
        self.cnt = min(self.cnt, other.cnt);
    }
}

impl<T: Field + Copy, const N: usize> Sub<FixedSeq<T, N>> for FixedSeq<T, N> {
    type Output = FixedSeq<T, N>;

    #[inline]
    fn sub(self, other: FixedSeq<T, N>) -> FixedSeq<T, N> {
        let mut res = self;
        res -= other;
        res
    }
}

impl<T: Field + Copy, const N: usize> Mul<FixedSeq<T, N>> for FixedSeq<T, N> {
    type Output = FixedSeq<T, N>;

    #[inline]
    fn mul(self, other: FixedSeq<T, N>) -> FixedSeq<T, N> {
        let mut seq: [T; N] = [T::zero(); N];
        for i in 0..N {
            for j in 0..N-i {
                seq[i + j] += self.seq[i] * other.seq[j];
            }
        }
        FixedSeq::<T, N> {
            seq,
            cnt: min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy, const N: usize> MulAssign<FixedSeq<T, N>> for FixedSeq<T, N> {
    #[inline]
    fn mul_assign(&mut self, other: FixedSeq<T, N>) {
        *self = *self * other;
    }
}

impl<T: Field + Copy, const N: usize> Div<FixedSeq<T, N>> for FixedSeq<T, N> {
    type Output = FixedSeq<T, N>;

    #[inline]
    fn div(self, other: FixedSeq<T, N>) -> FixedSeq<T, N> {
        let mut res = self;
        res /= other;
        res
    }
}

impl<T: Field + Copy, const N: usize> DivAssign<FixedSeq<T, N>> for FixedSeq<T, N> {
    #[inline]
    fn div_assign(&mut self, other: FixedSeq<T, N>) {
        for i in 0..N {
            self.seq[i] /= other.seq[0];
            for j in (i+1)..N {
                self.seq[j] -= self.seq[i] * other.seq[j - i];
            }
        }
        self.cnt = min(self.cnt, other.cnt);
    }
}

impl<T: Field + Copy, const N: usize> Index<usize> for FixedSeq<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.seq[index]
    }
}

impl<T: Field + Copy, const N: usize> IndexMut<usize> for FixedSeq<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.seq[index]
    }
}

impl<T: Field + Copy, const N: usize> PowerSeries for FixedSeq<T, N> {
    type Coeff = T;

    #[inline]
    fn expand_to(&mut self, _l: usize) {

    }

    #[inline]
    fn accuracy(&self) -> usize {
        self.cnt as usize
    }

    #[inline]
    fn nonzero_num(&self) -> usize {
        self.cnt as usize
    }
    
    #[inline]
    fn limit_accuracy(&mut self, l: usize) {
        self.cnt = min(l as u8, self.cnt);
    }

    #[inline]
    fn lshift(&self) -> Self {
        let mut seq: [T; N] = [T::zero(); N];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = if i + 1 == N { T::zero() } else { self.seq[i+1] };
        }
        Self {
            seq,
            cnt: self.cnt.saturating_sub(1)
        }
    }

    #[inline]
    fn rshift(&self) -> Self {
        let mut seq: [T; N] = [T::zero(); N];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = if i == 0 { T::zero() } else { self.seq[i-1] };
        }
        Self {
            seq,
            cnt: min(N as u8, self.cnt + 1)
        }
    }
}

forward_into_ref_field! { impl Field for FixedSeq<T, N> where T: Field + std::marker::Copy, const N: usize }

impl<T: Field + Copy, const N: usize> FixedSeq<T, N> {
    fn new(arr: [T; N]) -> Self {
        Self {
            seq: arr,
            cnt: N as u8
        }
    }
    fn new_u32(arr: [u32; N]) -> Self {
        let mut seq: [T; N] = [T::zero(); N];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = T::from(arr[i]);
        }
        Self {
            seq,
            cnt: N as u8
        }
    }
    fn pop(&self) -> Self {
        Self {
            seq: self.seq,
            cnt: self.cnt.saturating_sub(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ShortSeq;
    use crate::ModIntP32;
    use crate::PowerSeries;
    #[test]
    fn mul_test() {
        let catalan = ShortSeq::<ModIntP32>::new_u32([1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845]);
        assert_eq!(catalan, (catalan * catalan).rshift() + ShortSeq::<ModIntP32>::promote(ModIntP32::from(1)));
        let mut sq = catalan; 
        sq *= catalan;
        sq = sq.rshift();
        sq += ShortSeq::<ModIntP32>::promote(ModIntP32::from(1));
        assert_eq!(catalan, sq);
    }

    #[test]
    fn div_test() {
        let geom = ShortSeq::<ModIntP32>::new_u32([1; 16]);
        let mut poly = ShortSeq::<ModIntP32>::promote(ModIntP32::from(1));
        poly.seq[1] = -ModIntP32::from(1);
        assert_eq!(ShortSeq::<ModIntP32>::promote(ModIntP32::from(1)) / poly, geom);
    }

    #[test]
    fn compose_test() {
        let geom = ShortSeq::<ModIntP32>::new_u32([1; 16]);
        let nonzgeom = ShortSeq::<ModIntP32>::new_u32([0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        let pow2 = ShortSeq::<ModIntP32>::new_u32([1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384]);
        assert_eq!(geom.compose(&nonzgeom), pow2);
        let mut expseq: [ModIntP32; 16] = [ModIntP32::from(1); 16];
        for i in 1..16 {
            expseq[i] = ModIntP32::from(i as u32) * expseq[i - 1];
        }
        for i in 0..16 {
            expseq[i] = ModIntP32::from(1) / expseq[i];
        }
        let a262 = ShortSeq::<ModIntP32>::new_u32([1, 1, 3, 13, 73, 501, 4051, 37633, 394353, 4596553, 58941091, 824073141, (12470162233u64 % 65521u64) as u32, (202976401213u64 % 65521u64) as u32, (3535017524403u64 % 65521u64) as u32, (65573803186921u64 % 65521u64) as u32]);
        let exp = ShortSeq::<ModIntP32>::new(expseq);
        assert_eq!(exp.compose(&nonzgeom), a262.hadamard(&exp));
    }

    #[test]
    fn inverse_test() {
        let tang = ShortSeq::<ModIntP32>::new_u32([0, 1, 0, 2, 0, 16, 0, 272, 0, 7936, 0, 353792, 0, 22368256, 0, 1903757312]);
        let mut atanseq: [ModIntP32; 16] = Default::default();
        for i in (1..16).step_by(2) {
            atanseq[i] = ModIntP32::from(1) / ModIntP32::from(i as u32);
            if i % 4 == 3 { atanseq[i] = -atanseq[i]; }
        }
        let atan = ShortSeq::<ModIntP32>::new(atanseq);
        let mut expseq: [ModIntP32; 16] = [ModIntP32::from(1); 16];
        for i in 1..16 {
            expseq[i] = ModIntP32::from(i as u32) * expseq[i - 1];
        }
        for i in 0..16 {
            expseq[i] = ModIntP32::from(1) / expseq[i];
        }
        let exp = ShortSeq::<ModIntP32>::new(expseq);
        assert_eq!(atan.inverse(), tang.hadamard(&exp));
    }

    #[test]
    fn sqrt_test() {
        let simple = ShortSeq::<ModIntP32>::new_u32([1, 11, 19, 23, 3, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(simple, (simple * simple).sqrt());
        let catalan = ShortSeq::<ModIntP32>::new_u32([1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845]);
        assert_eq!(catalan.pop(), catalan.lshift().sqrt());
    }
}
