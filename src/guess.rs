use std::io;

pub fn guess_a_number() -> String {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Expecting to read a line");
    buf
}
