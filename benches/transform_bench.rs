use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{Field, ModIntP32, PowerSeries, Series, ShortSeq};

macro_rules! make_transform_func {
    ($f:ident, $meth:ident, $init:expr) => {
        fn $f(mxn: usize) {
            let mut s: ShortSeq<ModIntP32> = $init.parse().unwrap();
            for _i in 0..mxn {
                s = s.$meth();
            }
        }
    };
}

make_transform_func!(integrate_test, integrate, "16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1");
make_transform_func!(binomial_test, binomial, "1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0");
make_transform_func!(binomial_inv_test, binomial_inv, "1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0");

macro_rules! bench_method {
    ( $g:expr, $meth:ident, $f:ident ) => {
        $g.bench_function(
            BenchmarkId::new(stringify!($f), stringify!($meth)),
            |b| b.iter(|| $f(black_box(1000)))
        )
    };
}

#[allow(dead_code)]
fn bench_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("Seq transform test");
    bench_method!(group, integrate, integrate_test);
    bench_method!(group, binomial, binomial_test);
    bench_method!(group, binomial_inv, binomial_inv_test);
}

criterion_group!(benches, bench_transform);
criterion_main!(benches);
