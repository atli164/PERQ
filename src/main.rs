use std::io;
use std::fs;
mod hashing;
use std::io::BufRead;
use num::bigint::BigInt;
mod shortseq;
mod modulo;
mod genseqs;
use shortseq::ShortSeq;
use modulo::ModInt;
use std::ops::{Add, Div, Sub, Mul};
use std::time;


fn stripped_to_db(filename: String) -> io::Result<shortseq::ShortSeqDB> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut db = shortseq::ShortSeqDB::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let mut vals = [0; 16];
        let mut ind = 0;
        let mut flags = 0;
        let mut iter = line.split(",");
        // Line starts with "A". Unwrap is OK since A-numbers do not overflow 32 bits
        let a_num = match iter.next() {
            Some(s) => u32::from_str_radix(&s.trim()[1..], 10).unwrap(),
            None => break
        };
        for s in iter {
            vals[ind as usize] = match i32::from_str_radix(s, 10) {
                Ok(val) => val,
                _ => {
                    // TODO: Add mod i32 functionality
                    break;
                }
            };
            ind += 1;
            if ind == 15 {
                break;
            }
        }
        if ind < 15 {
            continue;
        }
        db.add_entry(ShortSeq {
            seq: vals,
        }, a_num).unwrap();
    }

    Ok(db)
}


fn main() {
    let res = genseqs::oeis::<BigInt>(45).unwrap();
    for i in 0..100 {
        println!("{}", res.coeff(i));
    }
}
