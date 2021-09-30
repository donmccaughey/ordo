use crate::errors::Irritus;
use crate::litterae::COMBINING_MACRON;

mod default;
#[cfg(test)]
mod default_tests;
mod display;
#[cfg(test)]
mod display_tests;
#[cfg(test)]
mod mod_tests;

/// A Latin word.
///
/// A _Verbum_ is a Latin word with metadata, including part of speech, number, gender and case or
/// tense.
///
/// A _Verbum_ contains a canonical spelling of the word, distinguishing between long and short
/// vowels and between vowel and consonant uses of __I__ and __V__.  This allows a _Verbum_
/// to be formatted in different ways, such as "Iūppiter", "Juppiter" or "IVPPITER".
#[derive(Debug)]
pub struct Verbum {
    s: String,
}

impl Verbum {
    /// Create a _Verbum_ from an ASCII representation.
    ///
    /// The ASCII string must follow these rules:
    /// - long vowels are followed by a single quote (`'`) instead of a
    ///     [macron](https://en.wikipedia.org/wiki/Macron_(diacritic)) or an
    ///     [apex](https://en.wikipedia.org/wiki/Apex_(diacritic))
    /// - `'j'` and `'v'` are used where __I__ and __V__ represent consonants or semivowels
    /// - `'i'` and `'u'` are used where __I__ and __V__ represent vowels
    /// - proper names start with a capital letter; all other letters are lower case
    /// - dipthongs like "ae" and "oe" are represented decomposed as separate letters
    /// - spaces, punctuation and non-Latin letters (including 'W') are prohibited
    ///
    /// Examples:
    /// - "ma'ter" (māter)
    /// - "jam" (iam)
    /// - "i'nsula" (īnsula)
    /// - "ve'rum" (vērum)
    /// - "u'nus" (ūnus)
    /// - "Ju'ppiter" (Iūppiter)
    /// - "praeter" (præter)
    pub fn try_from_ascii(ascii: &str) -> Result<Verbum, Irritus> {
        if ascii.is_empty() {
            return Err(Irritus);
        }
        let mut s = String::with_capacity(ascii.len());
        let mut prior: Option<char> = None;
        for ch in ascii.chars() {
            match ch {
                'A'..='V' | 'X'..='Z' | 'a'..='v' | 'x'..='z' => s.push(ch),
                '\'' => {
                    if prior.is_none() {
                        return Err(Irritus);
                    }
                    let prior = prior.unwrap();
                    if !"AEIOUYaeiouy".contains(prior) {
                        return Err(Irritus);
                    }
                    s.push(COMBINING_MACRON);
                }
                _ => return Err(Irritus),
            }
            prior = Some(ch);
        }
        Ok(Verbum { s })
    }

    /// Create a _Verbum_ from a canonical decomposed UTF-8 representation.
    ///
    /// The canonical string must follow these rules:
    /// - long vowels are followed by a combining macron character `'\u{0304}'`
    /// - `'j'` and `'v'` are used where __I__ and __V__ represent consonants or semivowels
    /// - `'i'` and `'u'` are used where __I__ and __V__ represent vowels
    /// - proper names start with a capital letter; all other letters are lower case
    /// - dipthongs like "ae" and "oe" are represented decomposed as separate letters
    /// - spaces, punctuation and non-Latin letters (including 'W') are prohibited
    ///
    /// Examples:
    /// - "ma\u{0304}ter" (māter)
    /// - "jam" (iam)
    /// - "i\u{0304}nsula" (īnsula)
    /// - "ve\u{0304}rum" (vērum)
    /// - "u\u{0304}nus" (ūnus)
    /// - "Ju\u{0304}ppiter" (Iūppiter)
    /// - "praeter" (præter)
    pub fn try_from_canonical(canonical: &str) -> Result<Verbum, Irritus> {
        if canonical.is_empty() {
            return Err(Irritus);
        }
        let mut s = String::with_capacity(canonical.len());
        let mut prior: Option<char> = None;
        for ch in canonical.chars() {
            match ch {
                'A'..='V' | 'X'..='Z' | 'a'..='v' | 'x'..='z' => s.push(ch),
                COMBINING_MACRON => {
                    if prior.is_none() {
                        return Err(Irritus);
                    }
                    let prior = prior.unwrap();
                    if !"AEIOUYaeiouy".contains(prior) {
                        return Err(Irritus);
                    }
                    s.push(COMBINING_MACRON);
                }
                _ => return Err(Irritus),
            }
            prior = Some(ch);
        }
        Ok(Verbum { s })
    }
}
