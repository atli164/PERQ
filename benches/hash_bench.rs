// Not in use anymore
// Showed that my fancy custom hash table is outperformed by fxhash
// Thus my custom hash table is now being removed
// For reference the below test showed a performance of 636us on mine, 495us on fx

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use perq::{FastIntHashTable};
use rand::thread_rng;
use rand::prelude::SliceRandom;
use fxhash::FxHashMap;

fn test_fx_hash(mxn: usize) {
    let mut rng = thread_rng();
    let mut vec: Vec<u32> = (0..mxn as u32).collect();
    vec.shuffle(&mut rng);
    let mut fx = FxHashMap::default();
    for x in &vec {
        fx.insert(*x, *x);
    }
    let mut sm = 0;
    vec.shuffle(&mut rng);
    for x in &vec {
        sm += *fx.get(x).unwrap() as usize;
    }
    assert!(sm == mxn * (mxn - 1) / 2);
}

fn test_custom_hash(mxn: usize) {
    let mut rng = thread_rng();
    let mut vec: Vec<u32> = (0..mxn as u32).collect();
    vec.shuffle(&mut rng);
    let mut my = FastIntHashTable::new();
    for x in &vec {
        my.insert(*x, *x);
    }
    let mut sm = 0;
    vec.shuffle(&mut rng);
    for x in &vec {
        sm += my.get(*x).unwrap() as usize;
    }
    assert!(sm == mxn * (mxn - 1) / 2);
}

#[allow(dead_code)]
fn bench_seq(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hash test");
    group.bench_function(
        BenchmarkId::new("test hash", "fx"),
        |b| b.iter(|| test_fx_hash(black_box(10000)))
    );
    group.bench_function(
        BenchmarkId::new("test hash", "my"),
        |b| b.iter(|| test_custom_hash(black_box(10000)))
    );
}

criterion_group!(benches, bench_seq);
criterion_main!(benches);
