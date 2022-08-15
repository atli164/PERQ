use crate::{ShortSeq, ModIntP32, PowerSeries, Series};
use crate::interpolate::{find_c_recursive, find_p_recursive};
use std::collections::BTreeMap;
use rustc_hash::FxHashMap;
use std::io::BufRead;

macro_rules! transform_arr {
    ($x:ident) => {
        [
            $x.derive(),
            $x.integrate(),
            $x.inverse(),
            // and so on
        ]
    };
}

macro_rules! binop_arr {
    ($x:ident, $y:ident) => {
        [
            $x.compose(&$y),
            $y.compose(&$x),
            $x + $y,
            $x - $y,
            $y - $x,
            // and so on
        ]
    };
}

#[derive(Debug, Default)]
pub struct SeqDB {
    short_map: BTreeMap<ShortSeq<ModIntP32>, u32>,
    long_vec: Vec<Series>,
    a_to_ind: FxHashMap<u32, usize>
}

impl SeqDB {
    fn new() -> Self {
        Default::default()
    }

    fn add_entry(&mut self, anum: u32, seq: &str) {
        let cur_ind = self.a_to_ind.len();
        let short_seq: ShortSeq<ModIntP32> = seq.parse().unwrap();
        let long_seq: Series = seq.parse().unwrap();
        if self.short_map.get(&short_seq).is_none() {
            self.short_map.insert(short_seq, anum);
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

    pub fn match_seq(&self, mut short: ShortSeq<ModIntP32>, long: &Series) {
        for i in short.accuracy()..16 {
            short[i] = ModIntP32::from(0u32);
        }
        for (k, v) in self.short_map.range(short..) {
            if !short.matches(k) { break; }
            let ind = *self.a_to_ind.get(v).unwrap();
            if self.long_vec[ind].matches(long) {
                println!("Matches OEIS entry {}", v);
            }
        }
        let sig = short.accuracy().saturating_sub(2);
        if find_c_recursive(&short.seq[0..short.accuracy()], sig / 2).is_some() {
            if let Some(coeff) = find_c_recursive(long.seq.as_slice(), sig) {
                println!("Matches C-recursive with coefficients {:?}", coeff);
            }
        }
        let sig = short.accuracy() + 2;
        'outer: for i in 1..sig+1 {
            for j in 1..sig/(i+2)+1 {
                if find_p_recursive(&short.seq[0..short.accuracy()], i, j).is_some() {
                    if let Some(coeff) = find_p_recursive(long.seq.as_slice(), i, j) {
                        println!("Matches P-recursive with coefficients {:?}", coeff);
                        break 'outer;
                    }
                }
            }
        }
    }
}
