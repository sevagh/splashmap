use std::collections::hash_map::HashMap;

fn main() {
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..1000000 {
        map.insert(i, i);
    }
    for i in 0..1000000 {
        assert_eq!(map.get(&i), Some(&i));
    }
}
