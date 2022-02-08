#![feature(bigint_helper_methods)]
// mod hashing;
mod shortseq;
// mod modulo;
// mod genseqs;
pub use shortseq::{Ring, Field, ModIntP32, MersP31B32, MersP61B64};
// use std::time::{Instant};

/*
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
*/

/*
fn test_t<T: Ring + Copy + std::fmt::Debug>(mxn: usize) -> f64 {
    let start = Instant::now();
    let mut a = T::from(0);
    let mut b = T::from(1);
    let mut sm = b * b;
    for _i in 0..mxn {
        a = a + b;
        std::mem::swap(&mut a, &mut b);
        sm = sm + b * b;
    }
    a = a + b;
    std::mem::swap(&mut a, &mut b);
    assert_eq!(sm, a * b);
    start.elapsed().as_secs_f64()
}

fn main() {
    println!("{:.2}s", test_t::<ModIntP32>(1_000_000_000));
    println!("{:.2}s", test_t::<ModIntP64>(1_000_000_000));
    println!("{:.2}s", test_t::<ModIntP128>(1_000_000_000));
    println!("{:.2}s", test_t::<MersP31B32>(1_000_000_000));
    println!("{:.2}s", test_t::<MersP31B64>(1_000_000_000));
}
*/
