use crate::iter::char::Iterators;

/// Transforms semivowel __J__ into __I__.
pub struct SemivowelI<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> SemivowelI<I> {
    pub fn new(iter: I) -> SemivowelI<I> {
        SemivowelI { iter }
    }
}

impl<I: Iterator<Item = char>> Iterators for SemivowelI<I> {}

impl<I: Iterator<Item = char>> Iterator for SemivowelI<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ch| match ch {
            'J' => 'I',
            'j' => 'i',
            _ => ch,
        })
    }
}
