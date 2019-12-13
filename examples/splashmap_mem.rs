use splashmap::map::SplashMap;

fn main() {
    let mut map = SplashMap::<i32, i32>::new();
    for i in 0..1000000 {
        map.insert(i, i);
    }
    for i in 0..1000000 {
        assert_eq!(map.get(&i), Some(&i));
    }
}
