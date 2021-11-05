use crate::errors::Irritus;
use crate::litterae::filters::CharFilters;
use crate::litterae::{
    CAPITAL_LONG_A, CAPITAL_LONG_E, CAPITAL_LONG_I, CAPITAL_LONG_O, CAPITAL_LONG_U, CAPITAL_LONG_Y,
    MACRON, SMALL_LONG_A, SMALL_LONG_E, SMALL_LONG_I, SMALL_LONG_O, SMALL_LONG_U, SMALL_LONG_Y,
};

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
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => match ch {
                    '\'' | '-' => Some(Ok(ch)),
                    'A'..='V' | 'X'..='Z' => Some(Ok(ch)),
                    'a'..='v' | 'x'..='z' => Some(Ok(ch)),
                    CAPITAL_LONG_A | SMALL_LONG_A => Some(Ok(ch)),
                    CAPITAL_LONG_E | SMALL_LONG_E => Some(Ok(ch)),
                    CAPITAL_LONG_I | SMALL_LONG_I => Some(Ok(ch)),
                    CAPITAL_LONG_O | SMALL_LONG_O => Some(Ok(ch)),
                    CAPITAL_LONG_U | SMALL_LONG_U => Some(Ok(ch)),
                    CAPITAL_LONG_Y | SMALL_LONG_Y => Some(Ok(ch)),
                    MACRON => Some(Ok(ch)),
                    _ => Some(Err(Irritus)),
                },
            },
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> CharFilters for CanonicalChars<I> {}
