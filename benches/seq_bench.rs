use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{Ring, Field, ModIntP32, MersP31B32, MersP61B64, ShortSeq};

fn test_seq<T: Ring + Copy + std::fmt::Debug>(mxn: usize) {
    
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
