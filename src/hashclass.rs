// half-baked code designed to debug lifetime issues

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

    fn init(&mut self) {
        let htable: HashMap<usize, char> = self
            .elems
            .iter()
            .enumerate()
            .map(|(ix, ch)| (ix, *ch))
            .collect();
        self.htable = htable;
        self.iter = Some(self.htable.values());
    }

    /*
    fn next(&mut self) -> char {
        let mut x = self.unwrap();
        let mut y = x.next().unwrap();
        *y
    }
    */
}

fn main() {
    let elems = vec!['a', 'b', 'c'];

    let mut hc = HashClass::new(elems);
    hc.init();
}
