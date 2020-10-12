
#[allow(unused_imports)]

use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T> {
    elem: T
}

impl<T> MyBox<T> {
    fn new(elem: T) -> Self {
        MyBox {elem}
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

fn main() {
    let box1 = MyBox::new(String::from("abc"));
    let elem = &*box1;

    println!("box1 = {:?}", elem);
}
