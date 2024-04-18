use crate::matrix::Matrix;
use crate::mathtypes::Zero;
use rug::{Integer, Complete};
use rug::ops::{DivRounding, NegAssign};

fn size_reduce_vector(b: &mut Matrix<f64>, gamma: &mut Matrix<f64>, k: usize, j: usize) {
    let theta: f64 = gamma[(j, k)].round();
    for i in 0..b.shape().0 {
        b[(i, k)] -= theta * b[(i, j)];
    }
    for i in 0..j+1 {
        gamma[(i, k)] -= theta * gamma[(i, j)];
    }
}

fn size_reduce_basis(b: &mut Matrix<f64>, gamma: &mut Matrix<f64>, n: usize, k: usize) {
    for j in (k..n).rev() {
        for i in (0..j).rev() {
            size_reduce_vector(b, gamma, j, i);
        }
    }
}

fn lll_swap(b: &mut Matrix<f64>, gamma: &mut Matrix<f64>, bstar: &mut Vec<f64>, k: usize) {
    for j in 0..b.shape().0 {
        b.swap((j, k - 1), (j, k));
    }
    let mu = gamma[(k - 1, k)];
    let nu = bstar[k] + mu.powi(2) * bstar[k - 1];
    gamma[(k - 1, k)] = mu * bstar[k - 1] / nu;
    bstar[k] *= bstar[k - 1] / nu;
    bstar[k - 1] = nu;
    for j in 0..k-1 {
        gamma.swap((j, k - 1), (j, k));
    }
    for j in k+1..gamma.shape().1 {
        let a = gamma[(k - 1, k)] * gamma[(k - 1, j)] + (1.0 - mu * gamma[(k - 1, k)]) * gamma[(k, j)];
        let b = gamma[(k - 1, j)] - mu * gamma[(k, j)];
        gamma[(k - 1, j)] = a;
        gamma[(k, j)] = b;
    }
}

fn gram_schmidt(mut b: Matrix<f64>) -> (Matrix<f64>, Vec<f64>) {
    let (m, n) = b.shape();
    let mut gamma = Matrix::<f64>::new(n, n);
    for i in 0..n {
        gamma[(i, i)] = 1.0;
        for j in 0..i {
            let (mut idot, mut jdot) = (0.0, 0.0);
            for k in 0..m {
                idot += b[(k, i)] * b[(k, j)];
                jdot += b[(k, j)] * b[(k, j)];
            }
            gamma[(j, i)] = idot / jdot;
            for k in 0..m {
                b[(k, i)] -= gamma[(j, i)] * b[(k, j)];
            }
        }
    }
    let mut bstar = vec![0.0; n];
    for i in 0..n {
        for j in 0..m {
            bstar[i] += b[(j, i)] * b[(j, i)];
        }
    }
    (gamma, bstar)
}

pub fn float_lll(b: &mut Matrix<f64>) {
    let n = b.shape().1;
    let (mut gamma, mut bstar) = gram_schmidt(b.clone());
    size_reduce_basis(b, &mut gamma, n, 0);
    let mut k: usize = 1;
    while k < n {
        if bstar[k] < (0.75 - gamma[(k - 1, k)].powi(2)) * bstar[k - 1] {
            lll_swap(b, &mut gamma, &mut bstar, k);
            size_reduce_basis(b, &mut gamma, n, k);
            if k > 1 { k -= 1; }
        } else { k += 1; }
    }
}

fn ff_gaussian(mut b: Matrix<Integer>) -> Matrix<Integer> {
    let (m, n) = b.shape();
    let mut div = Integer::from(1);
    let mut r: usize = 0;
    for k in 0..n {
        let mut p = r;
        while p < m && b[(p, k)].is_zero() {
            p += 1;
        }
        if p < m {
            for j in k..n {
                b.swap((p, j), (r, j));
            }
            for i in r+1..m {
                for j in k+1..n {
                    b[(i, j)] = (b[(r, k)].clone() * &b[(i, j)] - b[(r, j)].clone() * &b[(i, k)]).div_exact(&div);
                }
                b[(i, k)] = Integer::zero();
            }
            div = b[(r, k)].clone();
            r += 1;
        }
        if r >= m { break; }
    }
    b
}

