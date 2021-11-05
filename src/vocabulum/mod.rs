use crate::errors::Irritus;
use crate::litterae::filters::AsciiChars;
use crate::litterae::filters::CanonicalChars;
use crate::litterae::filters::InitialCaps;
use crate::litterae::filters::SoloHyphens;
use crate::litterae::*;
use std::iter::Peekable;
use std::str::Chars;

mod debug;
#[cfg(test)]
mod debug_tests;
mod default;
#[cfg(test)]
mod default_tests;
mod display;
#[cfg(test)]
mod display_tests;
#[cfg(test)]
mod mod_tests;

/// A single form of a Latin word.
///
/// An _Orthographia_ ("spelling") is a specific spelling of one form of a
/// Latin word.  It is a string-like immutable value type that encodes the word
/// in a canonical way using a limited set of characters.  Different
/// inflections of the same word, like _canis_ ("dog") and _canes_ ("dogs") are
/// separate _Orthographiae_.  Alternate spellings, like _verto_ and _vorto_
/// ("I turn") are also separate _Orthographiae_.
///
/// An _Orthographia_ can encode long vowels, as in _hōra_ ("hour") and
/// _īnfāns_ ("infant").  Consonant / semivowel uses of __I__ and __V__ can be
/// distinguished from vowels, as in the first letters of _Iulius_ ("Julius")
/// and _vērum_ ("truth").  Proper names are indicated by an initial capital
/// letter, so that the city of _Ostia_ is different than _ostia_ ("doors").
/// Compound words can be indicated with a hyphen, like _ex-īre_ ("to go out")
/// or _duo-decim_ ("twelve").
///
/// An _Orthographia_ can be formatted in a variety of ways for different
/// contexts, such as "Iūppiter", "Juppiter" or "IVPPITER".  The default
/// display format uses both "v" and "u" for __V__ but only "i" for __I__; long
/// vowel marks and compound word hyphens are omitted.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Orthographia {
    s: String,
}

impl Orthographia {
    /// Create an _Orthographia_ from an ASCII representation.
    ///
    /// The ASCII string must follow these rules:
    /// - long vowels are followed by a single quote (`'`) instead of a
    ///     [macron](https://en.wikipedia.org/wiki/Macron_(diacritic)) or an
    ///     [apex](https://en.wikipedia.org/wiki/Apex_(diacritic))
    /// - `'j'` and `'v'` are used when __I__ and __V__ are consonants / semivowels
    /// - `'i'` and `'u'` are used when __I__ and __V__ represent vowels
    /// - proper names start with a capital letter; all other letters are lower case
    /// - dipthongs like "ae" and "oe" are represented decomposed as separate letters
    /// - a hyphen between letters indicates a compound word
    /// - a hyphen at the end indicates that this is a word stem
    /// - a hyphen at the beginning indicates that this is a suffix
    /// - spaces, punctuation and non-Latin letters (including 'W') are prohibited
    ///
    /// Examples:
    /// - "ma'ter" (māter)
    /// - "jam" (iam)
    /// - "i'nsula" (īnsula)
    /// - "ve'rum" (vērum)
    /// - "Ju'ppiter" (Iūppiter)
    /// - "praeter" (præter)
    /// - "ex-i're" (compound word exīre)
    /// - "magn-" (stem of magnus)
    /// - "-que" (suffix, e.g. populusque)
    pub fn try_from_ascii(ascii: &str) -> Result<Orthographia, Irritus> {
        ascii
            .char_filter()
            .not_empty()
            .ascii_chars()
            .initial_caps()
            .long_vowel_ticks()
            .solo_hyphens()
            .collect::<Result<_, _>>()
            .map(|s| Orthographia { s })
    }

