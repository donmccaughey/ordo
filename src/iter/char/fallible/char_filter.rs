use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;
use std::str::Chars;

/// Adapt a `Chars` iterator into a fallible `char` iterator.
pub struct CharFilter<'a> {
    chars: Chars<'a>,
}

impl<'a> CharFilter<'a> {
    pub fn new(chars: Chars<'a>) -> CharFilter<'a> {
        CharFilter { chars }
    }
}

impl<'a> Iterator for CharFilter<'a> {
    type Item = Result<char, Irritus>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(Ok)
    }
}

impl<'a> Iterators for CharFilter<'a> {}
