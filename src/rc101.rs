use std::rc::Rc;

#[derive(Debug)]
struct Point(i32, i32);

#[test]
fn test_rc() {
    // Instantiate
    let mut rc = Rc::new(Point(-1, 2));

    // Clone
    dbg!(Rc::strong_count(&rc));

    // Cannot move wrapped value out if it doesn't implement Copy
    // let wrapped_point = *rc; <- Will not compile

    // Can get a reference to the wrapped value
    let wrapped_point = &*rc;
    dbg!(wrapped_point);

    // Can get the mutable reference to the wrapped value (if only one shared owner of Rc)
    let wrapped_point = Rc::get_mut(&mut rc);
    dbg!(wrapped_point);

    // Cloning
    {
        let rc2 = Rc::clone(&rc);
        dbg!(Rc::strong_count(&rc));
        dbg!(Rc::strong_count(&rc2));

        let wrapped_point = Rc::get_mut(&mut rc);
        dbg!(wrapped_point);
    
    }
    dbg!(Rc::strong_count(&rc));


}
