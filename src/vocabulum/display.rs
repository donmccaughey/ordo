use crate::litterae::*;
use crate::vocabulum::Orthographia;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Format an _Orthographia_ for display in the common modern way.
///
/// The common format uses both "v" and "u" for __V__ but only "i" for __I__;
/// long vowel marks and compound word hyphens are omitted.
impl Display for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(self.s.len());
        for (i, ch) in self.s.char_indices() {
            match ch {
                '-' => {
                    if i == 0 || i == self.s.len() - 1 {
                        s.push(ch);
                    }
                }
                'J' => s.push('I'),
                'j' => s.push('i'),
                CAPITAL_LONG_A | CAPITAL_LONG_E | CAPITAL_LONG_I | CAPITAL_LONG_O
                | CAPITAL_LONG_U | CAPITAL_LONG_Y | SMALL_LONG_A | SMALL_LONG_E | SMALL_LONG_I
                | SMALL_LONG_O | SMALL_LONG_U | SMALL_LONG_Y => s.push(remove_macron(ch)),
                _ => s.push(ch),
            }
        }
        f.write_str(&s)
    }
}
