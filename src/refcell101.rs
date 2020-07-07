mod tests {
    use std::cell::RefCell;

    #[test]
    fn test_basics() {
        let rfcl = RefCell::new(10);

        let b = rfcl.borrow_mut();

    }
}