fn ff_subtract(a: &mut Matrix<Integer>, t: &mut Matrix<Integer>, k: usize, r: usize, q: &Integer) {
    for i in 0..a.shape().1 {
        let sub = a[(r, i)].clone() * q;
        a[(k, i)] -= sub;
    }
    for i in 0..t.shape().0 {
        let sub = t[(i, r)].clone() * q;
        t[(i, k)] -= sub;
    }
}

fn ff_swap(a: &mut Matrix<Integer>, t: &mut Matrix<Integer>, k: usize) {
    for i in 0..a.shape().1 {
        a.swap((k, i), (k - 1, i));
    }
    for i in 0..t.shape().1 {
        let ml = if k == 1 { Integer::from(1) } else { t[(k - 2, k - 2)].clone() };
        println!("{}", (ml.clone() * &t[(k, i)] + t[(k - 1, k)].clone() * &t[(k - 1, i)]) % &t[(k - 1, k - 1)]);
        println!("");
        t[(k, i)] = (ml * &t[(k, i)] + t[(k - 1, k)].clone() * &t[(k - 1, i)]).div_exact(&t[(k - 1, k - 1)]);
    }
    for i in 0..t.shape().1 {
        t.swap((k, i), (k - 1, i));
    }
    for i in 0..t.shape().0 {
        t.swap((i, k), (i, k - 1));
    }
    for i in 0..t.shape().1 {
        let dv = if k == 1 { Integer::from(1) } else { t[(k - 2, k - 2)].clone() };
        t[(k, i)] = (t[(k - 1, k - 1)].clone() * &t[(k, i)] + t[(k - 1, k)].clone() * &t[(k - 1, i)]).div_exact(&dv);
    }
}

fn mod_subtract(a: &mut Matrix<Integer>, t: &mut Matrix<Integer>, m: &Integer, k: usize, r: usize, q: &Integer) {
    ff_subtract(a, t, k, r, q);
    for i in 0..k {
        let mi = if i == 0 { t[(i, i)].clone() * m } else { t[(i, i)].clone() * &t[(i - 1, i - 1)] * m };
        t[(i, k)] %= mi;
    }
    for j in 0..a.shape().1 {
        a[(k, j)] %= m;
    }
}

fn mod_swap(a: &mut Matrix<Integer>, t: &mut Matrix<Integer>, m: &Integer, k: usize) {
    ff_swap(a, t, k);
    for i in 0..k-1 {
        let mi = if i == 0 { t[(i, i)].clone() * m } else { t[(i, i)].clone() * &t[(i - 1, i - 1)] * m };
        t[(i, k - 1)] %= mi;
    }
    for i in 0..k {
        let mi = if i == 0 { t[(i, i)].clone() * m } else { t[(i, i)].clone() * &t[(i - 1, i - 1)] * m };
        t[(i, k)] %= mi;
    }
    let m1 = if k == 1 { t[(k - 1, k - 1)].clone() * m } else { t[(k - 2, k - 2)].clone() * &t[(k - 1, k - 1)] * m };
    for j in k..t.shape().1 {
        t[(k - 1, j)] %= &m1;
    }
    let m2 = t[(k - 1, k - 1)].clone() * &t[(k, k)] * m;
    for j in k+1..t.shape().1 {
        t[(k, j)] %= &m2;
    }
}

