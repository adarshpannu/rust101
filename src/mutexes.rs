#![allow(warnings)]

use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

#[test]
fn test() {
    let mtx = Arc::new(Mutex::new(10));

    for i in 0..5 {
        let mtx2 = Arc::clone(&mtx);
        let handle = thread::spawn(move || {
            let val = mtx2.lock().unwrap();
        });
    }
}

