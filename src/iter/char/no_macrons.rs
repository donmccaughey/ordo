use crate::iter::char::Iterators;
use crate::litterae;

pub struct NoMacrons<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> NoMacrons<I> {
    pub fn new(iter: I) -> NoMacrons<I> {
        NoMacrons { iter }
    }
}

impl<I: Iterator<Item = char>> Iterators for NoMacrons<I> {}

impl<I: Iterator<Item = char>> Iterator for NoMacrons<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(litterae::remove_macron)
    }
}
