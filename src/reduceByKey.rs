
#[test]
fn test() {
    let elems = vec![231, 45, 895];

    let it = elems.iter().map(|e| (e, e*2));

    it == 1;
}