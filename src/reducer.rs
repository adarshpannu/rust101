use std::collections::{HashMap, hash_map::Values};

struct Reducer<'a> {
    hmap: &'a mut HashMap<String, u32>,
    iter: Option<Values<'a, String, u32>>,
}

impl<'a> Reducer<'a> {
    fn new(hmap: &mut HashMap<String, u32>) -> Reducer {
        let hmw = Reducer {
            hmap: hmap,
            iter: None,
        };
        hmw 
    }

    fn make_iter(&mut self) {
        self.iter = Some(self.hmap.values());
    }

    fn add_elem(&mut self, key: String, val: u32) {
        self.hmap.insert(key, val);
    }
}

fn main() {
    let mut hmap = HashMap::new();
    let mut hmw = Reducer::new(&mut hmap);
    hmap.insert("hello".to_string(), 1);
    hmw.make_iter();
}

/*

struct ST<'a> {
    elems: &'a mut Vec<i32>,
    iter: Option<std::slice::Iter<'a, i32>>,
}

impl<'a> ST<'a> {
    fn make_iter(&mut self) {
        self.iter = Some(self.elems.iter());
    }

    fn add_elem(&mut self, elem: i32) {
        self.elems.push(elem);
    }

}

fn main() {
    let elems = vec![10, 30];
    let iter = None;

    let st = ST { elems: &mut elems, iter };
}
 */