//! Useful Unicode characters

pub use all_caps::AllCaps;
pub use char_transforms::CharTransforms;
pub use consonant_i::ConsonantI;
pub use no_compound_words::NoCompoundWords;
pub use no_macrons::NoMacrons;
pub use vowel_v::VowelV;

mod all_caps;
mod char_transforms;
mod consonant_i;
pub mod filters;
#[cfg(test)]
mod mod_tests;
mod no_compound_words;
mod no_macrons;
mod vowel_v;

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

pub fn is_long_vowel(ch: char) -> bool {
    ch == CAPITAL_LONG_A
        || ch == SMALL_LONG_A
        || ch == CAPITAL_LONG_E
        || ch == SMALL_LONG_E
        || ch == CAPITAL_LONG_I
        || ch == SMALL_LONG_I
        || ch == CAPITAL_LONG_O
        || ch == SMALL_LONG_O
        || ch == CAPITAL_LONG_U
        || ch == SMALL_LONG_U
        || ch == CAPITAL_LONG_Y
        || ch == SMALL_LONG_Y
}

pub fn is_short_vowel(ch: char) -> bool {
    ch == 'A'
        || ch == 'E'
        || ch == 'I'
        || ch == 'O'
        || ch == 'U'
        || ch == 'Y'
        || ch == 'a'
        || ch == 'e'
        || ch == 'i'
        || ch == 'o'
        || ch == 'u'
        || ch == 'y'
}

pub fn to_capital(ch: char) -> char {
    match ch {
        SMALL_LONG_A => CAPITAL_LONG_A,
        SMALL_LONG_E => CAPITAL_LONG_E,
        SMALL_LONG_I => CAPITAL_LONG_I,
        SMALL_LONG_O => CAPITAL_LONG_O,
        SMALL_LONG_U => CAPITAL_LONG_U,
        SMALL_LONG_Y => CAPITAL_LONG_Y,
        _ => ch.to_ascii_uppercase(),
    }
}

pub fn to_long_vowel(ch: char) -> char {
    match ch {
        'A' => CAPITAL_LONG_A,
        'E' => CAPITAL_LONG_E,
        'I' => CAPITAL_LONG_I,
        'O' => CAPITAL_LONG_O,
        'U' => CAPITAL_LONG_U,
        'Y' => CAPITAL_LONG_Y,
        'a' => SMALL_LONG_A,
        'e' => SMALL_LONG_E,
        'i' => SMALL_LONG_I,
        'o' => SMALL_LONG_O,
        'u' => SMALL_LONG_U,
        'y' => SMALL_LONG_Y,
        _ => ch,
    }
}

pub fn remove_macron(ch: char) -> char {
    match ch {
        CAPITAL_LONG_A => 'A',
        SMALL_LONG_A => 'a',
        CAPITAL_LONG_E => 'E',
        SMALL_LONG_E => 'e',
        CAPITAL_LONG_I => 'I',
        SMALL_LONG_I => 'i',
        CAPITAL_LONG_O => 'O',
        SMALL_LONG_O => 'o',
        CAPITAL_LONG_U => 'U',
        SMALL_LONG_U => 'u',
        CAPITAL_LONG_Y => 'Y',
        SMALL_LONG_Y => 'y',
        _ => ch,
    }
}
