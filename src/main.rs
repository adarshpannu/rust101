
struct Cacher<T: Fn(i32) -> i32> {
    closure:        T,
    param_opt:      Option<i32>,
    retval_opt:     Option<i32>
}

impl<T> Cacher<T> where T: Fn(i32) -> i32 {
    fn new(closure: T) -> Cacher<T> {
        Cacher {closure, param_opt: None, retval_opt: None}
    }

    fn call(&mut self, param: i32) -> i32 {
        if Some(param) == self.param_opt {
            println!("Reuse cache param={}", param);
            self.retval_opt.unwrap()
        } else {
            // Call the closure and reset cache
            println!("Reset cache param={}", param);
            let retval = (self.closure)(param);
            self.param_opt = Some(param);
            self.retval_opt = Some(retval);
            retval
        }
    }
}

fn main() {
    let sqr = |i| i * i;
    let mut cacher = Cacher::new(sqr);

    println!("sqr(6) = {}", cacher.call(6));
    println!("sqr(6) = {}", cacher.call(6));
    println!("sqr(7) = {}", cacher.call(7));
    println!("sqr(7) = {}", cacher.call(7));
    println!("sqr(7) = {}", cacher.call(7));
    println!("sqr(6) = {}", cacher.call(6));

}
