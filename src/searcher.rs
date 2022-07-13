use crate::{ShortSeq, Field, ModIntP32};
use crate::matrix::Matrix;
use std::collections::BTreeMap;

// Take input seq, perhaps as string to allow for big ints
// Start by searching with shortseq and modp32, possibly with < 16 precision
// Create exprtrees containing input seq, of course start with trivial one node tree
// Look up exprtree evals in database of oeis seqs
// Also try to interpolate evals against rational (Berlekamp-Massey) and P-recursive solvers
//
// All matches are fed into a double check that tests it against full given sequence with bigint
// Depending on the amount of false positives, maybe check against MersP61 first
// Results are then displayed with a confidence level
// Confidence level is calculated in terms of overfit degrees of freedom

struct Searcher {
    // Check later if fast hashes are better
    seq_db: BTreeMap<ShortSeq<ModIntP32>, u32>
}

// berlekamp-massey algorithm
fn find_c_recursive<T: Field + Copy>(seq: &Vec<T>, max_deg: usize) -> Option<Vec<T>> {
    let (mut m, mut l, mut b) = (1, 0, T::from(1u32));
    let (mut cp, mut bp, mut tmp, mut res) = (vec![T::from(1u32)], vec![T::from(1u32)], vec![], vec![]);
    for i in 0..seq.len() {
        if l > max_deg {
            return None;
        }
        let mut d = seq[i];
        for j in 1..l+1 {
            d = d + cp[j] * seq[i - j]; 
        }
        if d == T::from(0u32) {
            m += 1;
            continue;
        }
        let sw = 2 * l <= i;
        if sw {
            l = i + 1 - l;
            cp.resize_with(l + 1, Default::default);
            tmp = cp.clone();
        }
        let a = d / b;
        for j in m..cp.len() {
            cp[j] = cp[j] - a * bp[j - m];
        }
        m += 1;
        if sw {
            bp = tmp.clone();
            b = d;
            m = 1;
        }
    }
    for i in 1..l+1 {
        res.push(-cp[i]);
    }
    Some(res)
}

fn find_hypergeometric<T: Field + Copy>(seq: &Vec<T>, max_deg: usize) -> Option<(Vec<T>, Vec<T>)> {
    // P(n)a(n+1) = Q(n)a(n)
    // Normalize by setting coefficient sum of P to 1
    let coeff_num = std::cmp::min(max_deg + 1, seq.len() / 2);
    let mat_sz = 2 * coeff_num;
    let mut mat = Matrix::<T>::new(mat_sz, mat_sz);
    let mut targ = vec![];
    for i in 0..mat_sz-1 {
        let mut jpow = T::from(1u32);
        for j in 0..coeff_num {
            mat[(i, 2 * j)] = seq[i] * jpow;
            mat[(i, 2 * j + 1)] = -seq[i + 1] * jpow;
            jpow = jpow * T::from(i as u32);
        }
        targ.push(T::from(0u32));
    }
    for j in 0..coeff_num {
        mat[(mat_sz - 1, 2 * j + 1)] = T::from(1u32);
    }
    targ.push(T::from(1u32));
    let ret = mat.solve(&targ)?;
    let (mut p, mut q) = (vec![], vec![]);
    for i in 0..mat_sz {
        if i % 2 == 0 {
            q.push(ret[i]);
        } else {
            p.push(ret[i]);
        }
    }
    while p.len() > 1 && p.last() == Some(&T::from(0u32)) {
        p.pop();
    }
    while q.len() > 1 && q.last() == Some(&T::from(0u32)) {
        q.pop();
    }
    for i in 0..seq.len()-1 {
        let mut psm = T::from(0u32);
        for j in (0..p.len()).rev() {
            psm = psm * T::from(i as u32);
            psm = psm + p[j];
        }
        let mut qsm = T::from(0u32);
        for j in (0..q.len()).rev() {
            qsm = qsm * T::from(i as u32);
            qsm = qsm + q[j];
        }
        if psm * seq[i + 1] != qsm * seq[i] {
            return None;
        }
    }
    Some((p, q))
}

// fn find_p_recursive<T: Field + Copy>(seq: &Vec<T>, 

