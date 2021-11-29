use std::fmt;
use std::io;
use std::fs;
use pserq::Serializable;
use pserq::HashTable;
use std::io::BufRead;
use num_bigint::BigInt;

pub mod seq_flags {
    pub const VALUES_MISSING: u8 = 0x01;
    pub const VALUE_OVERFLOW: u8 = 0x02;
    // capacity for more flags
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ShortSeq {
    // Put vals at top so derived ord prioritizes it
    vals: [i64; 15],
    a_number: u32,
    val_num: u8,
    flags: u8,
}

#[derive(Debug)]
struct FullSeq {
    a_number: u32,
    val_num: u32,
    vals: Vec<BigInt>,
}

impl Serializable for ShortSeq {
    fn serialize<S: io::Write>(&self, ostr: &mut S) -> io::Result<()> {
        ostr.write(&self.a_number.to_be_bytes())?;
        ostr.write(&self.val_num.to_be_bytes())?;
        ostr.write(&self.flags.to_be_bytes())?;
        for i in 0..15 {
            ostr.write(&self.vals[i as usize].to_be_bytes())?;
        }
        Ok(())
    }

    fn deserialize<S: io::Read>(istr: &mut S) -> io::Result<ShortSeq> {
        let mut buf1: [u8; 1] = [0; 1];
        let mut buf4: [u8; 4] = [0; 4];
        let mut buf8: [u8; 8] = [0; 8];
        istr.read(&mut buf4)?;
        let a_number = u32::from_be_bytes(buf4);
        istr.read(&mut buf1)?;
        let val_num = u8::from_be_bytes(buf1);
        istr.read(&mut buf1)?;
        let flags = u8::from_be_bytes(buf1);
        let mut vals: [i64; 15] = [0; 15];
        for i in 0..15 {
            istr.read(&mut buf8)?;
            vals[i] = i64::from_be_bytes(buf8);
        }
        Ok(ShortSeq {
            a_number,
            val_num,
            flags,
            vals
        })
    }
}

impl fmt::Display for ShortSeq {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut comma_separated = String::new();

        for i in 0..self.val_num-1 {
            comma_separated.push_str(&self.vals[i as usize].to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.vals[(self.val_num - 1) as usize].to_string());
        write!(f, "{}", comma_separated)
    }
}

#[derive(Debug, Default, PartialEq)]
struct ShortSeqDB {
    entry_num: u32,
    a_to_index: HashTable,
    seqs: Vec<ShortSeq>
}

impl ShortSeqDB {
    fn new() -> ShortSeqDB {
        Default::default()
    }

    fn add_entry(&mut self, seq: ShortSeq) {
        self.a_to_index.insert(seq.a_number, self.entry_num);
        self.seqs.push(seq);
        self.entry_num += 1;
    }
}

impl Serializable for ShortSeqDB {
    fn serialize<S: io::Write>(&self, ostr: &mut S) -> io::Result<()> {
        ostr.write(&self.entry_num.to_be_bytes())?;
        self.a_to_index.serialize(ostr)?;
        for seq in &self.seqs {
            seq.serialize(ostr)?;
        }
        Ok(())
    }

    fn deserialize<S: io::Read>(istr: &mut S) -> io::Result<ShortSeqDB> {
        let mut buf4: [u8; 4] = [0; 4];
        istr.read(&mut buf4)?;
        let entry_num = u32::from_be_bytes(buf4);
        let a_to_index = HashTable::deserialize(istr)?;
        let mut seqs = Vec::with_capacity(entry_num as usize);
        for _i in 0..entry_num {
            seqs.push(ShortSeq::deserialize(istr)?);
        }
        Ok(ShortSeqDB {
            entry_num,
            a_to_index,
            seqs
        })
    }
}

fn stripped_to_db(filename: String) -> io::Result<ShortSeqDB> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut db = ShortSeqDB::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let mut vals = [0; 15];
        let mut ind = 0;
        let mut flags = 0;
        let mut iter = line.split(",");
        // Line starts with "A". Unwrap is OK since A-numbers do not overflow 32 bits
        let a_num = match iter.next() {
            Some(s) => u32::from_str_radix(&s.trim()[1..], 10).unwrap(),
            None => break
        };
        for s in iter {
            vals[ind as usize] = match i64::from_str_radix(s, 10) {
                Ok(val) => val,
                _ => {
                    flags |= seq_flags::VALUE_OVERFLOW;
                    break;
                }
            };
            ind += 1;
            if ind == 15 {
                break;
            }
        }
        if ind < 15 {
            flags |= seq_flags::VALUES_MISSING;
        }
        db.add_entry(ShortSeq {
            a_number: a_num,
            val_num: ind,
            flags: flags,
            vals: vals,
        });
    }

    Ok(db)
}

fn main() {
    {
        let db = stripped_to_db("stripped".to_string()).unwrap();
        let file_out = fs::File::create("shortdb").unwrap();
        let mut writer = io::BufWriter::new(file_out);
        db.serialize(&mut writer).unwrap();
        let score_seq_ind = db.a_to_index.get(300000_u32).unwrap();
        let score_seqs = db.seqs[score_seq_ind as usize];
        println!("{}", score_seqs);
    }

    {
        let file_in = fs::File::open("shortdb").unwrap();
        let mut reader = io::BufReader::new(file_in);
        let db = ShortSeqDB::deserialize(&mut reader).unwrap();
        let score_seq_ind = db.a_to_index.get(300000_u32).unwrap();
        let score_seqs = db.seqs[score_seq_ind as usize];
        println!("{}", score_seqs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_shortseq() -> ShortSeq {
        let a_number = rand::random::<u32>();
        let val_num = u8::min(15, rand::random::<u8>() % 30);
        let flags = 0;
        let mut vals: [i64; 15] = [0; 15];
        for i in 0..val_num {
            vals[i as usize] = rand::random::<i64>();
        }
        ShortSeq {
            a_number,
            val_num,
            flags,
            vals
        }
    }

    #[test]
    fn test_shortseq_serialization() {
        let mut buf = vec![];
        let seq = rand_shortseq();
        seq.serialize(&mut buf).unwrap();
        let mut arr: &[u8] = &buf;
        let new_seq = ShortSeq::deserialize(&mut arr).unwrap();
        assert_eq!(new_seq, seq);
    }

    #[test]
    fn test_shortseqdb_serialization() {
        let mut buf = vec![];
        let mut seqs = Vec::with_capacity(1000);
        let mut db = ShortSeqDB::new();
        for _i in 0..1000 {
            seqs.push(rand_shortseq());
        }
        seqs.sort();
        seqs.dedup();
        for seq in &seqs {
            db.add_entry(*seq);
        }
        db.serialize(&mut buf).unwrap();
        let mut arr: &[u8] = &buf;
        let new_db = ShortSeqDB::deserialize(&mut arr).unwrap();
        assert_eq!(new_db, db);
    }
}