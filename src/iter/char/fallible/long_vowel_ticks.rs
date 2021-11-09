use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;
use crate::litterae::{is_short_vowel, to_long_vowel};
use std::iter::Peekable;

pub struct LongVowelTicks<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> LongVowelTicks<I> {
    pub fn new(iter: I) -> LongVowelTicks<I> {
        LongVowelTicks {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for LongVowelTicks<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => Some(if is_short_vowel(ch) {
                    Ok(if let Some(Ok('\'')) = self.iter.peek() {
                        self.iter.next();
                        to_long_vowel(ch)
                    } else {
                        ch
                    })
                } else if ch == '\'' {
                    Err(Irritus)
                } else {
                    Ok(ch)
                }),
            },
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for LongVowelTicks<I> {}
