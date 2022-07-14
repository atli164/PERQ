#![feature(bigint_helper_methods)]
mod hashing;
mod matrix;
mod fixedseq;
mod coeff;
mod mathtypes;
// mod genseqs;
mod exprtree;
mod interpolate;
mod oeis;
pub use mathtypes::{Ring, Field, PowerSeries};
pub use coeff::{ModIntP32, MersP31, MersP61};
pub use fixedseq::{ShortSeq};

fn main() {
    let db = oeis::ShortSeqDB::<ModIntP32>::from_stripped("stripped".to_string()).unwrap();
    db.connectivity().unwrap();
}
