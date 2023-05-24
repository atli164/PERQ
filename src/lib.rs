#![feature(bigint_helper_methods)]

#[macro_use]
mod macros;

mod powerseries;
mod matrix;
mod fixedseq;
mod series;
mod coeff;
mod mathtypes;
pub mod lll;
pub mod interpolate;
pub mod oeis;
pub use mathtypes::{Ring, Field};
pub use powerseries::PowerSeries;
pub use coeff::{ModIntP32, MersP31, MersP61};
pub use fixedseq::ShortSeq;
pub use series::Series;
pub use matrix::Matrix;
