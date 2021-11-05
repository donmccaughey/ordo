use crate::litterae::filters::CharFilter;

pub trait ToCharFilter<'a> {
    fn char_filter(self) -> CharFilter<'a>;
}

impl<'a> ToCharFilter<'a> for &'a str {
    fn char_filter(self) -> CharFilter<'a> {
        CharFilter::new(self.chars())
    }
}
