use std::mem;
use std::io;

pub trait Serializable {
    fn serialize<S: io::Write>(&self, ostr: &mut S) -> io::Result<()>;
    fn deserialize<S: io::Read>(istr: &mut S) -> io::Result<Self> where Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct HashCell {
    dist: i8,
    key: u32,
    val: u32,
}

impl HashCell {
    fn has_value(&self) -> bool {
        return self.dist >= 0;
    }

    fn set_values(&mut self, d: i8, k: u32, v: u32) {
        self.dist = d;
        self.key = k;
        self.val = v;
    }
}

impl Default for HashCell {
    fn default() -> HashCell {
        HashCell {
            dist: -1,
            key: 0,
            val: 0
        }
    }
}

impl Serializable for HashCell {
    fn serialize<S: io::Write>(&self, ostr: &mut S) -> io::Result<()> {
        ostr.write(&self.dist.to_be_bytes())?;
        ostr.write(&self.key.to_be_bytes())?;
        ostr.write(&self.val.to_be_bytes())?;
        Ok(())
    }

    fn deserialize<S: io::Read>(istr: &mut S) -> io::Result<HashCell> {
        let mut buf1: [u8; 1] = [0; 1];
        let mut buf4: [u8; 4] = [0; 4];
        istr.read(&mut buf1)?;
        let dist = i8::from_be_bytes(buf1);
        istr.read(&mut buf4)?;
        let key = u32::from_be_bytes(buf4);
        istr.read(&mut buf4)?;
        let val = u32::from_be_bytes(buf4);
        Ok(HashCell {
            dist,
            key,
            val
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct HashTable
where {
    entries: usize,
    size: usize,
    mask: u32,
    log: i8,
    dat: Vec<HashCell>,
}

impl HashTable {
    pub fn new() -> HashTable {
        Default::default()
    }

    fn grow(&mut self) {
        self.size *= 2;
        self.mask = (self.size - 1) as u32;
        self.log += 1;
        self.rehash(self.size + (self.log as usize) + 1);
    }

    fn rehash(&mut self, buckets: usize) {
        let mut old_dat = vec![];
        mem::swap(&mut old_dat, &mut self.dat);
        self.dat = vec![HashCell::default(); buckets];
        self.entries = 0;
        for cell in &old_dat {
            if cell.has_value() {
                self.insert(cell.key, cell.val);
            }
        }
    }

    pub fn clear(&mut self) {
        *self = HashTable::new();
    }

    fn hash(&self, x: u32) -> u32 {
        let val = (x >> 16) ^ x;
        let num = val.wrapping_mul(0x45d9f3b);
        return num & self.mask;
    }

    pub fn insert(&mut self, mut k: u32, mut v: u32) {
        if self.entries > self.size / 2 {
            self.grow();
        }
        let mut ind = self.hash(k) as usize;
        let mut d = 0;
        while d < self.log {
            if !self.dat[ind].has_value() {
                self.dat[ind].set_values(d, k, v);
                self.entries += 1;
                return;
            } else if self.dat[ind].key == k {
                self.dat[ind].val = v;
                return;
            } else if self.dat[ind].dist < d {
                mem::swap(&mut v, &mut self.dat[ind].val);
                mem::swap(&mut k, &mut self.dat[ind].key);
                mem::swap(&mut d, &mut self.dat[ind].dist);
            }
            ind += 1; d += 1;
        }
        self.grow();
        self.insert(k, v);
    }

    pub fn get(&self, k: u32) -> Option<u32> {
        let mut ind = self.hash(k) as usize;
        let mut d = 0;
        while self.dat[ind].dist >= d {
            if self.dat[ind].key == k {
                return Some(self.dat[ind].val);
            }
            ind += 1; d += 1;
        }
        return None;
    }
}

impl Default for HashTable {
    fn default() -> Self {
            HashTable {
            entries: 0,
            size: 4,
            mask: 3,
            log: 2,
            dat: vec![HashCell::default(); 7]
        }
    }
}

impl Serializable for HashTable {
    fn serialize<S: io::Write>(&self, ostr: &mut S) -> io::Result<()> {
        ostr.write(&self.entries.to_be_bytes())?;
        ostr.write(&self.size.to_be_bytes())?;
        ostr.write(&self.mask.to_be_bytes())?;
        ostr.write(&self.log.to_be_bytes())?;
        for cell in &self.dat {
            cell.serialize(ostr)?;
        }
        Ok(())
    }

    fn deserialize<S: io::Read>(istr: &mut S) -> io::Result<HashTable> {
        let mut buf1: [u8; 1] = [0; 1];
        let mut buf4: [u8; 4] = [0; 4];
        let mut buf8: [u8; 8] = [0; 8];
        istr.read(&mut buf8)?;
        let entries = usize::from_be_bytes(buf8);
        istr.read(&mut buf8)?;
        let size = usize::from_be_bytes(buf8);
        istr.read(&mut buf4)?;
        let mask = u32::from_be_bytes(buf4);
        istr.read(&mut buf1)?;
        let log = i8::from_be_bytes(buf1);
        let mut dat = Vec::with_capacity(size);
        for _i in 0..(size + (log as usize) + 1) {
            dat.push(HashCell::deserialize(istr)?);
        }
        Ok(HashTable {
            entries,
            size,
            mask,
            log,
            dat
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entries(count: usize) -> Vec<(u32, u32)> {
        let mut res = Vec::with_capacity(count);
        for _i in 0..count {
            let key = rand::random::<u32>();
            let val = rand::random::<u32>();
            res.push((key, val));
        }
        res.sort();
        res.dedup();
        res
    }

    #[test]
    fn test_insert() {
        for size in vec![10, 100, 1000] {
            let mut table = HashTable::new();
            for (key, val) in make_entries(size) {
                table.insert(key, val);
            }
            assert_eq!(table.size - 1, table.mask as usize);
            assert_eq!(1, table.size.count_ones());
            assert_eq!(table.mask.leading_zeros(), table.mask.count_zeros());
            assert_eq!(table.log as u32, table.mask.count_ones());
        }
    }

    #[test]
    fn test_get() {
        let mut table = HashTable::new();
        for (key, val) in make_entries(1000) {
            table.insert(key, val);
        }
        table.insert(100, 100);
        assert_eq!(Some(100), table.get(100));
    }

    #[test]
    fn test_overwrite() {
        let mut table = HashTable::new();
        for (key, val) in make_entries(1000) {
            table.insert(key, val);
        }
        table.insert(100, 100);
        table.insert(100, 200);
        assert_eq!(Some(200), table.get(100));
    }

    #[test]
    fn test_serialization() {
        let mut buf = vec![];
        let mut table = HashTable::new();
        let entries = make_entries(2000);
        for (key, val) in &entries[..1000] {
            table.insert(*key, *val);
        }
        table.serialize(&mut buf).unwrap();
        let mut arr: &[u8] = &buf;
        let new_table = HashTable::deserialize(&mut arr).unwrap();
        for (key, val) in &entries[..1000] {
            assert_eq!(new_table.get(*key), Some(*val));
        }
        for (key, _val) in &entries[1000..] {
            assert_eq!(new_table.get(*key), None);
        }
    }
}