use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;
use crate::litterae::{is_short_vowel, to_long_vowel, MACRON};
use std::iter::Peekable;

/// Normalize combining macrons into precomposed long vowel characters.
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
        self.iter.next().map(|result| match result {
            Err(e) => Err(e),
            Ok(ch) => {
                if is_short_vowel(ch) {
                    if let Some(Ok(MACRON)) = self.iter.peek() {
                        self.iter.next();
                        Ok(to_long_vowel(ch))
                    } else {
                        Ok(ch)
                    }
                } else if ch == MACRON {
                    Err(Irritus)
                } else {
                    Ok(ch)
                }
            }
        })
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for LongVowelMacrons<I> {}
