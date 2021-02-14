#![allow(warnings)]

mod hashmapwrapper;

#[derive(Debug)]
struct ST<'a> {
    elems: &'a Vec<i32>,
    iter: Option<std::slice::Iter<'a, i32>>,
}

impl<'a> ST<'a> {
    fn make_iter(&mut self) {
        self.iter = Some(self.elems.iter());
    }
}
fn main() {
    let elems = vec![10, 30];
    let iter = None;

    let st = ST { elems: &elems, iter };
}
