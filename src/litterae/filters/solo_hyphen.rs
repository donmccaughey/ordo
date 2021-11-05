use crate::errors::Irritus;
use crate::litterae::filters::CharFilters;

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
        match self.iter.next() {
            None => None,
            Some(result) => match result {
                Err(e) => Some(Err(e)),
                Ok(ch) => {
                    if ch == '-' {
                        if let Some('-') = self.prior {
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

impl<I: Iterator<Item = Result<char, Irritus>>> CharFilters for SoloHyphens<I> {}
