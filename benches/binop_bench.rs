use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{Field, ModIntP32, PowerSeries, Series, ShortSeq, MersP31, MersP61};
use std::ops::{Add, Sub, Mul, Div};

macro_rules! make_binop_func {
    ($f:ident, $t:ty, $meth:ident, $init1:expr, $init2:expr) => {
        fn $f(mxn: usize) {
            let mut s: ShortSeq<$t> = $init1.parse().unwrap();
            let mut t: ShortSeq<$t> = $init2.parse().unwrap();
            for _i in 0..mxn {
                s = s.$meth(&t);
                t = t.$meth(&s);
            }
        }
    };
}

make_binop_func!(dir_test_1, ModIntP32, dirichlet, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(dir_test_2, MersP31  , dirichlet, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(dir_test_3, MersP61  , dirichlet, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");

make_binop_func!(add_test_1, ModIntP32, add, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(add_test_2, MersP31  , add, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(add_test_3, MersP61  , add, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");

make_binop_func!(sub_test_1, ModIntP32, sub, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(sub_test_2, MersP31  , sub, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(sub_test_3, MersP61  , sub, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");

make_binop_func!(mul_test_1, ModIntP32, mul, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(mul_test_2, MersP31  , mul, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(mul_test_3, MersP61  , mul, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");

make_binop_func!(div_test_1, ModIntP32, div, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(div_test_2, MersP31  , div, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
make_binop_func!(div_test_3, MersP61  , div, "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16", "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");

macro_rules! bench_method {
    ( $g:expr, $t:ty, $meth:ident, $f:ident ) => {
        $g.bench_function(
            BenchmarkId::new(stringify!($f), stringify!($meth).to_owned() + stringify!($t)),
            |b| b.iter(|| $f(black_box(1000)))
        )
    };
}

#[allow(dead_code)]
fn bench_binop(c: &mut Criterion) {
    let mut group = c.benchmark_group("Seq binop test");
    bench_method!(group, ModIntP32, dirichlet, dir_test_1);
    bench_method!(group, MersP31  , dirichlet, dir_test_2);
    bench_method!(group, MersP61  , dirichlet, dir_test_3);

    bench_method!(group, ModIntP32, add, add_test_1);
    bench_method!(group, MersP31  , add, add_test_2);
    bench_method!(group, MersP61  , add, add_test_3);

    bench_method!(group, ModIntP32, sub, sub_test_1);
    bench_method!(group, MersP31  , sub, sub_test_2);
    bench_method!(group, MersP61  , sub, sub_test_3);

    bench_method!(group, ModIntP32, mul, mul_test_1);
    bench_method!(group, MersP31  , mul, mul_test_2);
    bench_method!(group, MersP61  , mul, mul_test_3);

    bench_method!(group, ModIntP32, div, div_test_1);
    bench_method!(group, MersP31  , div, div_test_2);
    bench_method!(group, MersP61  , div, div_test_3);
}

criterion_group!(benches, bench_binop);
criterion_main!(benches);
