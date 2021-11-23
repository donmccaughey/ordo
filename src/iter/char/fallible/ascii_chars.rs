use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;

/// Accept only valid characters from the ASCII representation of Latin words.
pub struct AsciiChars<I> {
    iter: I,
}

impl<I: Iterator<Item = Result<char, Irritus>>> AsciiChars<I> {
    pub fn new(iter: I) -> AsciiChars<I> {
        AsciiChars { iter }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for AsciiChars<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|result| match result {
            Err(e) => Err(e),
            Ok(ch) => match ch {
                '\'' | '-' => Ok(ch),
                'A'..='V' | 'X'..='Z' => Ok(ch),
                'a'..='v' | 'x'..='z' => Ok(ch),
                '|' => Ok(ch),
                _ => Err(Irritus),
            },
        })
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for AsciiChars<I> {}
