//! Useful Unicode characters

use std::iter::Peekable;
use std::str::Chars;

#[cfg(test)]
mod mod_tests;

/// Precomposed `'Ā'` character
pub const CAPITAL_LONG_A: char = '\u{0100}';

/// Precomposed `'ā'` character
pub const SMALL_LONG_A: char = '\u{0101}';

/// Precomposed `'Ē'` character
pub const CAPITAL_LONG_E: char = '\u{0112}';

/// Precomposed `'ē'` character
pub const SMALL_LONG_E: char = '\u{0113}';

/// Precomposed `'Ī'` character
pub const CAPITAL_LONG_I: char = '\u{012a}';

/// Precomposed `'ī'` character
pub const SMALL_LONG_I: char = '\u{012b}';

/// Precomposed `'Ō'` character
pub const CAPITAL_LONG_O: char = '\u{014c}';

/// Precomposed `'ō'` character
pub const SMALL_LONG_O: char = '\u{014d}';

/// Precomposed `'Ū'` character
pub const CAPITAL_LONG_U: char = '\u{016a}';

/// Precomposed `'ū'` character
pub const SMALL_LONG_U: char = '\u{016b}';

/// Precomposed `'Ȳ'` character
pub const CAPITAL_LONG_Y: char = '\u{0232}';

/// Precomposed `'ȳ'` character
pub const SMALL_LONG_Y: char = '\u{0233}';

/// Combining macron character `'¯'`
pub const MACRON: char = '\u{0304}';

static CAPITALS: [char; 31] = [
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'X',
    'Y',
    'Z',
    CAPITAL_LONG_A,
    CAPITAL_LONG_E,
    CAPITAL_LONG_I,
    CAPITAL_LONG_O,
    CAPITAL_LONG_U,
    CAPITAL_LONG_Y,
];

static SMALLS: [char; 31] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'x',
    'y',
    'z',
    SMALL_LONG_A,
    SMALL_LONG_E,
    SMALL_LONG_I,
    SMALL_LONG_O,
    SMALL_LONG_U,
    SMALL_LONG_Y,
];

static VOWELS: [char; 12] = ['A', 'E', 'I', 'O', 'U', 'Y', 'a', 'e', 'i', 'o', 'u', 'y'];

static LONG_VOWELS: [char; 12] = [
    CAPITAL_LONG_A,
    CAPITAL_LONG_E,
    CAPITAL_LONG_I,
    CAPITAL_LONG_O,
    CAPITAL_LONG_U,
    CAPITAL_LONG_Y,
    SMALL_LONG_A,
    SMALL_LONG_E,
    SMALL_LONG_I,
    SMALL_LONG_O,
    SMALL_LONG_U,
    SMALL_LONG_Y,
];

pub fn to_capital(ch: char) -> char {
    if let Some(i) = SMALLS.iter().position(|&s| s == ch) {
        CAPITALS[i]
    } else {
        ch
    }
}

pub fn to_long_vowel(ch: char) -> Option<char> {
    VOWELS.iter().position(|&v| v == ch).map(|i| LONG_VOWELS[i])
}

pub fn remove_macron(ch: char) -> char {
    if let Some(i) = LONG_VOWELS.iter().position(|&v| v == ch) {
        VOWELS[i]
    } else {
        ch
    }
}

pub trait CharTransforms: Iterator<Item = char> + Sized {
    fn all_caps(self) -> AllCaps<Self> {
        AllCaps::new(self)
    }

    fn consonant_i(self) -> ConsonantI<Self> {
        ConsonantI::new(self)
    }

    fn vowel_v(self) -> VowelV<Self> {
        VowelV::new(self)
    }

    fn no_compound_words(self) -> NoCompoundWords<Self> {
        NoCompoundWords::new(self)
    }

    fn no_macrons(self) -> NoMacrons<Self> {
        NoMacrons::new(self)
    }
}

impl<'a> CharTransforms for Chars<'a> {}

pub struct ConsonantI<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> ConsonantI<I> {
    fn new(iter: I) -> ConsonantI<I> {
        ConsonantI { iter }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for ConsonantI<I> {}

impl<I: Iterator<Item = char>> Iterator for ConsonantI<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ch| match ch {
            'J' => 'I',
            'j' => 'i',
            _ => ch,
        })
    }
}

pub struct VowelV<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> VowelV<I> {
    fn new(iter: I) -> VowelV<I> {
        VowelV { iter }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for VowelV<I> {}

impl<I: Iterator<Item = char>> Iterator for VowelV<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ch| match ch {
            'U' => 'V',
            'u' => 'v',
            CAPITAL_LONG_U => 'V',
            SMALL_LONG_U => 'v',
            _ => ch,
        })
    }
}

pub struct NoCompoundWords<I: Iterator<Item = char>> {
    iter: Peekable<I>,
    prior: Option<char>,
}

impl<I: Iterator<Item = char>> NoCompoundWords<I> {
    fn new(iter: I) -> NoCompoundWords<I> {
        NoCompoundWords {
            iter: iter.peekable(),
            prior: None,
        }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for NoCompoundWords<I> {}

impl<I: Iterator<Item = char>> Iterator for NoCompoundWords<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.prior.is_none();
        let mut ch = self.iter.next();
        let is_last = self.iter.peek().is_none();

        if !is_first && !is_last && matches!(ch, Some('-')) {
            ch = self.iter.next();
        }

        self.prior = ch;
        ch
    }
}

pub struct NoMacrons<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> NoMacrons<I> {
    fn new(iter: I) -> NoMacrons<I> {
        NoMacrons { iter }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for NoMacrons<I> {}

impl<I: Iterator<Item = char>> Iterator for NoMacrons<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(remove_macron)
    }
}

pub struct AllCaps<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> AllCaps<I> {
    fn new(iter: I) -> AllCaps<I> {
        AllCaps { iter }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for AllCaps<I> {}

impl<I: Iterator<Item = char>> Iterator for AllCaps<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(to_capital)
    }
}
