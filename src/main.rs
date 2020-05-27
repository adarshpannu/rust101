// testing!

use std::collections::HashMap;

struct Cacher<T: Fn(i32) -> i32> {
    closure:        T,
    hash_map:       HashMap<i32,i32>,
}

impl<T> Cacher<T> where T: Fn(i32) -> i32 {
    fn new(closure: T) -> Cacher<T> {
        let hash_map = HashMap::new();
        Cacher {closure, hash_map}
    }

    fn call(&mut self, param: i32) -> i32 {
        *(self.hash_map.entry(param).or_insert( (self.closure)(param) ))
    }
}

fn main() {
    let sqr = |i| i * i;
    let mut cacher = Cacher::new(sqr);

    println!("sqr(6) = {}", cacher.call(6));
    println!("sqr(6) = {}", cacher.call(6));
    println!("sqr(7) = {}", cacher.call(7));
    println!("sqr(7) = {}", cacher.call(7));
    println!("sqr(7) = {}", cacher.call(7));
    println!("sqr(6) = {}", cacher.call(6));

}
