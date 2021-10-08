use crate::litterae::{CharTransforms, CAPITAL_LONG_U, SMALL_LONG_U};

pub struct VowelV<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> VowelV<I> {
    pub fn new(iter: I) -> VowelV<I> {
        VowelV { iter }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for VowelV<I> {}

impl<I: Iterator<Item = char>> Iterator for VowelV<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ch| match ch {
            'U' => 'V',
            'u' => 'v',
            CAPITAL_LONG_U => 'V',
            SMALL_LONG_U => 'v',
            _ => ch,
        })
    }
}
