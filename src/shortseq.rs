use std::fmt;
use std::ops::{Add, Sub, Mul};
use crate::hashing::{FastIntHashTable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShortSeq {
    pub seq: [i32; 16],
}

impl fmt::Display for ShortSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut comma_separated = String::new();

        for i in 0..15 {
            comma_separated.push_str(&self.seq[i].to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.seq[15].to_string());

        write!(f, "{}", comma_separated)
    }
}

impl Add for ShortSeq {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut seq: [i32; 16] = [0; 16];
        for i in 0..16 {
            seq[i] = self.seq[i].wrapping_add(other.seq[i])
        }
        Self {
            seq
        }
    }
}

impl Sub for ShortSeq {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut seq: [i32; 16] = [0; 16];
        for i in 0..16 {
            seq[i] = self.seq[i].wrapping_sub(other.seq[i])
        }
        Self {
            seq
        }
    }
}


impl Mul for ShortSeq {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut seq: [i32; 16] = [0; 16];
        for i in 0..16 {
            for j in 0..16-i {
                seq[i + j] = seq[i + j].wrapping_add(self.seq[i].wrapping_mul(other.seq[j]));
            }
        }
        Self {
            seq
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ShortSeqDB {
    entry_num: u32,
    a_to_index: FastIntHashTable,
    seqs: Vec<ShortSeq>
}

impl ShortSeqDB {
    pub fn new() -> ShortSeqDB {
        Default::default()
    }

    pub fn add_entry(&mut self, seq: ShortSeq, a_index: u32) -> Result<(), ()> {
        self.a_to_index.insert(self.entry_num, a_index);
        self.seqs.push(seq);
        self.entry_num += 1;
        Ok(())
    }
}