#[cfg(test)]
mod tests {
    use crate::ModIntP32;
    use crate::searcher::find_c_recursive;
    #[test]
    fn c_rec_test_1() {
        let mut fib = vec![];
        fib.push(ModIntP32::from(0u32));
        fib.push(ModIntP32::from(1u32));
        for i in 2..10 {
            fib.push(fib[i - 1] + fib[i - 2]);
        }
        let res = find_c_recursive::<ModIntP32>(&fib, 4);
        assert_eq!(res, Some(vec![ModIntP32::from(1u32), ModIntP32::from(1u32)]));
    }

    #[test]
    fn c_rec_test_2() {
        let mut seq = vec![];
        seq.push(ModIntP32::from(7u32));
        seq.push(ModIntP32::from(3u32));
        seq.push(ModIntP32::from(4u32));
        seq.push(ModIntP32::from(0u32));
        for i in 4..8 {
            seq.push(ModIntP32::from(5) * seq[i - 1] - seq[i - 2] - seq[i - 3] - seq[i - 3] + seq[i - 4]);
        }
        let res = find_c_recursive::<ModIntP32>(&seq, 4);
        assert_eq!(res, Some(vec![ModIntP32::from(5u32), -ModIntP32::from(1u32), -ModIntP32::from(2u32), ModIntP32::from(1u32)]));
    }

    #[test]
    fn c_rec_test_3() {
        let mut seq = vec![];
        seq.push(ModIntP32::from(2u32));
        seq.push(ModIntP32::from(1u32));
        seq.push(ModIntP32::from(3u32));
        seq.push(ModIntP32::from(2u32));
        for i in 4..8 {
            seq.push(ModIntP32::from(4) * seq[i - 1] + seq[i - 4]);
        }
        let res = find_c_recursive::<ModIntP32>(&seq, 3);
        assert_eq!(res, None);
    }

    use crate::searcher::find_hypergeometric;
    #[test]
    fn hyper_test_1() {
        let mut fac = vec![ModIntP32::from(1u32)];
        for i in 1..8 {
            fac.push(ModIntP32::from(i as u32) * fac[i - 1]);
        }
        let res = find_hypergeometric::<ModIntP32>(&fac, 2);
        let expected = (vec![ModIntP32::from(1u32)], vec![ModIntP32::from(1u32), ModIntP32::from(1u32)]);
        assert_eq!(res, Some(expected));
    }
    #[test]
    fn hyper_test_2() {
        let mut fac = vec![ModIntP32::from(1u32)];
        for i in 1..8 {
            fac.push(fac[i - 1] / ModIntP32::from(i as u32));
        }
        let res = find_hypergeometric::<ModIntP32>(&fac, 2);
        let half = ModIntP32::from(1u32) / ModIntP32::from(2u32);
        let expected = (vec![half, half], vec![half]);
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn hyper_test_3() {
        let mut seq = vec![ModIntP32::from(1u32)];
        let two = ModIntP32::from(2u32);
        for i in 1..10 {
            let n = ModIntP32::from(i as u32);
            seq.push(two * (two * n) * (two * n - ModIntP32::from(1u32)) * seq[i - 1]);
        }
        let res = find_hypergeometric::<ModIntP32>(&seq, 2);
        let expected = (vec![ModIntP32::from(1u32)], vec![ModIntP32::from(4u32), ModIntP32::from(12u32), ModIntP32::from(8u32)]);
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn hyper_test_4() {
        let mut seq = vec![ModIntP32::from(1u32)];
        let two = ModIntP32::from(2u32);
        for i in 1..10 {
            let n = ModIntP32::from(i as u32);
            seq.push(two * (two * n) * (two * n - ModIntP32::from(1u32)) * seq[i - 1]);
        }
        let res = find_hypergeometric::<ModIntP32>(&seq, 1);
        assert_eq!(res, None);
    }

    #[test]
    fn hyper_test_5() {
        // n!! is not hypergeometric
        let mut fac = vec![ModIntP32::from(1u32), ModIntP32::from(1u32)];
        for i in 2..16 {
            fac.push(ModIntP32::from(i as u32) * fac[i - 2]);
        }
        let res = find_hypergeometric::<ModIntP32>(&fac, 4);
        assert_eq!(res, None);
    }
}

