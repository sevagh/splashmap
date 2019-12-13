# splashmap

A toy hashmap which uses splay trees for separate chaining. Parts of splashmap's design and API are borrowed from the new [Rust hashbrown](https://github.com/rust-lang/hashbrown) in std.

### Introduction

This is an experiment for me to learn about how hashmaps work under the hood. The introduction of the splay tree for separate chaining adds an addtional `Ord` requirement on the keys of the hashmap.

### 100% collision benching

Here I run a doctored benchmark against [std HashMap](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html) with a custom hash function that intentionally only creates collisions, to verify that using a splaytree has some interesting effect on hash-colliding lookups.  The benchmark inserts 100,000 items, and retrieves 1,000 of them (the same 1,000 each time) in repeated bench iterations. The [collision bench](./benches/collisions.rs) is run with criterion:

```
Benchmarking hashmap: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 387.3s or reduce sample count to 10
hashmap                 time:   [77.880 ms 78.574 ms 79.475 ms]
                        change: [-8.0484% -6.3295% -4.5687%] (p = 0.00 < 0.05)
                        Performance has improved.

splashmap               time:   [37.634 us 37.672 us 37.716 us]
                        change: [-15.564% -14.466% -13.384%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
```

Keep in mind that my benchmark is probably flawed, and that I'm intentionally benching the exact thing splay trees can solve - in a case of horrendous (i.e. 100%) collision, repeated lookups of a handful of keys will become very quick. This is not a serious comparison or competition with the std HashMap.

### Memory

I use massif (`cargo build --examples && valgrind --tool=massif ./target/debug/examples/*`) and `massif-visualizer`.

`HashMap::new()`, insert 1,000,000 `<i32, i32>` pairs:

<img src=".github/hashmap_mem.png" width=800px>

`SplashMap::new()`, insert 1,000,000 `<i32, i32>` pairs:

<img src=".github/splashmap_mem.png" width=800px>
