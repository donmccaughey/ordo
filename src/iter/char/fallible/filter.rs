use crate::iter::char::fallible::CharFilter;

pub trait Filter<'a> {
    fn filter(self) -> CharFilter<'a>;
}

impl<'a> Filter<'a> for &'a str {
    fn filter(self) -> CharFilter<'a> {
        CharFilter::new(self.chars())
    }
}
