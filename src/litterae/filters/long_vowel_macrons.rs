use crate::errors::Irritus;
use crate::litterae::{is_short_vowel, to_long_vowel, MACRON};
use crate::vocabulum::CharFilters;
use std::iter::Peekable;

pub struct LongVowelMacrons<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> LongVowelMacrons<I> {
    pub fn new(iter: I) -> LongVowelMacrons<I> {
        LongVowelMacrons {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for LongVowelMacrons<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => Some(if is_short_vowel(ch) {
                    Ok(if let Some(Ok(MACRON)) = self.iter.peek() {
                        self.iter.next();
                        to_long_vowel(ch)
                    } else {
                        ch
                    })
                } else if ch == MACRON {
                    Err(Irritus)
                } else {
                    Ok(ch)
                }),
            },
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> CharFilters for LongVowelMacrons<I> {}
