// half-baked code designed to debug lifetime issues elsewhere, doesn't compile

use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::{fmt::Debug, hash::Hash};

struct HashClass<'a> {
    elems: Vec<char>,
    htable: HashMap<usize, char>,
    iter: Option<Values<'a, usize, char>>,
}

impl<'a> HashClass<'a> {
    fn new(elems: Vec<char>) -> HashClass<'a> {
        let htable = HashMap::new();
        let iter = None;
        HashClass {
            elems,
            htable,
            iter,
        }
    }

    fn foo(&'a mut self) {
        let htable: HashMap<usize, char> = self
            .elems
            .iter()
            .enumerate()
            .map(|(ix, ch)| (ix, *ch))
            .collect();
        self.htable = htable;
        let iter = self.htable.values().into_iter();
        self.iter = Some(iter);
    }

    fn next(&mut self) -> char {
        let mut x = self.iter.as_ref().unwrap();
        let mut y = x.next().unwrap();
        *y
    }
}

fn main() {
    let elems = vec!['a', 'b', 'c'];

    let mut hc = HashClass::new(elems);
    hc.foo();
    for e in &hc.iter {
        println!("{:?}", e);
    }
}
