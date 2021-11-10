use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// "Invalid" ([_irritus_](https://logeion.uchicago.edu/irritus)) error that
/// indicates invalid or malformed input.
#[derive(Debug)]
pub struct Irritus;

impl Display for Irritus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "irritus")
    }
}

impl error::Error for Irritus {}
