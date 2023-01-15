#![allow(warnings)]

#[test]
fn test() {
    println!("Hello I'm back");

    irrigation_measurements(11, 16);
    irrigation_measurements(10, 22);
    //irrigation_measurements(31, 166);
}

fn irrigation_measurements(dim1: u32, dim2: u32) {
    let (width, length) = (dim1.min(dim2), dim1.max(dim2));
    println!("\n--- w = {} l = {}", width, length);
    let lines = ((width * 12 - 4) as f64 / 18f64);
    dbg!(lines);

    // # of parallel lines to lay down
    let lines = (lines.round() + 0.2) as u32;
    dbg!(lines);

    // Pipe length
    let pipe_length = (lines * length) + (2 * width);
    dbg!(pipe_length);


}