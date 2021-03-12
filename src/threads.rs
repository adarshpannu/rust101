#![allow(warnings)]
use std::{thread, time};

fn do_loop(s: &str, limit: i32) {
    let ten_millis = time::Duration::from_millis(100);

    for i in 1..=limit {
        println!("{}: {}", s, i);
        thread::sleep(ten_millis);
    }
}

#[derive(Debug)]
struct ST {
    elem: i32,
}

#[test]
fn test() {
    let v = vec![1, 3, 4];
    let st = ST { elem: 10 };
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join().unwrap();
}
