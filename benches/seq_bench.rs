use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{Ring, ModIntP32, MersP31B32, MersP61B64, ShortSeq};

fn test_seq<T: Ring + Copy + std::fmt::Debug>(mxn: usize) {
    let mut a = ShortSeq::<T>::from(T::from(0));
    let mut b = ShortSeq::<T>::from(T::from(1));
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

macro_rules! bench_type {
    ( $g:expr, $t:ty, $f:tt ) => {
        $g.bench_function(
            BenchmarkId::new(stringify!($f), std::any::type_name::<$t>()),
            |b| b.iter(|| $f::<$t>(black_box(1000)))
        )
    };
}

#[allow(dead_code)]
fn bench_seq(c: &mut Criterion) {
    let mut group = c.benchmark_group("Seq ring test");
    bench_type!(group, ModIntP32, test_seq);
    bench_type!(group, MersP31B32, test_seq);
    bench_type!(group, MersP61B64, test_seq);
}

criterion_group!(benches, bench_seq);
criterion_main!(benches);
