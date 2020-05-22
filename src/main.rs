
struct Cacher<T: Fn(i32) -> i32> {
    closure:        T,
    param_opt:      Option<i32>,
    retval_opt:     Option<i32>
}

impl<T> Cacher<T> where T: Fn(i32) -> i32 {
    fn new(closure: T) -> Cacher<T> {
        Cacher {closure, param_opt: None, retval_opt: None}
    }

    fn callme(&mut self, param: i32) -> i32 {
        if param == self.param_opt.unwrap_or(param + 1) {
            println!("Reuse cache param={}", param);
            return self.retval_opt.unwrap();
        }

        // Call the closure and reset cache
        println!("Reset cache param={}", param);
        let retval = (self.closure)(param);
        self.param_opt = Some(param);
        self.retval_opt = Some(retval);
        retval
    }
}

fn main() {
    let sqr = |i| i * i;
    let mut cacher = Cacher::new(sqr);

    println!("sqr(6) = {}", cacher.callme(6));
    println!("sqr(6) = {}", cacher.callme(6));
    println!("sqr(7) = {}", cacher.callme(7));
    println!("sqr(7) = {}", cacher.callme(7));
    println!("sqr(7) = {}", cacher.callme(7));
    println!("sqr(6) = {}", cacher.callme(6));

}
