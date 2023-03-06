use crate::{ShortSeq, MersP31, PowerSeries, Series};
use crate::interpolate::{find_c_recursive, find_p_recursive};
use crate::mathtypes::{One, Zero};
use std::collections::BTreeMap;
use std::sync::Mutex;
use rustc_hash::FxHashMap;
use std::io::BufRead;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

const BINOP_NUM: usize = 10;
fn ps_binop<P: PowerSeries>(a: P, b: &P, i: usize) -> (Option<P>, i32) {
    match i {
        0 => (Some(a + b), 100),
        1 => (Some(a - b), 100),
        2 => (Some(a * b), 100),
        3 => (if b[0].is_zero() { None } else { Some(a / b) }, 150),
        4 => (if a[0].is_zero() { None } else { Some(b.clone() / a) }, 150),
        5 => (if !b[0].is_zero() { None } else { Some(a.compose(&b)) }, 200),
        6 => (if !a[0].is_zero() { None } else { Some(b.compose(&a)) }, 200),
        7 => (Some(a.hadamard(&b)), 200),
        8 => (Some(a.exp_mul(&b)), 200),
        9 => (Some(a.dirichlet(&b)), 200),
        _ => unreachable!()
    }
}

const UNOP_NUM: usize = 30;
fn ps_unop<P: PowerSeries>(a: P, i: usize) -> (Option<P>, i32) {
    match i {
        0 => (Some(a), 0),
        1 => (Some(-a), 50),
        2 => (Some(a.derive()), 100),
        3 => (Some(a.integrate()), 100),
        4 => (Some(a.point()), 100),
        5 => (if a[0].is_zero() { None } else { Some(a.log_derive()) }, 100),
        6 => (Some(a.partial_sums()), 200),
        7 => (Some(a.partial_products()), 200),
        8 => (Some(a.delta()), 150),
        9 => (Some(a.binomial()), 100),
        10 => (Some(a.binomial_inv()), 100),
        11 => (Some(a.t019()), 200),
        12 => (if !a[0].is_zero() { None } else { Some(a.exp()) }, 100),
        13 => (if !a[0].is_zero() { None } else { Some(a.log()) }, 100),
        14 => (Some(a.laplace()), 200),
        15 => (Some(a.laplace_inv()), 200),
        16 => (Some(a.bous()), 200),
        17 => (Some(a.bous_inv()), 200),
        18 => (Some(a.mobius()), 200),
        19 => (Some(a.mobius_inv()), 200),
        20 => (Some(a.stirling()), 150),
        21 => (Some(a.stirling_inv()), 150),
        22 => (Some(a.euler()), 150),
        23 => (Some(a.euler_inv()), 150),
        24 => (Some(a.lah()), 200),
        25 => (Some(a.lah_inv()), 200),
        26 => (if !a[0].is_one() { None } else { Some(a.sqrt()) }, 150),
        27 => (if !a[0].is_one() { None } else { Some(a.ratpow(1, 3)) }, 200),
        28 => (Some(a.pow(2)), 100),
        29 => (Some(a.pow(3)), 150),
        _ => unreachable!()
    }
}


#[derive(Debug, Default)]
pub struct SeqDB {
    short_map: BTreeMap<ShortSeq<MersP31>, u32>,
    short_vec: Vec<ShortSeq<MersP31>>,
    long_vec: Vec<Series>,
    a_to_ind: FxHashMap<u32, usize>
}

#[derive(Debug)]
pub struct SearchResult {
    score: i32,
    series: Series,
    description: String
}

impl std::fmt::Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.score, self.description)
    }
}

#[derive(Debug, Default)]
pub struct TopResults {
    results: Vec<SearchResult>
}

impl TopResults {
    fn add_result(&mut self, result: SearchResult) {
        self.results.push(result);
        self.results.sort_by(|a, b| b.score.cmp(&a.score));
        if self.results.len() > 10 { self.results.pop(); }
    }
}

impl std::fmt::Display for TopResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..10 {
            write!(f, "{}\n", self.results[i])?;
        }
        Ok(())
    }
}

impl SeqDB {
    fn new() -> Self {
        Default::default()
    }

