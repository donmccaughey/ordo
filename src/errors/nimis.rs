use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// "Beyond measure" ([_nimis_](https://logeion.uchicago.edu/nimis)) error that
/// indicates input outside the valid range.
#[derive(Debug)]
pub struct Nimis;

impl Display for Nimis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "nimis")
    }
}

impl error::Error for Nimis {}