    /// Create an _Orthographia_ from a canonical UTF-8 representation.
    ///
    /// The canonical string must follow these rules:
    /// - long vowels are represented by precomposed Unicode letters with
    ///     macrons (`'Ā'`, `'Ē'`, etc. -- see [crate::litterae] for the list)
    ///     or by the plain vowel followed by a combining macron character
    ///     `'\u{0304}'`
    /// - `'j'` and `'v'` are used where __I__ and __V__ represent consonants or semivowels
    /// - `'i'` and `'u'` are used where __I__ and __V__ represent vowels
    /// - proper names start with a capital letter; all other letters are lower case
    /// - dipthongs like "ae" and "oe" are represented decomposed as separate letters
    /// - a hyphen between letters indicates a compound word
    /// - a hyphen at the end indicates that this is a word stem
    /// - a hyphen at the beginning indicates that this is a suffix
    /// - spaces, punctuation and non-Latin letters (including 'W') are prohibited
    ///
    /// Examples:
    /// - "ma\u{0304}ter" (combining macron, māter)
    /// - "jam" (iam)
    /// - "\u{012b}nsula" (precomposed long 'i', īnsula)
    /// - "ve\u{0304}rum" (vērum)
    /// - "Ju\u{0304}ppiter" (Iūppiter)
    /// - "praeter" (præter)
    /// - "ex-\u{012b}re" (compound word exīre)
    /// - "magn-" (stem of magnus)
    /// - "-que" (suffix, e.g. populusque)
    pub fn try_from_canonical(canonical: &str) -> Result<Orthographia, Irritus> {
        canonical
            .char_filter()
            .not_empty()
            .canonical_chars()
            .initial_caps()
            .long_vowel_macrons()
            .solo_hyphens()
            .collect::<Result<_, _>>()
            .map(|s| Orthographia { s })
    }

    /// Format an _Orthographia_ as a String in the classical way.
    ///
    /// The classical format uses only "v" for __V__ and only "i" for __I__;
    /// only capital letters are used.  Long vowel marks and compound word
    /// hyphens are omitted.
    pub fn to_classical_format(&self) -> String {
        self.s
            .chars()
            .all_caps()
            .consonant_i()
            .vowel_v()
            .no_macrons()
            .no_compound_words()
            .collect()
    }

    /// Format an _Orthographia_ as a String in the common modern way.
    ///
    /// The common modern format uses both "v" and "u" for __V__ but only "i"
    /// for __I__; long vowel marks and compound word hyphens are omitted.
    pub fn to_modern_format(&self) -> String {
        self.s
            .chars()
            .consonant_i()
            .no_macrons()
            .no_compound_words()
            .collect()
    }

    /// Format an _Orthographia_ as a String for teaching.
    ///
    /// The teaching format uses both "v" and "u" for __V__ but only "i"
    /// for __I__; long vowel marks are shown but compound word hyphens are
    /// omitted.
    pub fn to_teaching_format(&self) -> String {
        self.s.chars().consonant_i().no_compound_words().collect()
    }
}

pub trait ToCharFilter<'a> {
    fn char_filter(self) -> CharFilter<'a>;
}

impl<'a> ToCharFilter<'a> for &'a str {
    fn char_filter(self) -> CharFilter<'a> {
        CharFilter::new(self.chars())
    }
}

pub trait CharFilters: Iterator<Item = Result<char, Irritus>> + Sized {
    fn ascii_chars(self) -> AsciiChars<Self> {
        AsciiChars::new(self)
    }

    fn canonical_chars(self) -> CanonicalChars<Self> {
        CanonicalChars::new(self)
    }

    fn initial_caps(self) -> InitialCaps<Self> {
        InitialCaps::new(self)
    }

    fn long_vowel_macrons(self) -> LongVowelMacrons<Self> {
        LongVowelMacrons::new(self)
    }

    fn long_vowel_ticks(self) -> LongVowelTicks<Self> {
        LongVowelTicks::new(self)
    }

    fn not_empty(self) -> NotEmpty<Self> {
        NotEmpty::new(self)
    }

    fn solo_hyphens(self) -> SoloHyphens<Self> {
        SoloHyphens::new(self)
    }
}

pub struct CharFilter<'a> {
    chars: Chars<'a>,
}

impl<'a> CharFilter<'a> {
    pub fn new(chars: Chars<'a>) -> CharFilter<'a> {
        CharFilter { chars }
    }
}

impl<'a> Iterator for CharFilter<'a> {
    type Item = Result<char, Irritus>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(Ok)
    }
}

impl<'a> CharFilters for CharFilter<'a> {}

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

impl<I: Iterator<Item = Result<char, Irritus>>> CharFilters for LongVowelTicks<I> {}

pub struct NotEmpty<I> {
    iter: I,
    prior: Option<char>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> NotEmpty<I> {
    pub fn new(iter: I) -> NotEmpty<I> {
        NotEmpty { iter, prior: None }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for NotEmpty<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => {
                if self.prior.is_none() {
                    Some(Err(Irritus))
                } else {
                    None
                }
            }
            Some(result) => {
                if let Ok(ch) = result {
                    self.prior = Some(ch);
                }
                Some(result)
            }
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> CharFilters for NotEmpty<I> {}
