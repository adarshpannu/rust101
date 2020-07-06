use std::rc::Rc;

#[derive(Debug)]
struct Point(i32, i32);

#[test]
fn test_rc() {
    // Instantiate
    let mut rc1 = Rc::new(Point(-1, 2));

    // Clone
    dbg!(Rc::strong_count(&rc1));

    // Cannot move wrapped value out if it doesn't implement Copy
    // let wrapped_point = *rc; <- Will not compile

    // Can get a reference to the wrapped value
    let wrapped_point = &*rc1;
    dbg!(wrapped_point);

    // Can get the mutable reference to the wrapped value (if only one shared owner of Rc)
    let wrapped_point = Rc::get_mut(&mut rc1);
    dbg!(wrapped_point);

    // Cloning
    {
        let rc2 = Rc::clone(&rc1);
        dbg!(Rc::strong_count(&rc1));
        dbg!(Rc::strong_count(&rc2));

        // Can no longer get mutable inner because of 2 shared owners
        let wrapped_point = Rc::get_mut(&mut rc1);
        assert!(wrapped_point.is_none());

    }
    dbg!(Rc::strong_count(&rc1));

    // Equality: Do two Rcs point to the same wrapped value?
    let rc2 = Rc::clone(&rc1);
    assert!(Rc::ptr_eq(&rc1, &rc2));

    let rc2 = Rc::new(Point(100, 200));
    assert!(! Rc::ptr_eq(&rc1, &rc2));

    // Sameness - 

}
