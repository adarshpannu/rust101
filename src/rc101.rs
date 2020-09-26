#![allow(warnings)]

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn distance_from_origin(&self) -> f64 {
        (((self.0 * self.0) + (self.1 * self.1)) as f64).sqrt()
    }
}

mod tests {
    use std::rc::Rc;
    use super::Point;

    #[test]
    fn test_cloning() {
        // Instantiate
        let rc1 = Rc::new(Point(-1, 2));

        // Check strong_count
        assert_eq!(Rc::strong_count(&rc1), 1);

        // Cloning increases strong_count
        {
            let rc2 = Rc::clone(&rc1);
            assert_eq!(Rc::strong_count(&rc1), 2);
            assert_eq!(Rc::strong_count(&rc2), 2);
        }

        // strong_count decreases when a clone drops off
        assert_eq!(Rc::strong_count(&rc1), 1);
    }

    #[test]
    fn test_access_inner() {
        // Instantiate
        let rc1 = Rc::new(Point(-1, 2));

        // Cannot move wrapped value out if it doesn't implement Copy.
        // let wrapped_point = *rc; // <- Will not compile

        // Can get a reference to the wrapped value
        let wrapped_point = &*rc1;
        assert_eq!(wrapped_point.0, -1);

        // Can get the wrapped value and own it ... but only if Rc::strong_count() == 1
        // Ref: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap
        let wrapped_result = Rc::try_unwrap(rc1);
        assert!(wrapped_result.is_ok());
        let _p_owned = wrapped_result.unwrap();

        // Re-Instantiate
        let mut rc1 = Rc::new(Point(-1, 2));

        // Can get the mutable reference to the wrapped value (if only one shared owner of Rc)
        let wrapped_point = Rc::get_mut(&mut rc1);
        assert!(wrapped_point.is_some());

        let _rc2 = Rc::clone(&rc1);

        // Can no longer get mutable inner because of 2 shared owners
        let wrapped_point = Rc::get_mut(&mut rc1);
        assert!(wrapped_point.is_none());

    }

    #[test]
    fn test_access_inner_state() {
        let rc1 = Rc::new(Point(3, 4));

        // Access fields directly
        assert_eq!(rc1.0, 3);
        assert_eq!(rc1.1, 4);

        // Access methods directly
        assert_eq!(rc1.distance_from_origin(), 5f64);

    }

    #[test]
    fn test_equality() {
        let rc1 = Rc::new(Point(-1, 2));
        let rc2 = Rc::clone(&rc1);

        // Clones have the same inner pointer? Yes
        assert!(Rc::ptr_eq(&rc1, &rc2));

        // Non-clones will not have the same inner pointer
        let rc3 = Rc::new(Point(-1, 2));
        assert!(Rc::ptr_eq(&rc1, &rc3) == false);

        // Non-clones can only be compared if they implement the PartialEq (which Point does not!)
        // assert_eq!(rc1, rc2); // <- Will not compile
        
        let rcint1 = Rc::new(101);
        let rcint2 = Rc::new(101);
        assert_eq!(rcint1, rcint2)
    }
}
