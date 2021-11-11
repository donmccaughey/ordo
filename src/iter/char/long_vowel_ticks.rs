use crate::iter::char::Iterators;
use crate::litterae::{is_long_vowel, remove_macron};

/// Transform macrons into ASCII ticks (`'`).
pub struct LongVowelTicks<I> {
    iter: I,
    buffer: Option<char>,
}

impl<I: Iterator<Item = char>> LongVowelTicks<I> {
    pub fn new(iter: I) -> LongVowelTicks<I> {
        LongVowelTicks { iter, buffer: None }
    }
}

impl<I: Iterator<Item = char>> Iterators for LongVowelTicks<I> {}

impl<I: Iterator<Item = char>> Iterator for LongVowelTicks<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.take().or_else(|| {
            self.iter.next().map(|ch| {
                if is_long_vowel(ch) {
                    self.buffer = Some('\'');
                    remove_macron(ch)
                } else {
                    ch
                }
            })
        })
    }
}
