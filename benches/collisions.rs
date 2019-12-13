use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
    fn write(&mut self, bytes: &[u8]) {}

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

fn hashmap_colliding_inserts(n: usize) {
    let mut map = HashMap::with_hasher(CollisionBuildHasher::new());
    for i in 0..n {
        map.insert(i, i);
    }
}

fn criterion_benchmark_hashmap(c: &mut Criterion) {
    c.bench_function("hashmap", |b| {
        b.iter(|| hashmap_colliding_inserts(black_box(1000000)))
    });
}

fn splashmap_colliding_inserts(n: usize) {
    let mut map = SplashMap::with_hasher(CollisionBuildHasher::new());
    for i in 0..n {
        map.insert(i, i);
    }
}

fn criterion_benchmark_splashmap(c: &mut Criterion) {
    c.bench_function("splashmap", |b| {
        b.iter(|| splashmap_colliding_inserts(black_box(1000000)))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_hashmap,
    criterion_benchmark_splashmap
);
//criterion_group!(benches, criterion_benchmark_splashmap);
criterion_main!(benches);
