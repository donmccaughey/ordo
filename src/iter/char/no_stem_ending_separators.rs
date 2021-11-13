use crate::iter::char::Iterators;
use std::iter::Peekable;

/// Omits `|` separators in words and complex endings.
pub struct NoStemEndingSeparators<I: Iterator<Item = char>> {
    iter: Peekable<I>,
    prior: Option<char>,
}

impl<I: Iterator<Item = char>> NoStemEndingSeparators<I> {
    pub fn new(iter: I) -> NoStemEndingSeparators<I> {
        NoStemEndingSeparators {
            iter: iter.peekable(),
            prior: None,
        }
    }
}

impl<I: Iterator<Item = char>> Iterators for NoStemEndingSeparators<I> {}

impl<I: Iterator<Item = char>> Iterator for NoStemEndingSeparators<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.prior.is_none();
        let mut ch = self.iter.next();
        let is_last = self.iter.peek().is_none();

        if !is_first && !is_last && matches!(ch, Some('|')) {
            ch = self.iter.next();
        }

        self.prior = ch;
        ch
    }
}
