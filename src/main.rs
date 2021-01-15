
#![allow(warnings)]

use std::fmt::Debug;

fn main() {
    let keytimes = [(0, 2), (1, 5), (0, 9), (2, 10)];
    slowest_key(&keytimes);
}

fn slowest_key(keytimes: &[(i32, i32)]) -> i32 {
    let mut prev_time = 0;
    let max = keytimes
        .iter()
        .map(|&(key, time)| {
            let diff = time - prev_time;
            prev_time = time;
            diff
        })
        .enumerate()
        .max_by(|e1, e2| e1.1.cmp(&e2.1));
    dbg!(&max);
    0
}
