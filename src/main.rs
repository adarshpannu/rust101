
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {    
    let str = String::from("Hello, World");
    let (_, len) = strlen(str);
    println!("{}", len);

}

fn strlen(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}