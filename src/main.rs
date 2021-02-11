#![allow(warnings)]

mod so41270052;

struct Person<'a> {
    full_name: String,
    first_name: &'a str,
}

impl<'a> Person<'a> {
    fn set_name(&mut self) {
        self.first_name = &self.full_name[..4];
    }
}

fn main() {}
