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
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => match ch {
                    '\'' | '-' => Some(Ok(ch)),
                    'A'..='V' | 'X'..='Z' => Some(Ok(ch)),
                    'a'..='v' | 'x'..='z' => Some(Ok(ch)),
                    _ => Some(Err(Irritus)),
                },
            },
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for AsciiChars<I> {}
