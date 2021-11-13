use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;

/// Reject two or more adjacent vertical line characters.
pub struct SoloPipes<I> {
    iter: I,
    prior: Option<char>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> SoloPipes<I> {
    pub fn new(iter: I) -> SoloPipes<I> {
        SoloPipes { iter, prior: None }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for SoloPipes<I> {
    type Item = Result<char, Irritus>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => {
                    if ch == '|' {
                        if let Some('|') = self.prior {
                            return Some(Err(Irritus));
                        }
                    }
                    self.prior = Some(ch);
                    Some(Ok(ch))
                }
            },
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for SoloPipes<I> {}
