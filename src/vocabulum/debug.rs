use crate::litterae::*;
use crate::Orthographia;
use std::fmt::{Debug, Formatter};

impl Debug for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(self.s.len());
        for ch in self.s.chars() {
            match ch {
                CAPITAL_LONG_A | CAPITAL_LONG_E | CAPITAL_LONG_I | CAPITAL_LONG_O
                | CAPITAL_LONG_U | CAPITAL_LONG_Y | SMALL_LONG_A | SMALL_LONG_E | SMALL_LONG_I
                | SMALL_LONG_O | SMALL_LONG_U | SMALL_LONG_Y => {
                    s.push(remove_macron(ch));
                    s.push('\'');
                }
                _ => s.push(ch),
            }
        }
        write!(f, "\"{}\"", &s)
    }
}
