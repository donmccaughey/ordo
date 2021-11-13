use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;

/// Accept only non empty `char` sequences.
pub struct NotEmpty<I> {
    iter: I,
    prior: Option<char>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> NotEmpty<I> {
    pub fn new(iter: I) -> NotEmpty<I> {
        NotEmpty { iter, prior: None }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for NotEmpty<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => {
                if self.prior.is_none() {
                    Some(Err(Irritus))
                } else {
                    None
                }
            }
            Some(result) => {
                if let Ok(ch) = result {
                    self.prior = Some(ch);
                }
                Some(result)
            }
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for NotEmpty<I> {}
