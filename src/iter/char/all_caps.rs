use crate::iter::char::Iterators;
use crate::litterae;

/// Transforms `char`s to uppercase letters.
pub struct AllCaps<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> AllCaps<I> {
    pub fn new(iter: I) -> AllCaps<I> {
        AllCaps { iter }
    }
}

impl<I: Iterator<Item = char>> Iterators for AllCaps<I> {}

impl<I: Iterator<Item = char>> Iterator for AllCaps<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(litterae::to_capital)
    }
}
