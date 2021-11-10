use crate::errors::Irritus;
use crate::iter::char::fallible::Iterators;

/// Accept uppercase as the first letter only.
pub struct InitialCaps<I> {
    iter: I,
    prior: Option<char>,
}

impl<I: Iterator<Item = Result<char, Irritus>>> InitialCaps<I> {
    pub fn new(iter: I) -> InitialCaps<I> {
        InitialCaps { iter, prior: None }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterator for InitialCaps<I> {
    type Item = Result<char, Irritus>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => {
                    match ch {
                        'A'..='V' | 'X'..='Z' => {
                            if self.prior.is_some() {
                                return Some(Err(Irritus));
                            }
                        }
                        _ => (),
                    }
                    self.prior = Some(ch);
                    Some(Ok(ch))
                }
            },
        }
    }
}

impl<I: Iterator<Item = Result<char, Irritus>>> Iterators for InitialCaps<I> {}
