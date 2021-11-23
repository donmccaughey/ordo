use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;

/// Reject two or more adjacent hyphens.
pub struct SoloHyphens<I> {
    iter: I,
    prior: Option<char>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> SoloHyphens<I> {
    pub fn new(iter: I) -> SoloHyphens<I> {
        SoloHyphens { iter, prior: None }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for SoloHyphens<I> {
    type Item = Result<char, Irritus>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|result| match result {
            Err(e) => Err(e),
            Ok(ch) => {
                if ch == '-' {
                    if let Some('-') = self.prior {
                        return Err(Irritus);
                    }
                }
                self.prior = Some(ch);
                Ok(ch)
            }
        })
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for SoloHyphens<I> {}
