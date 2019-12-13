use criterion::{criterion_group, criterion_main, Criterion};
use splashmap::map::SplashMap;
use std::collections::hash_map::HashMap;
use std::hash::{BuildHasher, Hasher};

struct CollisionHasher {}

impl CollisionHasher {
    fn new() -> Self {
        CollisionHasher {}
    }
}

impl Hasher for CollisionHasher {
    fn write(&mut self, _bytes: &[u8]) {}

    fn finish(&self) -> u64 {
        1337
    }
}

struct CollisionBuildHasher {}

impl CollisionBuildHasher {
    fn new() -> Self {
        CollisionBuildHasher {}
    }
}

impl BuildHasher for CollisionBuildHasher {
    type Hasher = CollisionHasher;

    fn build_hasher(&self) -> CollisionHasher {
        CollisionHasher::new()
    }
}

const NUM_ENTRIES: usize = 100000;
const RETRIEVE_SUBSET: usize = 100;

fn criterion_benchmark_hashmap(c: &mut Criterion) {
    let mut map = HashMap::with_hasher(CollisionBuildHasher::new());
    for i in 0..NUM_ENTRIES {
        map.insert(i, i);
    }

    c.bench_function("hashmap", |b| {
        b.iter(|| {
            // repeatedly get the same subset of keys
            for i in (0..NUM_ENTRIES).step_by(RETRIEVE_SUBSET) {
                assert_eq!(map.get(&i), Some(&i));
            }
        });
    });
}

fn criterion_benchmark_splashmap(c: &mut Criterion) {
    let mut map = SplashMap::with_hasher(CollisionBuildHasher::new());
    for i in 0..NUM_ENTRIES {
        map.insert(i, i);
    }

    c.bench_function("splashmap", |b| {
        b.iter(|| {
            // repeatedly get the same subset of keys
            for i in (0..NUM_ENTRIES).step_by(RETRIEVE_SUBSET) {
                assert_eq!(map.get(&i), Some(&i));
            }
        });
    });
}

criterion_group!(
    benches,
    criterion_benchmark_hashmap,
    criterion_benchmark_splashmap
);
criterion_main!(benches);
