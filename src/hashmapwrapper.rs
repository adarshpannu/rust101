use std::collections::{HashMap, hash_map::Values};

struct HashMapWrapper<'a> {
    hmap: &'a mut HashMap<String, u32>,
    iter: Option<Values<'a, String, u32>>,
}

impl<'a> HashMapWrapper<'a> {
    fn new(hmap: &mut HashMap<String, u32>) -> HashMapWrapper {
        let hmw = HashMapWrapper {
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
    let mut hmw = HashMapWrapper::new(&mut hmap);

    hmw.make_iter();

}
