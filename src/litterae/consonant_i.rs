use crate::litterae::CharTransforms;

pub struct ConsonantI<I> {
    iter: I,
}

impl<I: Iterator<Item = char>> ConsonantI<I> {
    pub fn new(iter: I) -> ConsonantI<I> {
        ConsonantI { iter }
    }
}

impl<I: Iterator<Item = char>> CharTransforms for ConsonantI<I> {}

impl<I: Iterator<Item = char>> Iterator for ConsonantI<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ch| match ch {
            'J' => 'I',
            'j' => 'i',
            _ => ch,
        })
    }
}
