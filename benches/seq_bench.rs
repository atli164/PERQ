use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{Field, ModIntP32, MersP31, MersP61, PowerSeries, Series, ShortSeq};

fn test_seq<T: Field + Copy + std::fmt::Debug>(mxn: usize) {
    /*
    let mut a = Series::<T> {
        seq: vec![Default::default(); 16]
    };
    let mut b = Series::<T> {
        seq: vec![Default::default(); 16]
    };
    b.seq[0] = T::from(0u32);
    let mut sm = &b * &b;
    for _i in 0..mxn {
        a = a + &b;
        std::mem::swap(&mut a, &mut b);
        sm = sm + &b * &b;
    }
    a = a + &b;
    std::mem::swap(&mut a, &mut b);
    assert_eq!(sm, a * b);
    */
    let mut a = ShortSeq::<T>::promote(T::from(0u32));
    let mut b = ShortSeq::<T>::promote(T::from(1u32));
    let mut sm = b * b;
    for _i in 0..mxn {
        a += b;
        std::mem::swap(&mut a, &mut b);
        sm += b * b;
    }
    a += b;
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
    bench_type!(group, MersP31, test_seq);
    bench_type!(group, MersP61, test_seq);
}

criterion_group!(benches, bench_seq);
criterion_main!(benches);
