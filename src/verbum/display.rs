use crate::verbum::Verbum;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

impl Display for Verbum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.s)
    }
}
