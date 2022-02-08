use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use pserq::{Ring, Field, ModIntP32, MersP31B32, MersP61B64};

#[allow(dead_code)]
fn test_fib<T: Ring + Copy + std::fmt::Debug>(mxn: usize) {
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
}

#[allow(dead_code)]
fn test_div<T: Field + Copy + std::fmt::Debug>(mxn: usize) {
    let mut sm = T::from(0);
    for i in 0..(mxn as u32) {
        let x = T::from(i);
        if x == T::from(0) { continue; }
        let y = T::from(1) / x;
        assert_eq!(x * y, T::from(1));
        sm = sm + y;
    }
}

macro_rules! bench_type {
    ( $g:expr, $t:ty, $f:tt ) => {
        $g.bench_function(
            BenchmarkId::new(stringify!($f), std::any::type_name::<$t>()),
            |b| b.iter(|| $f::<$t>(black_box(10000)))
        )
    };
}

#[allow(dead_code)]
fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("Coeff div test");
    bench_type!(group, ModIntP32, test_div);
    bench_type!(group, MersP31B32, test_div);
    bench_type!(group, MersP61B64, test_div);
}

#[allow(dead_code)]
fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("Coeff test on Fib");
    bench_type!(group, ModIntP32, test_fib);
    bench_type!(group, MersP31B32, test_fib);
    bench_type!(group, MersP61B64, test_fib);
}

// criterion_group!(benches, bench_fib);
criterion_group!(benches, bench_div);
criterion_main!(benches);