pub fn modular_lll(a: &mut Matrix<Integer>) {
    let (n, _m) = a.shape();
    let mut t = ff_gaussian(&*a * &a.transpose());
    println!("{}", t);
    let mut m = t[(0, 0)].clone();
    for i in 1..n {
        let a_i = t[(i, i)].clone().div_ceil(&t[(i - 1, i - 1)]);
        if a_i > m { m = a_i; }
    }
    m *= n;
    m.sqrt_mut();
    m *= 2;
    m += 2;
    let mut k: usize = 1;
    loop {
        let mu = t[(k - 1, k)].div_rem_round_ref(&t[(k - 1, k - 1)]).complete().0;
        mod_subtract(a, &mut t, &m, k, k - 1, &mu);
        let lhs = if k == 1 { t[(k, k)].clone() * &t[(k - 1, k - 1)] * 4 } else { t[(k, k)].clone() * &t[(k - 1, k - 1)] * &t[(k - 2, k - 2)] * 4 };
        let rhs = (t[(k - 1, k - 1)].clone().square() * 3 - t[(k - 1, k)].clone().square() * 4) * &t[(k - 1, k - 1)];
        if lhs < rhs {
            println!("swp k={}", k);
            mod_swap(a, &mut t, &m, k);
            if k > 1 { k -= 1; }
        } else {
            k += 1;
            if k == n { break; }
        }
    }
    /*
    let mut m = Integer::new();
    loop {
        m = t[(0, 0)].clone();
        for i in 1..n {
            let a_i = t[(i, i)].clone().div_ceil(&t[(i - 1, i - 1)]);
            if a_i > m { m = a_i; }
        }
        m *= n;
        m.sqrt_mut();
        m *= 2;
        m += 2;
        m.next_power_of_two_mut();
        let mut k: usize = 1;
        for i in 2..n {
            let lhs =  t[(k - 1, k - 1)].clone().square() * &t[(i, i)] * &t[(i - 2, i - 2)];
            let rhs = if k == 1 { t[(i - 1, i - 1)].clone().square() * &t[(k, k)] } else { t[(i - 1, i - 1)].clone().square() * &t[(k, k)] * &t[(k - 2, k - 2)] };
            if lhs < rhs { k = i; }
        }
        let lhs = if k == 1 { t[(k, k)].clone() * 2 } else { t[(k, k)].clone() * &t[(k - 2, k - 2)] * 2 };
        let rhs = t[(k - 1, k - 1)].clone().square();
        if lhs >= rhs { break; }
        let mu = t[(k - 1, k)].div_rem_round_ref(&t[(k - 1, k - 1)]).complete().0;
        mod_subtract(a, &mut t, &m, k, k - 1, &mu);
        mod_swap(a, &mut t, &m, k);
    }
    */
    for k in 1..n {
        for j in (0..k).rev() {
            let mu = t[(j, k)].div_rem_round_ref(&t[(j, j)]).complete().0;
            mod_subtract(a, &mut t, &m, k, j, &mu);
        }
    }
}

pub fn hermite_normal_form(g: &Matrix<Integer>) -> Matrix<Integer> {
    let (n, m) = g.shape();
    let mut b = Matrix::<Integer>::new(m, m);
    for i in 0..m {
        b[(i, i)] = Integer::from(1);
    }
    let mut lam = Matrix::<Integer>::new(m, m);
    let mut a = g.clone();
    let mut d = vec![Integer::from(1); m + 1];
    let (m1, n1) = (3, 4);
    let mut nonz_row = 0;
    while (0..m).all(|j| a[(nonz_row, j)].is_zero()) {
        nonz_row += 1;
        if nonz_row == n {
            return a;
        }
    }
    let nonz_inds: Vec<usize> = (0..m).filter(|&j| !a[(nonz_row, j)].is_zero()).collect();
    if nonz_inds[0] == m - 1 && a[(nonz_row, m - 1)] < 0 {
        for j in 0..n {
            a[(j, m - 1)].neg_assign();
        }
        b[(m - 1, m - 1)] = Integer::from(-1);
    }
    let mut k: usize = 1;
    while k < m {
        let (row1, row2) = hermite_reduce(&mut a, &mut b, &mut lam, &d, k, k - 1);
        if (row1 <= row2 && row1 < n) || (row1 == n && row2 == n && n1 * (d[k - 1].clone() * &d[k + 1] + lam[(k - 1, k)].square_ref()) < m1 * d[k].clone().square()) {
            hermite_swap(&mut a, &mut b, &mut lam, &mut d, k);
            if k >= 2 {
                k -= 1;
            }
        } else {
            for i in (0..k-1).rev() {
                hermite_reduce(&mut a, &mut b, &mut lam, &d, k, i);
            }
            k += 1;
        }
    }
    a
}

fn hermite_swap(a: &mut Matrix<Integer>, b: &mut Matrix<Integer>, lam: &mut Matrix<Integer>, d: &mut Vec<Integer>, k: usize) {
    for i in 0..a.shape().0 {
        a.swap((i, k), (i, k - 1));
    }
    for i in 0..b.shape().0 {
        b.swap((i, k), (i, k - 1));
    }
    for j in 0..k-1 {
        lam.swap((j, k), (j, k - 1));
    }
    for i in k+1..b.shape().0 {
        let t = lam[(k - 1, i)].clone() * &d[k + 1] - lam[(k, i)].clone() * &lam[(k - 1, k)];
        lam[(k - 1, i)] = (lam[(k - 1, i)].clone() * &lam[(k - 1, k)] + lam[(k, i)].clone() * &d[k - 1]).div_exact(&d[k]);
        lam[(k, i)] = t.div_exact(&d[k]);
    }
    d[k] = (d[k - 1].clone() * &d[k + 1] + lam[(k - 1, k)].square_ref()).div_exact(&d[k]);
}

