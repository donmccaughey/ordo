use crate::iter::char::Iterators;
use std::iter::Peekable;

/// Indicate word stems with a trailing `-` character.
pub struct StemHyphens<I: Iterator<Item = char>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = char>> StemHyphens<I> {
    pub fn new(iter: I) -> StemHyphens<I> {
        StemHyphens {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator<Item = char>> Iterators for StemHyphens<I> {}

impl<I: Iterator<Item = char>> Iterator for StemHyphens<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ch = self.iter.next();
        let is_last = self.iter.peek().is_none();

        if is_last && matches!(ch, Some('|')) {
            ch = Some('-');
        }
        ch
    }
}
