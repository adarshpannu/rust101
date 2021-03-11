#![allow(warnings)]
use std::{thread, time};

fn do_loop(s: &str, limit: i32) {
    let ten_millis = time::Duration::from_millis(100);

    for i in 1..=limit {
        println!("{}: {}", s, i);
        thread::sleep(ten_millis);
    }
}

#[test]
fn test() {
    let handle = thread::spawn(|| {
        do_loop("child", 10)
    });

    handle.join().unwrap();

    do_loop("parent", 5);

}

