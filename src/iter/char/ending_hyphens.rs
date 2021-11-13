use crate::iter::char::Iterators;

/// Indicate word endings with a leading `-` character.
pub struct EndingHyphens<I> {
    iter: I,
    prior: Option<char>,
}

impl<I: Iterator<Item = char>> EndingHyphens<I> {
    pub fn new(iter: I) -> EndingHyphens<I> {
        EndingHyphens { iter, prior: None }
    }
}

impl<I: Iterator<Item = char>> Iterators for EndingHyphens<I> {}

impl<I: Iterator<Item = char>> Iterator for EndingHyphens<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.prior.is_none();
        let mut ch = self.iter.next();
        self.prior = ch;

        if is_first && matches!(ch, Some('|')) {
            ch = Some('-');
        }
        ch
    }
}
