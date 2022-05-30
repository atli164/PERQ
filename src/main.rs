#![feature(bigint_helper_methods)]
mod hashing;
mod shortseq;
mod coeff;
mod mathtypes;
// mod genseqs;
mod exprtree;
mod oeis;
pub use mathtypes::{Ring, Field, PowerSeries};
pub use coeff::{ModIntP32, MersP31B32, MersP61B64};
pub use shortseq::{ShortSeq};

fn main() {
    let db = oeis::ShortSeqDB::<ModIntP32>::from_stripped("stripped".to_string()).unwrap();
    db.connectivity().unwrap();
}