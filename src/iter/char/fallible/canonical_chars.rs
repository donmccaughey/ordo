use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;
use crate::litterae::{
    CAPITAL_LONG_A, CAPITAL_LONG_E, CAPITAL_LONG_I, CAPITAL_LONG_O, CAPITAL_LONG_U, CAPITAL_LONG_Y,
    MACRON, SMALL_LONG_A, SMALL_LONG_E, SMALL_LONG_I, SMALL_LONG_O, SMALL_LONG_U, SMALL_LONG_Y,
};

/// Accept only valid characters from the canonical representation of Latin words.
pub struct CanonicalChars<I> {
    iter: I,
}

impl<I: Iterator<Item = Result<char, Irritus>>> CanonicalChars<I> {
    pub fn new(iter: I) -> CanonicalChars<I> {
        CanonicalChars { iter }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for CanonicalChars<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|result| match result {
            Err(e) => Err(e),
            Ok(ch) => match ch {
                '\'' | '-' => Ok(ch),
                'A'..='V' | 'X'..='Z' => Ok(ch),
                'a'..='v' | 'x'..='z' => Ok(ch),
                '|' => Ok(ch),
                CAPITAL_LONG_A | SMALL_LONG_A => Ok(ch),
                CAPITAL_LONG_E | SMALL_LONG_E => Ok(ch),
                CAPITAL_LONG_I | SMALL_LONG_I => Ok(ch),
                CAPITAL_LONG_O | SMALL_LONG_O => Ok(ch),
                CAPITAL_LONG_U | SMALL_LONG_U => Ok(ch),
                CAPITAL_LONG_Y | SMALL_LONG_Y => Ok(ch),
                MACRON => Ok(ch),
                _ => Err(Irritus),
            },
        })
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for CanonicalChars<I> {}
