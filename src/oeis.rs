use crate::{ShortSeq, Field};
use crate::mathtypes::PowerSeries;
use crate::hashing::FastIntHashTable;
use std::io::{BufRead, Write};

#[derive(Debug, Default)]
pub struct ShortSeqDB<T: Field + Copy> {
    a_to_index: FastIntHashTable,
    anum: Vec<u32>,
    seqs: Vec<ShortSeq<T>>
}

impl<T: Field + std::str::FromStr + Ord + Copy> ShortSeqDB<T> {
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
            let mut vals = [T::default(); 16];
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
        let mut conn: Vec<(u32,u32)> = vec![];
        for i in 0..self.anum.len() {
            in_db.insert(self.seqs[i], i);
            conn.push((0, i as u32))
        }
        for i in 0..self.anum.len() {
            println!("{} / {}", i, self.anum.len());
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
                    conn[i].0 += 1;
                    conn[*j].0 += 1;
                }
            }
            for j in (i+1)..self.anum.len() {
                let sum = self.seqs[i] + self.seqs[j];
                let diff1 = self.seqs[i] - self.seqs[j];
                let diff2 = self.seqs[j] - self.seqs[i];
                let mul = self.seqs[i] * self.seqs[j];
                let hadam = self.seqs[i].hadamard(self.seqs[j]);
                let mut cand2 = vec![sum, diff1, diff2, mul, hadam];
                if self.seqs[i].seq[0] == T::from(0) {
                    cand2.push(self.seqs[j].compose(self.seqs[i]));
                } else {
                    cand2.push(self.seqs[j] / self.seqs[i]);
                }
                if self.seqs[j].seq[0] == T::from(0) {
                    cand2.push(self.seqs[i].compose(self.seqs[j]));
                } else {
                    cand2.push(self.seqs[i] / self.seqs[j]);
                }
                for seq in cand2 {
                    if let Some(k) = in_db.get(&seq) {
                        conn[i].0 += 1;
                        conn[j].0 += 1;
                        conn[*k].0 += 1;
                    }
                }
            }
        }
        conn.sort();
        let strings: Vec<String> = conn.iter().map(|(x, y)| x.to_string() + "," + &y.to_string()).collect();
        let mut file = std::fs::File::create("conn.txt")?;
        writeln!(file, "{}", strings.join(", "))?;
        Ok(())
    }
}
