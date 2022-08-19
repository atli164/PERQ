use crate::{ShortSeq, MersP31, PowerSeries, Series};
use crate::interpolate::{find_c_recursive, find_p_recursive};
use std::collections::BTreeMap;
use rustc_hash::FxHashMap;
use std::io::BufRead;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

const BINOP_NUM: usize = 10;
fn ps_binop<P: PowerSeries>(a: P, b: &P, i: usize) -> P {
    match i {
        0 => a + b,
        1 => a - b,
        2 => a * b,
        3 => a / b,
        4 => b.clone() / a,
        5 => a.compose(&b),
        6 => b.compose(&a),
        7 => a.hadamard(&b),
        8 => a.exp_mul(&b),
        9 => a.dirichlet(&b),
        _ => unreachable!()
    }
}

const UNOP_NUM: usize = 32;
fn ps_unop<P: PowerSeries>(a: P, i: usize) -> P {
    match i {
        0 => a,
        1 => -a,
        2 => a.derive(),
        3 => a.integrate(),
        4 => a.point(),
        5 => a.log_derive(),
        6 => a.partial_sums(),
        7 => a.partial_products(),
        8 => a.delta(),
        9 => a.binomial(),
        10 => a.binomial_inv(),
        11 => a.t019(),
        12 => a.exp(),
        13 => a.log(),
        14 => a.laplace(),
        15 => a.laplace_inv(),
        16 => a.bous(),
        17 => a.bous_inv(),
        18 => a.mobius(),
        19 => a.mobius_inv(),
        20 => a.stirling(),
        21 => a.stirling_inv(),
        22 => a.euler(),
        23 => a.euler_inv(),
        24 => a.powerset(),
        25 => a.cycle(),
        26 => a.lah(),
        27 => a.lah_inv(),
        28 => a.sqrt(),
        29 => a.ratpow(1, 3),
        30 => a.pow(2),
        31 => a.pow(3),
        _ => unreachable!()
    }
}

fn calc_ps<P: PowerSeries>(t1: usize, t2: usize, op1: usize, inp: &P, seq: &P) -> P {
    ps_unop(ps_binop(ps_unop(inp.clone(), t1), seq, op1), t2)
}

fn print_ps<P: PowerSeries>(t1: usize, t2: usize, op1: usize, inp: &P, seq: &P) {
    println!("UNOP-{} (\nBINOP-{} (\nUNOP-{} (\n{:?}\n),\n{:?}\n)\n)", t1, op1, t2, inp, seq);
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

    fn add_entry(&mut self, anum: u32, seq: &str) {
        let cur_ind = self.a_to_ind.len();
        let short_seq: ShortSeq<MersP31> = seq.parse().unwrap();
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
        
        let it = (0..UNOP_NUM)
            .into_par_iter()
            .flat_map(|i| { (0..self.short_vec.len()).map(|j| (i, j)).collect::<Vec<(usize,usize)>>() })
            .flat_map(|(i, j)| { (0..BINOP_NUM).map(|k| (i, j, k)).collect::<Vec<(usize,usize,usize)>>() })
            .flat_map(|(i, j, k)| { (0..UNOP_NUM).map(|l| (i, j, k, l)).collect::<Vec<(usize,usize,usize,usize)>>() });

        it.for_each(|(t1, seq, binop, t2)| {
            let cur = calc_ps(t1, t2, binop, &short_inp, &self.short_vec[seq]);
            let inds = self.db_match_short(cur);
            let ipol = self.db_interpolate_short(cur);
            if inds.len() > 0 || ipol.0.is_some() || ipol.1.is_some() {
                let long_inp: Series = inp.parse().unwrap();
                let long_cur = calc_ps(t1, t2, binop, &long_inp, &self.long_vec[seq]);
                for ind in inds {
                    if self.long_vec[ind].matches(&long_cur) {
                        print_ps(t1, t2, binop, &long_inp, &self.long_vec[seq]);
                        println!("mathces OEIS seq {}", ind);
                    }
                }
                if let Some(s) = ipol.0 {
                    if let Some(v) = find_c_recursive(long_cur.seq.as_slice(), s) {
                        print_ps(t1, t2, binop, &long_inp, &self.long_vec[seq]);
                        println!("matches C-recursive coeff:\n{:?}", v);
                    }
                }
                if let Some((s, t)) = ipol.1 {
                    if let Some(v) = find_p_recursive(long_cur.seq.as_slice(), s, t) {
                        print_ps(t1, t2, binop, &long_inp, &self.long_vec[seq]);
                        println!("matches D-finite coeff:");
                        for w in v {
                            println!("{:?}", w);
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
        let sig = short.accuracy().saturating_sub(2);
        if find_c_recursive(&short.seq[0..short.accuracy()], sig / 2).is_some() {
            return (Some(sig / 2), None);
        }
        let sig = short.accuracy() + 2;
        for i in 1..sig+1 {
            for j in 1..sig/(i+2)+1 {
                if find_p_recursive(&short.seq[0..short.accuracy()], i, j).is_some() {
                    return (None, Some((i, j)));
                }
            }
        }
        (None, None)
    }
}
