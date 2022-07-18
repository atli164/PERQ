use crate::{ShortSeq, Field};
use crate::mathtypes::PowerSeries;
use crate::hashing::FastIntHashTable;
use std::io::{BufRead, Write};
use rayon::prelude::*;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Debug, Default)]
pub struct ShortSeqDB<T: Field + Copy> {
    a_to_index: FastIntHashTable,
    anum: Vec<u32>,
    seqs: Vec<ShortSeq<T>>
}

impl<T: Field + std::str::FromStr + Ord + Copy + std::marker::Sync> ShortSeqDB<T> {
    fn new() -> Self {
        Default::default()
    }

    fn add_entry(&mut self, a_num: u32, seq: ShortSeq<T>) {
        self.a_to_index.insert(a_num, self.seqs.len() as u32);
        self.seqs.push(seq);
        self.anum.push(a_num);
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
            let mut vals = [Default::default(); 16];
            let mut ind = 0;
            let mut iter = line.split(",");
            // Line starts with "A". Unwrap is OK since A-numbers do not overflow 32 bits
            let a_num = match iter.next() {
                Some(s) => u32::from_str_radix(&s.trim()[1..], 10).unwrap(),
                None => break
            };
            for s in iter {
                vals[ind as usize] = match T::from_str(s) {
                    Ok(val) => val,
                    _ => {
                        break;
                    }
                };
                ind += 1;
                if ind == 16 {
                    break;
                }
            }
            db.add_entry(a_num, ShortSeq {
                seq: vals,
                cnt: ind
            });
        }

        Ok(db)
    }

    pub fn connectivity(self) -> std::io::Result<()> {
        let mut in_db = std::collections::BTreeMap::<ShortSeq<T>, usize>::new();
        let mut conn = vec![];
        for i in 0..self.anum.len() {
            if self.seqs[i].seq.len() >= 10 {
                if !in_db.contains_key(&self.seqs[i]) {
                    in_db.insert(self.seqs[i].clone(), i);
                }
            }
            conn.push((AtomicU32::new(0), i as u32))
        }
        (0..self.anum.len()).into_par_iter().for_each(|i| {
            println!("{} / {}", i, self.anum.len());
            if self.seqs[i].cnt < 10 {
                return;
            }
            if *in_db.get(&self.seqs[i]).unwrap() != i {
                return;
            }
            let deriv = self.seqs[i].derive();
            let integ = self.seqs[i].integrate();
            let lshft = self.seqs[i].lshift();
            let rshft = self.seqs[i].rshift();
            let minus = -self.seqs[i];
            let mut cand1 = vec![deriv, integ, lshft, rshft, minus];
            if self.seqs[i].seq[0] == T::from(0) {
                cand1.push(self.seqs[i].inverse());
            }
            if self.seqs[i].seq[0] == T::from(1) {
                cand1.push(self.seqs[i].sqrt());
            }
            for seq in cand1 {
                if let Some(j) = in_db.get(&seq) {
                    conn[i].0.fetch_add(1, SeqCst);
                    conn[*j].0.fetch_add(1, SeqCst);
                }
            }
            for j in (i+1)..self.anum.len() {
                if self.seqs[j].seq.len() < 10 {
                    continue;
                }
                if *in_db.get(&self.seqs[j]).unwrap() != j {
                    continue;
                }
                let sum = self.seqs[i] + self.seqs[j];
                let diff1 = self.seqs[i] - self.seqs[j];
                let diff2 = self.seqs[j] - self.seqs[i];
                let mul = self.seqs[i] * self.seqs[j];
                let mut cand2 = vec![sum, diff1, diff2, mul];
                if self.seqs[i].seq[0] == T::from(0) {
                    cand2.push(self.seqs[j].compose(&self.seqs[i]));
                } else {
                    cand2.push(self.seqs[j] / self.seqs[i]);
                }
                if self.seqs[j].seq[0] == T::from(0) {
                    cand2.push(self.seqs[i].compose(&self.seqs[j]));
                } else {
                    cand2.push(self.seqs[i] / self.seqs[j]);
                }
                for seq in cand2 {
                    if let Some(k) = in_db.get(&seq) {
                        conn[i].0.fetch_add(1, SeqCst);
                        conn[j].0.fetch_add(1, SeqCst);
                        conn[*k].0.fetch_add(1, SeqCst);
                    }
                }
            }
        });
        let mut to_sort: Vec<(u32, u32)> = conn.iter().map(|(x, y)| (x.load(SeqCst), *y)).collect();
        to_sort.sort();
        let strings: Vec<String> = to_sort.iter().map(|(x, y)| x.to_string() + "," + &self.anum[*y as usize].to_string()).collect();
        let mut file = std::fs::File::create("conn.txt")?;
        writeln!(file, "{}", strings.join(", "))?;
        Ok(())
    }
}
