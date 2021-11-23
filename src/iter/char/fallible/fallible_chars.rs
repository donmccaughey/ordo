use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;
use std::str::Chars;

/// Adapt a `Chars` iterator into a fallible `char` iterator.
pub struct FallibleChars<'a> {
    chars: Chars<'a>,
}

impl<'a> FallibleChars<'a> {
    pub fn new(chars: Chars<'a>) -> FallibleChars<'a> {
        FallibleChars { chars }
    }
}

impl<'a> Iterator for FallibleChars<'a> {
    type Item = Result<char, Irritus>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(Ok)
    }
}

impl<'a> Iterators for FallibleChars<'a> {}
