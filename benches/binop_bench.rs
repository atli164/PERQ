use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{Field, PowerSeries, Series, ShortSeq, MersP31};
use std::ops::{Add, Sub, Mul, Div};

macro_rules! make_binop_func {
    ($f:ident, $meth:ident, $init1:expr, $init2:expr) => {
        fn $f(mxn: usize) {
            let mut s: ShortSeq<MersP31> = $init1.parse().unwrap();
            let mut t: ShortSeq<MersP31> = $init2.parse().unwrap();
            for _i in 0..mxn {
                s = s.$meth(&t);
                t = t.$meth(&s);
            }
        }
    };
}

make_binop_func!(dir_test, dirichlet, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(add_test, add, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(sub_test, sub, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(mul_test, mul, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(div_test, div, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(compose_test, compose, "0,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "0,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");

macro_rules! bench_method {
    ( $g:expr, $meth:ident, $f:ident ) => {
        $g.bench_function(
            BenchmarkId::new(stringify!($f), stringify!($meth).to_owned()),
            |b| b.iter(|| $f(black_box(1000)))
        )
    };
}

#[allow(dead_code)]
fn bench_binop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Seq binop test");
    /*
    bench_method!(group, dirichlet, dir_test);
    bench_method!(group, add, add_test);
    bench_method!(group, sub, sub_test);
    bench_method!(group, mul, mul_test);
    bench_method!(group, div, div_test);
    */
    bench_method!(group, compose, compose_test);
}

criterion_group!(benches, bench_binop);
criterion_main!(benches);