fn hermite_reduce(a: &mut Matrix<Integer>, b: &mut Matrix<Integer>, lam: &mut Matrix<Integer>, d: &Vec<Integer>, k: usize, i: usize) -> (usize, usize) {
    let row1 = match (0..a.shape().0).find(|&j| !a[(j, i)].is_zero()) {
        Some(x) => {
            if a[(x, i)] < 0 {
                for k in 0..lam.shape().0 {
                    if i > k {
                        lam[(i, k)].neg_assign();
                    }
                    if k > i {
                        lam[(k, i)].neg_assign();
                    }
                }
                for j in 0..a.shape().0 {
                    a[(j, i)].neg_assign();
                }
                for j in 0..b.shape().0 {
                    b[(j, i)].neg_assign();
                }
            }
            x
        },
        None => a.shape().0
    };
    let row2 = match (0..a.shape().0).find(|&j| !a[(j, k)].is_zero()) {
        Some(x) => x,
        None => a.shape().0
    };
    let q = if row1 < a.shape().0 {
        a[(row1, k)].clone().div_floor(&a[(row1, i)])
    } else {
        if 2 * lam[(i, k)].clone().abs() > d[i + 1] {
            lam[(i, k)].clone().div_rem_round_ref(&d[i + 1]).complete().0
        } else {
            Integer::from(0)
        }
    };
    if q != 0 {
        for j in 0..a.shape().0 {
            let sub = a[(j, i)].clone() * &q;
            a[(j, k)] -= sub;
        }
        for j in 0..b.shape().0 {
            let sub = b[(j, i)].clone() * &q;
            b[(j, k)] -= sub;
        }
        let sub = d[i + 1].clone() * &q;
        lam[(i, k)] -= sub;
        for j in 0..i {
            let sub = lam[(j, i)].clone() * &q;
            lam[(j, k)] -= sub
        }
    }
    (row1, row2)
}

#[cfg(test)]
mod tests {
    use crate::lll;
    use rug::Integer;
    use crate::Matrix;

    #[test]
    fn test_hermite() {
        let mut g = Matrix::<Integer>::new(10, 10);
        for i in 1..11 {
            for j in 1..11 {
                g[(i - 1, j - 1)] = Integer::from(i * i * j * j * j + i + j);
            }
        }
        let a = lll::hermite_normal_form(&g);
        let expected_a = "[[0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
                           [0, 0, 0, 0, 0, 0, 0, 0, 1, 0], 
                           [0, 0, 0, 0, 0, 0, 0, 12, 4, 7], 
                           [0, 0, 0, 0, 0, 0, 0, 36, 9, 22], 
                           [0, 0, 0, 0, 0, 0, 0, 72, 16, 45], 
                           [0, 0, 0, 0, 0, 0, 0, 120, 25, 76], 
                           [0, 0, 0, 0, 0, 0, 0, 180, 36, 115], 
                           [0, 0, 0, 0, 0, 0, 0, 252, 49, 162], 
                           [0, 0, 0, 0, 0, 0, 0, 336, 64, 217], 
                           [0, 0, 0, 0, 0, 0, 0, 432, 81, 280]]".parse().unwrap();
        assert!(a == expected_a);
    }

    #[test]
    fn test_ff_reduction() {
        let a: Matrix<Integer> = "[[ 3,  4, -2,  1, -2],
                                   [ 1, -1,  2,  2,  7],
                                   [ 4, -3,  4, -3,  2],
                                   [-1,  1,  6, -1,  1]]".parse().unwrap();
        let b = lll::ff_gaussian(a.clone());
        let expected_b = "[[3, 4, -2, 1, -2],
                           [0, -7, 8, 5, 23].
                           [0, 0, 20, 72, 159],
                           [0, 0, 0, -556, -1112]]".parse().unwrap();
        assert!(b == expected_b);
    }
}
