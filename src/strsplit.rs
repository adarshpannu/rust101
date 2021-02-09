#![allow(warnings)]

pub struct StrSplit {
    remainder: &str,
    delimiter: &str,
}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl Iterator for StrSplit {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // BUG?
            None
        } else {
            let rest = self.remainder;
            // self.remainder = &[]; <=
            Some(rest)
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
