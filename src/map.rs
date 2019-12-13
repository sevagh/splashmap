use splay::SplayMap;
use std::collections::hash_map::RandomState;
use std::{
    cmp::Ord,
    default::Default,
    hash::{BuildHasher, Hash, Hasher},
};

const LOAD_FACTOR_HIGH: f32 = 0.75; //grow with this
const LOAD_FACTOR_LOW: f32 = 0.25; //shrink with this
const INITIAL_SIZE: usize = 1;

pub type DefaultHashBuilder = RandomState;

pub struct SplashMap<K, V, S = DefaultHashBuilder>
where
    K: Ord,
{
    buckets: Vec<Option<SplayMap<K, V>>>,
    num_entries: usize,
    hash_builder: S,
}

fn make_hash<K: Hash + ?Sized>(hash_builder: &impl BuildHasher, val: &K) -> u64 {
    let mut state = hash_builder.build_hasher();
    val.hash(&mut state);
    state.finish()
}

impl<K, V> Default for SplashMap<K, V, DefaultHashBuilder>
where
    K: Hash + Ord,
{
    fn default() -> Self {
        SplashMap::<K, V>::new()
    }
}

impl<K, V> SplashMap<K, V, DefaultHashBuilder>
where
    K: Hash + Ord,
{
    pub fn new() -> Self {
        SplashMap::<K, V>::with_capacity_and_hasher(INITIAL_SIZE, DefaultHashBuilder::default())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        SplashMap::<K, V>::with_capacity_and_hasher(capacity, DefaultHashBuilder::default())
    }
}

impl<K, V, S> SplashMap<K, V, S>
where
    K: Hash + Ord,
    S: BuildHasher,
{
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        let mut buckets = Vec::<Option<SplayMap<K, V>>>::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(None);
        }
        SplashMap {
            buckets,
            num_entries: 0,
            hash_builder,
        }
    }

    /// resize the buckets vector and recompute hashes within
    pub fn resize_and_rehash(&mut self, new_size: usize) {
        self.buckets.resize_with(new_size, || None);
        let mut displaced_nodes = Vec::<SplayMap<K, V>>::new();
        for i in 0..self.buckets.len() {
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
                self.insert_priv(pair.0, pair.1);
            }
        }
    }

    fn insert_priv(&mut self, k: K, v: V) -> usize {
        // this one doesn't increment
        let mut ret: usize = 0;
        let hash = make_hash(&self.hash_builder, &k);
        let idx = (hash % (self.buckets.len() as u64)) as usize;
        match self.buckets[idx] {
            None => {
                let mut s = SplayMap::new();
                s.insert(k, v);
                ret = 1;
                self.buckets[idx] = Some(s);
            }
            Some(ref mut s) => {
                if s.insert(k, v).is_none() {
                    ret = 1;
                }
            }
        }

        let real_load_factor = self.num_entries as f32 / self.buckets.len() as f32;

        if real_load_factor >= LOAD_FACTOR_HIGH {
            self.resize_and_rehash(2 * self.buckets.len());
        }
        ret
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.num_entries += self.insert_priv(k, v);
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let hash = make_hash(&self.hash_builder, &k);
        let idx = (hash % (self.buckets.len() as u64)) as usize;
        let mut ret: Option<V> = None;
        match self.buckets[idx] {
            None => return ret,
            Some(ref mut x) => {
                ret = x.remove(k);
                if ret.is_some() {
                    self.num_entries -= 1;
                }
            }
        }

        let real_load_factor = self.num_entries as f32 / self.buckets.len() as f32;

        println!("real load factor: {}", real_load_factor);
        if real_load_factor < LOAD_FACTOR_LOW {
            println!("len buckets: {}", self.buckets.len());
            self.resize_and_rehash(self.buckets.len() / 2);
        }
        ret
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        let hash = make_hash(&self.hash_builder, &k);
        let idx = (hash % (self.buckets.len() as u64)) as usize;
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
        let ret = (LOAD_FACTOR_HIGH * (self.buckets.len() as f32)) as usize;
        if ret == 0 {
            return 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::HashMap;

    #[test]
    fn test_hash_capacity_grows() {
        let mut sm = SplashMap::new();
        assert_eq!(sm.capacity(), 1);
        let mut cap: usize = 0;

        for i in 0..10 {
            assert_eq!(sm.len(), i);
            sm.insert(i, "hello");

            // make sure it's growing
            assert!(sm.capacity() >= cap);
            cap = sm.capacity();
        }
    }

    #[test]
    fn test_hash_capacity_shrinks() {
        let mut sm = SplashMap::new();

        for i in 0..10 {
            sm.insert(i, i);
        }

        assert_eq!(sm.len(), 10);

        let mut cap = sm.capacity();

        for i in 0..10 {
            assert_eq!(sm.get(&i), Some(&i));
            assert_eq!(sm.remove(&i), Some(i));
            assert_eq!(sm.get(&i), None);
            assert_eq!(sm.remove(&i), None);
            assert!(sm.capacity() <= cap);
            cap = sm.capacity();
        }
    }

    #[test]
    fn test_collisions() {
        let mut sm = SplashMap::<i32, i32>::new();

        for i in 0..10000 {
            sm.insert(i, i + i);
        }
    }

    #[test]
    fn test_hash_anything() {
        let mut m = SplashMap::new();
        m.insert("hello", "world");
        println!("m get: {:#?}", m.get(&"hello"));
    }
}
