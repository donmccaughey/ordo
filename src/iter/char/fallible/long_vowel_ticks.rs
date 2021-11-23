use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;
use crate::litterae::{is_short_vowel, to_long_vowel};
use std::iter::Peekable;

/// Normalize trailing ASCII ticks (`'`) into precomposed long vowel characters.
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
        self.iter.next().map(|result| match result {
            Err(e) => Err(e),
            Ok(ch) => {
                if is_short_vowel(ch) {
                    if let Some(Ok('\'')) = self.iter.peek() {
                        self.iter.next();
                        Ok(to_long_vowel(ch))
                    } else {
                        Ok(ch)
                    }
                } else if ch == '\'' {
                    Err(Irritus)
                } else {
                    Ok(ch)
                }
            }
        })
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for LongVowelTicks<I> {}
