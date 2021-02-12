#![allow(warnings)]
use std::collections::HashMap;
use std::str;

#[derive(Debug)]
pub enum ParserError {
    Generic
}

pub struct Resource(
    pub HashMap<String, String>
);

pub struct Parser<'a> {
    source: Option<str::Chars<'a>>
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { source: None }
    }
    pub fn parse(&mut self, source: &'a str) -> Result<Resource, ParserError> {
        self.source = Some(source.chars());

        let entries = HashMap::new();
        Ok(Resource(entries))
    }
}

fn main() {
    let mut parser = Parser::new();
    parser.parse("key1 = Value 1");
    parser.parse("key2 = Value 2");
}
