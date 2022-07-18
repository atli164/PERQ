#![feature(bigint_helper_methods)]

#[macro_use]
mod macros;

mod hashing;
mod matrix;
mod fixedseq;
mod series;
mod coeff;
mod mathtypes;
// mod exprtree;
mod interpolate;
pub mod oeis;
pub use mathtypes::{Ring, Field, PowerSeries};
pub use coeff::{ModIntP32, MersP31, MersP61};
pub use fixedseq::{ShortSeq};
