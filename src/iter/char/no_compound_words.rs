use crate::iter::char::Iterators;
use std::iter::Peekable;

/// Omits `-` characters that indicate compound words like "ex-īre".
pub struct NoCompoundWords<I: Iterator<Item = char>> {
    iter: Peekable<I>,
    prior: Option<char>,
}

impl<I: Iterator<Item = char>> NoCompoundWords<I> {
    pub fn new(iter: I) -> NoCompoundWords<I> {
        NoCompoundWords {
            iter: iter.peekable(),
            prior: None,
        }
    }
}

impl<I: Iterator<Item = char>> Iterators for NoCompoundWords<I> {}

impl<I: Iterator<Item = char>> Iterator for NoCompoundWords<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.prior.is_none();
        let mut ch = self.iter.next();
        self.prior = ch;
        let is_last = self.iter.peek().is_none();

        if !is_first && !is_last && matches!(ch, Some('-')) {
            ch = self.iter.next();
        }
        ch
    }
}
