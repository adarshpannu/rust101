
#[allow(unused_imports)]

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

use List::*;

fn main() {
    let l1 = Cons(10, Box::new(Nil));
    let l2 = Cons(20, Box::new(l1));

    println!("l2 = {:?}", l2);
}
