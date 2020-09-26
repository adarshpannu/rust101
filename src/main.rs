mod rc101;
mod refcell101;
mod guess;

use std::io::{self, Write};

fn main() {
    print!("Enter a line: ");
    io::stdout().flush().unwrap();

    let g: String = guess::guess_a_number();
    println!("guess = {}", g)
}
