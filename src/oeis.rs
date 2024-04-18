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

const UNOP_NUM: usize = 31;
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
        30 => (Some(a.exp_integ()), 150),
        _ => unreachable!()
    }
}

const PREOP_NUM: usize = 8;
fn ps_preop<P: PowerSeries>(mut a: P, i: usize) -> P {
    match i {
        0 => a,
        1 => a.lshift(),
        2 => a.lshift().lshift(),
        3 => a.rshift(),
        4 => a.rshift().rshift(),
        5 => a.lshift().rshift(),
        6 => { a[0] += P::Coeff::from(1); a },
        7 => { a[0] -= P::Coeff::from(1); a },
        _ => unreachable!()
    }
}

fn binop_format(a: &str, op: usize, b: &str) -> String {
    match op {
        0 => format!("({} + {})", a, b),
        1 => format!("({} - {})", a, b),
        2 => format!("({} * {})", a, b),
        3 => format!("({} / {})", a, b),
        4 => format!("({} / {})", b, a),
        5 => format!("{}({})", a, b),
        6 => format!("{}({})", b, a),
        7 => format!("({} .* {})", a, b),
        8 => format!("exp_mul({}, {})", a, b),
        9 => format!("dirichlet({}, {})", a, b),
        _ => unreachable!()
    }
}

const OPNAMES: &'static [&str] = &[
    "", "", "derive", "integrate", "point",
    "log_derive", "partial_sums", "partial_products",
    "delta", "binomial_transform", 
    "inverse_binomial_transform", "t019", "exp", "log",
    "laplace_transform", "inverse_laplace_transform",
    "boustrophedon_transform",
    "inverse_boustrophedon_transform",
    "mobius_transform", "inverse_mobius_transform",
    "stirling_transform", "inverse_stirling_transform",
    "euler_transform", "inverse_euler_transform",
    "lah_transform", "inverse_lah_transform",
    "sqrt", "cbrt", "", "", "exp_integrate"
];

fn unop_format(a: &str, op: usize) -> String {
    match op {
        0 => format!("{}", a),
        1 => format!("-{}", a),
        28 => format!("({})^2", a),
        29 => format!("({})^3", a),
        _ => format!("{}({})", OPNAMES[op], a),
    }
}

fn preop_format(a: &str, op: usize) -> String {
    match op {
        0 => format!("{}", a),
        1 => format!("({})/x", a),
        2 => format!("({})/x^2", a),
        3 => format!("({})*x", a),
        4 => format!("({})*x^2", a),
        5 => format!("({} - {}[0])", a, a),
        6 => format!("({} + 1)", a),
        7 => format!("({} - 1)", a),
        _ => unreachable!()
    }

}

