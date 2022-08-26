use crate::{ShortSeq, MersP31, PowerSeries, Series};
use crate::interpolate::{find_c_recursive, find_p_recursive};
use crate::mathtypes::{One, Zero};
use std::collections::BTreeMap;
use rustc_hash::FxHashMap;
use std::io::BufRead;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

const BINOP_NUM: usize = 10;
fn ps_binop<P: PowerSeries>(a: P, b: &P, i: usize) -> Option<P> {
    match i {
        0 => Some(a + b),
        1 => Some(a - b),
        2 => Some(a * b),
        3 => if b[0].is_zero() { None } else { Some(a / b) },
        4 => if a[0].is_zero() { None } else { Some(b.clone() / a) },
        5 => if !b[0].is_zero() { None } else { Some(a.compose(&b)) },
        6 => if !a[0].is_zero() { None } else { Some(b.compose(&a)) },
        7 => Some(a.hadamard(&b)),
        8 => Some(a.exp_mul(&b)),
        9 => Some(a.dirichlet(&b)),
        _ => unreachable!()
    }
}

const UNOP_NUM: usize = 30;
fn ps_unop<P: PowerSeries>(a: P, i: usize) -> Option<P> {
    match i {
        0 => Some(a),
        1 => Some(-a),
        2 => Some(a.derive()),
        3 => Some(a.integrate()),
        4 => Some(a.point()),
        5 => if a[0].is_zero() { None } else { Some(a.log_derive()) },
        6 => Some(a.partial_sums()),
        7 => Some(a.partial_products()),
        8 => Some(a.delta()),
        9 => Some(a.binomial()),
        10 => Some(a.binomial_inv()),
        11 => Some(a.t019()),
        12 => if !a[0].is_zero() { None } else { Some(a.exp()) },
        13 => if !a[0].is_zero() { None } else { Some(a.log()) },
        14 => Some(a.laplace()),
        15 => Some(a.laplace_inv()),
        16 => Some(a.bous()),
        17 => Some(a.bous_inv()),
        18 => Some(a.mobius()),
        19 => Some(a.mobius_inv()),
        20 => Some(a.stirling()),
        21 => Some(a.stirling_inv()),
        22 => Some(a.euler()),
        23 => Some(a.euler_inv()),
        24 => Some(a.lah()),
        25 => Some(a.lah_inv()),
        26 => if !a[0].is_one() { None } else { Some(a.sqrt()) },
        27 => if !a[0].is_one() { None } else { Some(a.ratpow(1, 3)) },
        28 => Some(a.pow(2)),
        29 => Some(a.pow(3)),
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

impl SeqDB {
    fn new() -> Self {
        Default::default()
    }

    fn significant(seq: ShortSeq<MersP31>) -> bool {
        if seq.accuracy() < 10 { return false; }
        let mut zprf = 0;
        while zprf < seq.accuracy() && seq[zprf].is_zero() { zprf += 1 };
        if zprf >= 4 { return false; }
        let mut vcnt = 0;
        for i in 0..seq.accuracy() { if !seq[i].is_zero() && !seq[i].is_one() { vcnt += 1; } }
        if vcnt < 6 { return false; }
        true
    }

    fn add_entry(&mut self, anum: u32, seq: &str) {
        let cur_ind = self.a_to_ind.len();
        let short_seq: ShortSeq<MersP31> = seq.parse().unwrap();
        if !Self::significant(short_seq) { return; }
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

    pub fn search_full(&self, inp: &str) {
        let short_inp: ShortSeq<MersP31> = inp.parse().unwrap();
        let long_inp: Series = inp.parse().unwrap();
        let trans_dat: Vec<(ShortSeq<MersP31>, usize)> = (0..UNOP_NUM)
            .map(|i| (ps_unop(short_inp, i), i))
            .filter(|x| x.0.is_some())
            .map(|x| (x.0.unwrap(), x.1))
            .filter(|(x, _i)| Self::significant(*x))
            .collect();
        (0..self.short_vec.len()).into_par_iter().for_each(|i| {
            for (t_seq, t_ind) in &trans_dat {
                for b_op in 0..BINOP_NUM {
                    let res = match ps_binop(*t_seq, &self.short_vec[i], b_op) {
                        Some(val) => val,
                        None => continue
                    };
                    if !Self::significant(res) { continue; }
                    let inds = self.db_match_short(res);
                    if inds.len() > 0 {
                        let long_unop = match ps_unop(long_inp.clone(), *t_ind) {
                            Some(v) => v,
                            None => continue
                        };
                        let long_res = match ps_binop(long_unop, &self.long_vec[i], b_op) {
                            Some(v) => v,
                            None => continue
                        };
                        for ind in inds {
                            if self.long_vec[ind].matches(&long_res) {
                                println!("unop-{} => {:?}", t_ind, ps_unop(long_inp.clone(), *t_ind).unwrap());
                                println!("op-{} w. {:?}\n=> {:?}\n", b_op, self.long_vec[i], long_res);
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn db_match_short(&self, mut short: ShortSeq<MersP31>) -> Vec<usize> {
        for i in short.accuracy()..16 {
            short[i] = MersP31::from(0u32);
        }
        let mut res = vec![];
        for (k, v) in self.short_map.range(short..) {
            if !short.matches(k) { break; }
            let ind = *self.a_to_ind.get(v).unwrap();
            res.push(ind);
        }
        res
    }

    pub fn db_interpolate_short(&self, short: ShortSeq<MersP31>) -> (Option<usize>, Option<(usize,usize)>) {
        let sig = short.accuracy().saturating_sub(4);
        if find_c_recursive(&short.seq[0..short.accuracy()], sig / 2).is_some() {
            return (Some(sig / 2), None);
        }
        let sig = short.accuracy().saturating_sub(4);
        for i in 1..sig/2+1 {
            for j in 1..sig/(i+2)+1 {
                if find_p_recursive(&short.seq[0..short.accuracy()], i, j).is_some() {
                    return (None, Some((i, j)));
                }
            }
        }
        (None, None)
    }
}
