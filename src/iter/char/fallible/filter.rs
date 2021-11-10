use crate::iter::char::fallible::CharFilter;

/// Method for getting a fallible `char` iterator.
pub trait Filter<'a> {
    fn filter(self) -> CharFilter<'a>;
}

impl<'a> Filter<'a> for &'a str {
    fn filter(self) -> CharFilter<'a> {
        CharFilter::new(self.chars())
    }
}
