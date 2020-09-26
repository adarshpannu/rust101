#![allow(warnings)]

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (((self.0 * self.0) + (self.1 * self.1)) as f64).sqrt()
    }
}

mod tests {
    use std::cell::RefCell;
    use super::Point;

    #[test]
    fn test_borrowing() {
        let p1 = Point(-1, 2);
        //let mut ref_p1 = &mut p1; // <- Won't compile, cannot borrow as mutable when p1 is immutable

        let rfcl1 = RefCell::new(Point(-1, 2));

        let mut b1 = rfcl1.borrow_mut();

        b1.0 = -10;
        b1.1 = 20;

        let mut b2 = rfcl1.borrow_mut();

    }
}

