use std::str::CharIndices;
use std::iter::Peekable;

pub struct ToSnakeCase<I: Iterator<Item = (usize, char)>> {
    it: Peekable<I>,
    underscore: bool,
}

impl<'a> ToSnakeCase<CharIndices<'a>> {
    pub fn new(s: &'a str) -> Self {
        Self { it: s.char_indices().peekable(), underscore: false }
    }
}

impl<I: Iterator<Item = (usize, char)>> Iterator for ToSnakeCase<I> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        match self.it.peek() {
            Some((i, c)) if c.is_uppercase() && *i != 0 && !self.underscore => {
                self.underscore = true;
                return Some('_');
            }
            _ => {}
        }
        
        self.underscore = false;
        self.it.next().map(|(_, c)| c.to_ascii_lowercase())
    }
}