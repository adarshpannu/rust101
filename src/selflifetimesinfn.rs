use std::slice::Iter;

#[derive(Debug)]
struct Foo<'a> {
    name: &'a str,
    elems: Vec<i32>,
    iter: Option<Iter<'a, i32>>,
}

impl<'a> Foo<'a> {
    fn create_iter(&'a mut self) {
        self.iter = Some(self.elems.iter());
    }

    fn get_next(&mut self) -> Option<&i32> {
        self.create_iter();
        
        if let Some(ref mut iter) = self.iter {
            iter.next()
        } else {
            None
        }
    }
}

fn main() {
    let foo = Foo {
        elems: vec![1, 4, 5],
        name: "abc",
        iter: None,
    };
    println!("{:?}", foo);
    //let iter: Iter<'_, _> = foo.elems.iter();
}
