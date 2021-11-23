use crate::iter::char::fallible::FallibleChars;

/// Method for getting a fallible `char` iterator.
pub trait CharSequences<'a> {
    fn fallible_chars(self) -> FallibleChars<'a>;
}

impl<'a> CharSequences<'a> for &'a str {
    fn fallible_chars(self) -> FallibleChars<'a> {
        FallibleChars::new(self.chars())
    }
}