#[derive(Debug, Default)]
pub struct SeqDB {
    short_map: BTreeMap<ShortSeq<MersP31>, u32>,
    short_vec: Vec<ShortSeq<MersP31>>,
    pub long_vec: Vec<Series>,
    pub a_to_ind: FxHashMap<u32, usize>,
    pub ind_to_a: Vec<u32>
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

// TODO: Make top shown editable
impl std::fmt::Display for TopResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..std::cmp::min(10, self.results.len()) {
            write!(f, "{}\n", self.results[i])?;
        }
        Ok(())
    }
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
            self.ind_to_a.push(anum);
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

    pub fn search_full(&self, long_inp: &Series) -> TopResults {
        let top = Mutex::new(TopResults::default());
        let short_inp = ShortSeq::<MersP31>::from_series(long_inp);
        for i in 0..PREOP_NUM {
            let pre_proc = ps_preop(short_inp, i);
            let trans_dat: Vec<(ShortSeq<MersP31>, i32, usize)> = (0..UNOP_NUM)
                .map(|i| (ps_unop(pre_proc, i), i))
                .filter(|x| x.0.0.is_some())
                .map(|x| (x.0.0.unwrap(), x.0.1, x.1))
                .filter(|x| Self::significant(x.0))
                .collect();
            for (t_seq, t_cost, t_ind) in &trans_dat {
                self.process_result(&top, *t_seq, long_inp, i, *t_cost, *t_ind, usize::MAX, usize::MAX);
            }
            (0..self.short_vec.len()).into_par_iter().for_each(|j| {
                for (t_seq, t_cost, t_ind) in &trans_dat {
                    for b_op in 0..BINOP_NUM {
                        self.process_result(&top, *t_seq, long_inp, i, *t_cost, *t_ind, j, b_op);
                    }
                }
            });
        }
        top.into_inner().unwrap()
    }

    fn pretty_result(&self, in_pre: usize, out_pre: usize, u_op: usize, seq: usize, b_op: usize, ind: usize) -> String {
        let p_format = preop_format("INPUT", in_pre);
        let u_format = unop_format(&p_format, u_op);
        let b_format = match seq {
            usize::MAX => u_format,
            _ => {
                let seqname = format!("A{}", self.ind_to_a[seq]);
                binop_format(&u_format, b_op, &seqname)
            }
        };
        format!("{} matches A{}", preop_format(&b_format, out_pre), self.ind_to_a[ind])
    }

    fn calculate_long(&self, long_inp: &Series, pre_op: usize, post_op: usize, u_op: usize, b_op: usize, seq: usize) -> Option<Series> {
        let long_preop = ps_preop(long_inp.clone(), pre_op);
        let Some(long_unop) = ps_unop(long_preop, u_op).0 else { return None; };
        let long_nres = match seq {
            usize::MAX => long_unop,
            _ => {
                match ps_binop(long_unop, &self.long_vec[seq], b_op).0 {
                    Some(v) => v,
                    None => { return None; }
                }
            }
        };
        Some(ps_preop(long_nres, post_op))
    }

    fn tautology(&self, pre_op: usize, post_op: usize, u_op: usize, b_op: usize, seq: usize) -> bool {
        let Some(l1) = self.calculate_long(&self.long_vec[0], pre_op, post_op, u_op, b_op, seq) else { return false; };
        l1.matches(&self.long_vec[0]) 
    }

    fn process_result(&self, top: &Mutex<TopResults>, pre_seq: ShortSeq<MersP31>, long_inp: &Series, pre_op: usize, pre_cost: i32, u_op: usize, seq: usize, b_op: usize) {
        let (res, b_cost) = match seq {
            usize::MAX => (pre_seq, 0),
            _ => {
                match ps_binop(pre_seq, &self.short_vec[seq], b_op) {
                    (Some(val), cost) => (val, cost),
                    (None, _) => { return; }
                }
            }
        };
        if !Self::significant(res) { return; }
        for k in 0..PREOP_NUM {
            let res2 = ps_preop(res, k);
            let inds = self.db_match_short(res2);
            if inds.len() == 0 { return; }
            let Some(long_res) = self.calculate_long(long_inp, pre_op, k, u_op, b_op, seq) else { return; };
            let base_score = (0..long_res.accuracy()).map(|i| std::cmp::min(10, long_res[i].clone().abs().to_f64().round() as i32) * 10).sum::<i32>();
            let cost_corr = - pre_cost - b_cost - (self.short_vec.len().checked_ilog2().unwrap() as i32) * 33;
            let dupl_corr = if seq != usize::MAX && long_res.matches(&self.long_vec[seq]) { -800 } else { 0 };
            let cur_score = base_score + cost_corr + dupl_corr;
            if cur_score < 0 { return; }
            for ind in inds {
                if self.long_vec[ind].matches(&long_res) {
                    top.lock().expect("Mutex failed").add_result(SearchResult {
                        score: cur_score,
                        series: long_res.clone(),
                        description: self.pretty_result(pre_op, k, u_op, seq, b_op, ind),
                    });
                }
            }
        }
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
