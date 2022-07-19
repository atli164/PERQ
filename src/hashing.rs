use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
struct HashCell {
    dist: i8,
    key: u32,
    val: u32,
}

impl HashCell {
    fn has_value(&self) -> bool {
        self.dist >= 0
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

#[derive(Debug, PartialEq)]
pub struct FastIntHashTable
where {
    entries: usize,
    size: usize,
    mask: u32,
    log: i8,
    dat: Vec<HashCell>,
}

impl FastIntHashTable {
    pub fn new() -> FastIntHashTable {
        Default::default()
    }

    pub fn entries(&self) -> usize {
        self.entries
    }

    fn grow(&mut self) {
        self.size *= 2;
        self.mask = (self.size - 1) as u32;
        self.log += 1;
        self.rehash(self.size + (self.log as usize) + 1);
    }

    fn rehash(&mut self, buckets: usize) {
        let mut old_dat = vec![HashCell::default(); buckets];
        mem::swap(&mut old_dat, &mut self.dat);
        self.entries = 0;
        for cell in &old_dat {
            if cell.has_value() {
                self.insert(cell.key, cell.val);
            }
        }
    }

    pub fn clear(&mut self) {
        *self = FastIntHashTable::new();
    }

    fn hash(&self, x: u32) -> u32 {
        let val = (x >> 16) ^ x;
        let num = val.wrapping_mul(0x45d9f3b);
        num & self.mask
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
        None
    }
}

impl Default for FastIntHashTable {
    fn default() -> Self {
        FastIntHashTable {
            entries: 0,
            size: 4,
            mask: 3,
            log: 2,
            dat: vec![HashCell::default(); 7]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cells(count: usize) -> Vec<HashCell> {
        let mut res = Vec::with_capacity(count);
        for _i in 0..count {
            let key = rand::random::<u32>();
            let val = rand::random::<u32>();
            let dist = rand::random::<i8>();
            res.push(HashCell {
                dist,
                key,
                val
            });
        }
        res
    }

    fn make_entries(count: usize) -> Vec<(u32, u32)> {
        let mut res = Vec::with_capacity(count);
        for _i in 0..count {
            let key = rand::random::<u32>();
            let val = rand::random::<u32>();
            res.push((key, val));
        }
        res.sort_by_key(|x| x.0);
        res.dedup_by_key(|x| x.0);
        res
    }

    #[test]
    fn test_insert() {
        for size in vec![10, 100, 1000] {
            let mut table = FastIntHashTable::new();
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
        let mut table = FastIntHashTable::new();
        for (key, val) in make_entries(1000) {
            table.insert(key, val);
        }
        table.insert(100, 100);
        assert_eq!(Some(100), table.get(100));
    }

    #[test]
    fn test_overwrite() {
        let mut table = FastIntHashTable::new();
        for (key, val) in make_entries(1000) {
            table.insert(key, val);
        }
        table.insert(100, 100);
        table.insert(100, 200);
        assert_eq!(Some(200), table.get(100));
    }

    #[test]
    fn test_big() {
        let mut table = FastIntHashTable::new();
        let entries = make_entries(100000);
        for (key, val) in &entries {
            table.insert(*key, *val);
        }
        println!("{}", entries.len());
        for (key, val) in &entries {
            assert_eq!(table.get(*key), Some(*val));
        }
    }
}
