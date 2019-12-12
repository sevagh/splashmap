use splay::SplayMap;
use std::collections::hash_map::DefaultHasher;
use std::{
    cmp::Ord,
    hash::{Hash, Hasher},
};

const LOAD_FACTOR_HIGH: f32 = 0.75; //grow with this
const LOAD_FACTOR_LOW: f32 = 0.25; //shrink with this
const INITIAL_SIZE: usize = 16;

#[derive(Default)]
pub struct SplashMap<K, V>
where
    K: Ord,
{
    buckets: Vec<Option<SplayMap<K, V>>>,
    num_entries: usize,
}

impl<K, V> SplashMap<K, V>
where
    K: Hash + Ord,
{
    pub fn new() -> SplashMap<K, V> {
        let mut buckets = Vec::<Option<SplayMap<K, V>>>::with_capacity(INITIAL_SIZE);
        for _ in 0..INITIAL_SIZE {
            buckets.push(None);
        }
        SplashMap {
            buckets,
            num_entries: 0,
        }
    }

    pub fn hash_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.buckets.len() as u64)) as usize
    }

    /// double the size of the buckets vector and recompute hashes within
    pub fn resize_and_rehash(&mut self, new_size: usize) {
        self.buckets.resize_with(new_size, || None);
        let mut displaced_nodes = Vec::<SplayMap<K, V>>::new();
        for i in 0..self.buckets.len() {
            if self.buckets[i].is_some() {
                // occupied chain
                let old = self.buckets.swap_remove(i).unwrap();
                displaced_nodes.push(old);
                self.buckets[i] = None;
            }
        }

        for i in 0..displaced_nodes.len() {
            let displaced_node = displaced_nodes.swap_remove(i);
            for pair in displaced_node.into_iter() {
                self.insert(pair.0, pair.1);
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.num_entries += 1;

        let real_load_factor = self.num_entries as f32 / self.buckets.len() as f32;

        if real_load_factor >= LOAD_FACTOR_HIGH {
            self.resize_and_rehash(2 * self.buckets.len());
        }

        let idx = self.hash_index(&k);
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

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let idx = self.hash_index(k);
        let mut ret: Option<V> = None;
        match self.buckets[idx] {
            None => return ret,
            Some(ref mut x) => {
                self.num_entries -= 1;
                ret = x.remove(k);
            }
        }

        let real_load_factor = self.num_entries as f32 / self.buckets.len() as f32;

        if real_load_factor < LOAD_FACTOR_LOW {
            self.resize_and_rehash(self.buckets.len() / 2);
        }
        ret
    }

    pub fn get(&self, k: K) -> Option<&V> {
        let idx = self.hash_index(&k);
        match self.buckets[idx] {
            None => None,
            Some(ref x) => x.get(&k),
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
