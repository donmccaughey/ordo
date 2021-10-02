use crate::vocabulum::Orthographia;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

impl Display for Orthographia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.s)
    }
}
