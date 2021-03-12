#![allow(warnings)]

use std::sync::mpsc;
use std::thread;
use std::time;

#[test]
fn test() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let dur = time::Duration::from_millis(1000);

        // thread::sleep(dur);
        tx.send(String::from("hello"))
    });

    let dur = time::Duration::from_millis(1000);

    thread::sleep(dur);

    let s = rx.try_recv().unwrap();
    println!("{}", s);
}
