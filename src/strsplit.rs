#![allow(warnings)]

pub struct StrSplit<'a, 'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder.is_none() {
            return None;
        } else {
            let remainder = self.remainder.unwrap();
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                self.remainder = Some(&remainder[(next_delim + self.delimiter.len())..]);
                Some(until_delimiter)
            } else if remainder.is_empty() {
                self.remainder = None;
                Some("")
            } else {
                let rest = remainder;
                self.remainder = None;
                Some(rest)
            }
        }
    }
}

fn until_char(s: &str, c: char) -> Option<&str> {
    let delim = format!("{}", c);

    let mut ss = StrSplit::new(s, &delim);
    ss.next()
}

#[cfg(test)]
mod test {
    use super::StrSplit;

    #[test]
    fn test() {
        let haystack = "a b c".to_string();
        let mut ss = StrSplit::new(&haystack, " ");

        let part = ss.next();
        println!("{:?}", part);
        drop(ss);
        println!("{:?}", part);
    }

    #[test]
    fn test1() {
        let haystack = "a b c d ".to_string();
        let mut ss = StrSplit::new(&haystack, " ");

        while let Some(part) = ss.next() {
            println!("{:?}", part);
        }
    }
}
