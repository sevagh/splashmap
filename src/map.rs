use std::collections::hash_map::DefaultHasher;
use std::{cmp::Ord, hash::{Hash, Hasher}};
use splay::SplayMap;

const LOAD_FACTOR_HIGH: f32 = 0.75;
const LOAD_FACTOR_LOW: f32 = 0.5;
const INITIAL_SIZE: usize = 16;

pub struct SplashMap<K, V>
where
    K: Ord
{
    buckets: Vec<Option<SplayMap<K, V>>>,
    num_entries: usize,
}

fn hash_anything<K: Hash>(key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

impl<K, V> SplashMap<K, V>
where
    K: Hash + Ord
{
    fn new() -> SplashMap<K, V> {
        let mut buckets = Vec::with_capacity(INITIAL_SIZE);
        for i in 0..INITIAL_SIZE {
            buckets.push(None);
        }
        SplashMap {
            buckets,
            num_entries: 0,
        }
    }

    /// double the size of the buckets vector and recompute hashes within
    fn grow_and_rehash(&mut self) {
    }

    fn insert(&mut self, k: K, v: V) {
        self.num_entries += 1;

        let real_load_factor = self.num_entries as f32 / self.buckets.len() as f32;

        if real_load_factor >= LOAD_FACTOR_HIGH {
            self.grow_and_rehash();
        } else if real_load_factor <= LOAD_FACTOR_LOW {
            self.shrink_and_rehash();
        }

        let idx = (hash_anything(&k) % (self.buckets.len() as u64)) as usize;
        match self.buckets[idx] {
            None => {
                let mut s = SplayMap::new();
                s.insert(k, v);
                self.buckets[idx] = Some(s);
            }
            Some(ref mut x) => {
                x.insert(k, v);
            }
        }
    }

    fn get(&self, k: K) -> Option<&V> {
        let idx = (hash_anything(&k) % (self.buckets.len() as u64)) as usize;
        match self.buckets[idx] {
            None => None,
            Some(ref x) => {
                x.get(&k)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_anything() {
        let mut m = SplashMap::new();
        m.insert("hello", "world");
        println!("m get: {:#?}", m.get("hello"));
    }
}