    fn add_entry(&mut self, anum: u32, seq: &str) {
        let cur_ind = self.a_to_ind.len();
        let short_seq: ShortSeq<MersP31> = seq.parse().unwrap();
        if short_seq.accuracy() < 10 { return; }
        let long_seq: Series = seq.parse().unwrap();
        if self.short_map.get(&short_seq).is_none() {
            self.short_map.insert(short_seq, anum);
            self.short_vec.push(short_seq);
            self.long_vec.push(long_seq);
            self.a_to_ind.insert(anum, cur_ind);
        }
    }

    pub fn from_stripped(filename: String) -> std::io::Result<Self> {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);
        let mut db = Self::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') {
                continue;
            }
            let (apart, seqpart) = line.split_once(" ,").unwrap();
            let anum: u32 = apart[1..].trim_start_matches('0').parse().unwrap();
            db.add_entry(anum, &seqpart[..seqpart.len() - 1]);
        }

        Ok(db)
    }

    pub fn search_full(&self, inp: &str) -> TopResults {
        let top = Mutex::new(TopResults::default());
        let short_inp: ShortSeq<MersP31> = inp.parse().unwrap();
        let long_inp: Series = inp.parse().unwrap();
        let trans_dat: Vec<(ShortSeq<MersP31>, i32, usize)> = (0..UNOP_NUM)
            .map(|i| (ps_unop(short_inp, i), i))
            .filter(|x| x.0.0.is_some())
            .map(|x| (x.0.0.unwrap(), x.0.1, x.1))
            .collect();
        (0..self.short_vec.len()).into_par_iter().for_each(|i| {
            for (t_seq, t_cost, t_ind) in &trans_dat {
                for b_op in 0..BINOP_NUM {
                    let (res, b_cost) = match ps_binop(*t_seq, &self.short_vec[i], b_op) {
                        (Some(val), cost) => (val, cost),
                        (None, _) => continue
                    };
                    let inds = self.db_match_short(res);
                    if inds.len() > 0 {
                        let long_unop = match ps_unop(long_inp.clone(), *t_ind).0 {
                            Some(v) => v,
                            None => continue
                        };
                        let long_res = match ps_binop(long_unop, &self.long_vec[i], b_op).0 {
                            Some(v) => v,
                            None => continue
                        };
                        let score = (0..long_res.accuracy()).map(|i| std::cmp::min(10, long_res[i].clone().abs().to_f64().round() as i32) * 10).sum::<i32>()
                            - t_cost - b_cost - (self.short_vec.len().checked_ilog2().unwrap() as i32) * 33;
                        if score < 0 {
                            continue;
                        }
                        for ind in inds {
                            if self.long_vec[ind].matches(&long_res) {
                                let s1 = format!("unop-{} => {:?}", t_ind, ps_unop(long_inp.clone(), *t_ind).0.unwrap());
                                let s2 = format!("op-{} w. {:?}\n=> {:?}\n", b_op, self.long_vec[i], long_res);
                                top.lock().expect("Mutex failed").add_result(SearchResult {
                                    score: score,
                                    series: long_res.clone(),
                                    description: s1 + "\n" + &s2
                                });
                            }
                        }
                    }
                }
            }
        });
        top.into_inner().unwrap()
    }

    pub fn db_match_short(&self, mut short: ShortSeq<MersP31>) -> Vec<usize> {
        for i in short.accuracy()..16 {
            short[i] = MersP31::from(0u32);
        }
        let mut res = vec![];
        for (k, v) in self.short_map.range(short..) {
            if short.matches(k) { break; }
            let ind = *self.a_to_ind.get(v).unwrap();
            res.push(ind);
        }
        res
    }

    pub fn db_interpolate_short_c(&self, short: ShortSeq<MersP31>) -> Option<usize> {
        let sig = short.accuracy().saturating_sub(4);
        if find_c_recursive(&short.seq[0..short.accuracy()], sig / 2).is_some() {
            return Some(sig / 2 + 1);
        }
        let sig = short.accuracy().saturating_sub(4);
        for i in 1..sig/2+1 {
            for j in 1..sig/(i+2)+1 {
                if find_p_recursive(&short.seq[0..short.accuracy()], i, j).is_some() {
                    return Some(j * (i + 2));
                }
            }
        }
        None
    }
}
