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
        SplashMap::<K, V>::with_capacity(INITIAL_SIZE)
    }

    pub fn with_capacity(capacity: usize) -> SplashMap<K, V> {
        let mut buckets = Vec::<Option<SplayMap<K, V>>>::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(None);
        }
        SplashMap {
            buckets,
            num_entries: 0,
        }
    }

    pub(crate) fn print_collisions(&self) {
        for i in 0..self.buckets.len() {
            match self.buckets[i] {
                None => println!("Bucket slot {} is empty", i),
                Some(ref x) => println!(
                    "Bucket slot {} has {} entries in SplayMap chain",
                    i,
                    x.len()
                ),
            }
        }
    }

    pub fn hash_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.buckets.len() as u64)) as usize
    }

    /// resize the buckets vector and recompute hashes within
    pub fn resize_and_rehash(&mut self, new_size: usize) {
        self.buckets.resize_with(new_size, || None);
        let mut displaced_nodes = Vec::<SplayMap<K, V>>::new();
        for i in 0..self.buckets.len() {
            println!("i is {}, len of buckets is {}", i, self.buckets.len());
            if self.buckets[i].is_some() {
                // occupied chain
                self.buckets.push(None);
                let old = self.buckets.swap_remove(i).unwrap();
                displaced_nodes.push(old);
            }
        }

        for d in displaced_nodes.drain(..) {
            // reinsert all the entries of the SplayMap
            // to recompute the hashes with the new buckets size
            for pair in d.into_iter() {
                self.insert(pair.0, pair.1);
            }
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        let idx = self.hash_index(&k);
        match self.buckets[idx] {
            None => {
                self.num_entries += 1;
                let mut s = SplayMap::new();
                s.insert(k, v);
                self.buckets[idx] = Some(s);
            }
            Some(ref mut x) => {
                let ret = x.insert(k, v);
                if ret.is_none() {
                    // https://docs.rs/splay/0.1.8/splay/map/struct.SplayMap.html#method.insert
                    self.num_entries += 1;
                }
            }
        }

        let real_load_factor = self.num_entries as f32 / self.buckets.len() as f32;

        if real_load_factor >= LOAD_FACTOR_HIGH {
            self.resize_and_rehash(2 * self.buckets.len());
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

    pub fn len(&self) -> usize {
        self.num_entries
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        (LOAD_FACTOR_HIGH * (self.buckets.len() as f32)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::HashMap;

    #[test]
    fn test_hash_capacity() {
        let mut sm = SplashMap::new();
        let mut hm = HashMap::new();

        sm.insert("Hello", "World");
        hm.insert("Hello", "World");

        println!("SplashMap capacity: {:#?}", sm.capacity());
        println!("SplashMap len: {:#?}", sm.len());

        println!("HashMap capacity: {:#?}", hm.capacity());
        println!("HashMap len: {:#?}", hm.len());

        sm.insert("Hello", "World2");
        hm.insert("Hello", "World2");

        println!("SplashMap capacity: {:#?}", sm.capacity());
        println!("SplashMap len: {:#?}", sm.len());

        println!("HashMap capacity: {:#?}", hm.capacity());
        println!("HashMap len: {:#?}", hm.len());

        sm.insert("Hello2", "World2");
        hm.insert("Hello2", "World2");

        println!("SplashMap capacity: {:#?}", sm.capacity());
        println!("SplashMap len: {:#?}", sm.len());

        println!("HashMap capacity: {:#?}", hm.capacity());
        println!("HashMap len: {:#?}", hm.len());
    }

    #[test]
    fn test_collisions() {
        let mut sm = SplashMap::<i32, i32>::new();

        for i in 0..10000 {
            sm.insert(i, i + i);
        }

        sm.print_collisions();
    }

    #[test]
    fn test_hash_anything() {
        let mut m = SplashMap::new();
        m.insert("hello", "world");
        println!("m get: {:#?}", m.get("hello"));
    }
}
