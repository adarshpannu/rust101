struct Counter {
    cur: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { cur: 0u32, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.max {
            let old_cur = self.cur;
            self.cur += 1;
            Some(old_cur)
        } else {
            None
        }
    }
}

#[allow(unused_imports)]
fn main() {
    let ctr = Counter::new(5);

    for elem in ctr {
        println!("{}", elem)
    }
}
