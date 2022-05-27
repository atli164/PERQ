use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::{Field, PowerSeries};

#[derive(Debug, Clone, Copy)]
pub struct ShortSeq<T: Field + Copy> {
    pub seq: [T; 16],
    pub cnt: u8
}

impl<T: Field + Copy> PartialEq for ShortSeq<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let mx = std::cmp::min(self.cnt, other.cnt) as usize;
        for i in 0..mx {
            if self.seq[i] != other.seq[i] {
                return false;
            }
        }
        true
    }
}


impl<T: Field + Copy> Default for ShortSeq<T> {
    #[inline]
    fn default() -> Self {
        Self {
            seq: Default::default(),
            cnt: 16
        }
    }
}

impl<T: Field + Copy> From<u32> for ShortSeq<T> {
    #[inline]
    fn from(x: u32) -> ShortSeq<T> {
        ShortSeq::promote(T::from(x))
    }
}

impl<T: Field + Copy> Add for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        // This seems to get automatically unrolled
        // Spelling it out explicitly makes no performance difference
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] + other.seq[i]
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy> Neg for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = -self.seq[i]
        }
        Self {
            seq: seq,
            cnt: self.cnt
        }
    }
}

impl<T: Field + Copy> Sub for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] - other.seq[i]
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}


impl<T: Field + Copy> Mul for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for i in 0..16 {
            for j in 0..16-i {
                seq[i + j] = seq[i + j] + (self.seq[i] * other.seq[j]);
            }
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy> Div for ShortSeq<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let mut seq: [T; 16] = self.seq.clone();
        for i in 0..16 {
            seq[i] = seq[i] / other.seq[0];
            for j in (i+1)..16 {
                seq[j] = seq[j] - seq[i] * other.seq[j - i];
            }
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }
}

impl<T: Field + Copy> PowerSeries for ShortSeq<T> {
    type Coeff = T;

    #[inline]
    fn promote(x: T) -> Self {
        let mut res: Self = Default::default();
        res.seq[0] = x;
        res
    }

    #[inline]
    fn coefficient(self, i: usize) -> Self::Coeff {
        self.seq[i]
    }

    #[inline]
    fn identity() -> Self {
        let mut res: ShortSeq<T> = Default::default();
        res.seq[1] = T::from(1);
        res
    }

    #[inline]
    fn derive(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for i in 1..16 {
            seq[i - 1] = self.seq[i] * T::from(i as u32);
        }
        Self {
            seq: seq,
            cnt: self.cnt.saturating_sub(1)
        }
    }

    #[inline]
    fn integrate(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for i in 1..16 {
            seq[i] = self.seq[i - 1] / T::from(i as u32);
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(16, self.cnt + 1)
        }
    }

    #[inline]
    fn compose(self, other: Self) -> Self {
        assert_eq!(other.seq[0], T::from(0));
        if self.cnt == 1 { return self; }
        let reccomp = self.lshift().compose(other);
        let mut tail = (other.lshift() * reccomp).rshift();
        tail.seq[0] = tail.seq[0] + self.seq[0];
        // cnt value?
        return tail;
    }

    #[inline]
    fn inverse(self) -> Self {
        assert_eq!(self.seq[0], T::from(0));
        let mut r: Self = Default::default();
        let comp = self.lshift();
        r.cnt = self.cnt;
        for _i in 0..self.cnt {
            r = (Self::promote(T::from(1)) / comp.compose(r)).rshift();
        }
        return r;
    }

    #[inline]
    fn hadamard(self, other: Self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = self.seq[i] * other.seq[i]
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(self.cnt, other.cnt)
        }
    }

    #[inline]
    fn sqrt(self) -> Self {
        // for now, tonnelli-shanks later
        assert!(self.seq[0] == T::from(1));
        let mut r = Self::promote(T::from(1));
        for _i in 0..16 {
            let q = (self - r * r).tail_term() / (Self::promote(T::from(2)) * r).tail_term();
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
    fn lshift(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = if i == 15 { Default::default() } else { self.seq[i+1] };
        }
        Self {
            seq: seq,
            cnt: self.cnt.saturating_sub(1)
        }
    }

    #[inline]
    fn rshift(self) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = if i == 0 { Default::default() } else { self.seq[i-1] };
        }
        Self {
            seq: seq,
            cnt: std::cmp::min(16, self.cnt + 1)
        }
    }
}

impl<T: Field + Copy> ShortSeq<T> {
    fn new(arr: [T; 16]) -> Self {
        Self {
            seq: arr,
            cnt: 16
        }
    }
    fn new_u32(arr: [u32; 16]) -> Self {
        let mut seq: [T; 16] = [Default::default(); 16];
        for (i, x) in seq.iter_mut().enumerate() {
            *x = T::from(arr[i]);
        }
        Self {
            seq: seq,
            cnt: 16
        }
    }
    #[inline]
    fn tail_term(self) -> Self {
        let mut found = false;
        let mut seq: [T; 16] = self.seq;
        for x in seq.iter_mut() {
            if *x == T::from(0) {
                continue;
            }
            if found {
                *x = T::from(0);
            } else {
                found = true;
            }
        }
        Self {
            seq: seq,
            cnt: self.cnt
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
        let catalan = ShortSeq::<ModIntP32>::new_u32([	1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845]);
        assert_eq!(catalan, (catalan * catalan).rshift() + ShortSeq::<ModIntP32>::promote(ModIntP32::from(1)));
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
        assert_eq!(geom.compose(nonzgeom), pow2);
        let mut expseq: [ModIntP32; 16] = [ModIntP32::from(1); 16];
        for i in 1..16 {
            expseq[i] = ModIntP32::from(i as u32) * expseq[i - 1];
        }
        for i in 0..16 {
            expseq[i] = ModIntP32::from(1) / expseq[i];
        }
        let a262 = ShortSeq::<ModIntP32>::new_u32([1, 1, 3, 13, 73, 501, 4051, 37633, 394353, 4596553, 58941091, 824073141, (12470162233u64 % 65521u64) as u32, (202976401213u64 % 65521u64) as u32, (3535017524403u64 % 65521u64) as u32, (65573803186921u64 % 65521u64) as u32]);
        let exp = ShortSeq::<ModIntP32>::new(expseq);
        assert_eq!(exp.compose(nonzgeom), a262.hadamard(exp));
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
        assert_eq!(atan.inverse(), tang.hadamard(exp));
    }

    #[test]
    fn sqrt_test() {
        let simple = ShortSeq::<ModIntP32>::new_u32([1, 11, 19, 23, 3, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(simple, (simple * simple).sqrt());
        let catalan = ShortSeq::<ModIntP32>::new_u32([1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845]);
        assert_eq!(catalan, catalan.lshift().sqrt());
    }
}
