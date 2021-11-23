use crate::iter::char::fallible::FallibleChars;

/// Method for getting a fallible `char` iterator.
pub trait Filter<'a> {
    fn filter(self) -> FallibleChars<'a>;
}

impl<'a> Filter<'a> for &'a str {
    fn filter(self) -> FallibleChars<'a> {
        FallibleChars::new(self.chars())
    }
}
