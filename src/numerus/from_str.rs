use crate::errors::Irritus;
use crate::numerus::Numerus;

use std::str::FromStr;

impl FromStr for Numerus {
    type Err = Irritus;

    /// Create a _Numerus_ from a string.
    /// ```
    /// use ordo::Numerus;
    /// use std::str::FromStr;
    ///
    /// let xvii = Numerus::from_str("XVII").unwrap();
    /// assert_eq!(17_u16, xvii.into());
    ///
    /// let xcix = "XCIX".parse::<Numerus>().unwrap();
    /// assert_eq!(99_u16, xcix.into());
    /// ```
    ///
    /// This is a strict parse that only recognizes characters `MDCLXVI`.  Valid Roman numeral strings
    /// must follow the [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form)
    /// rules.  For invalid strings, `Err(Irritus)` (_invalid_) is returned.
    /// ```
    /// use ordo::Numerus;
    /// use std::str::FromStr;
    ///
    /// /// whitespace is invalid
    /// let result = Numerus::from_str(" XVI");
    /// assert!(matches!(result, Err(Irritus)));
    ///
    /// /// lower case is invalid
    /// let result = Numerus::from_str("xvi");
    /// assert!(matches!(result, Err(Irritus)));
    ///
    /// /// punctuation is invalid
    /// let result = Numerus::from_str("XVI.");
    /// assert!(matches!(result, Err(Irritus)));
    ///
    /// /// malformed Roman numeral
    /// let result = Numerus::from_str("IIC");
    /// assert!(matches!(result, Err(Irritus)));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut summa: u16 = 0;
        let mut prior: char = ' ';

        for ch in s.chars() {
            if ch == 'I' {
                summa += 1;
            } else if ch == 'V' {
                if prior == 'I' {
                    summa += 3;
                } else {
                    summa += 5;
                }
            } else if ch == 'X' {
                summa += match prior {
                    ' ' => 10,
                    'I' => 8,
                    'V' => return Err(Irritus),
                    _ => 10,
                };
            } else if ch == 'L' {
                summa += match prior {
                    ' ' => 50,
                    'I' => return Err(Irritus),
                    'V' => return Err(Irritus),
                    'X' => 30,
                    _ => 50,
                }
            } else if ch == 'C' {
                summa += match prior {
                    ' ' => 100,
                    'I' => return Err(Irritus),
                    'V' => return Err(Irritus),
                    'X' => 80,
                    _ => 100,
                }
            } else if ch == 'D' {
                summa += match prior {
                    ' ' => 500,
                    'I' => return Err(Irritus),
                    'V' => return Err(Irritus),
                    'X' => return Err(Irritus),
                    'C' => 300,
                    _ => 500,
                }
            } else if ch == 'M' {
                summa += match prior {
                    ' ' => 1000,
                    'I' => return Err(Irritus),
                    'V' => return Err(Irritus),
                    'X' => return Err(Irritus),
                    'C' => 800,
                    _ => 1000,
                }
            } else {
                return Err(Irritus);
            }
            prior = ch;
        }

        if summa > 0 {
            Ok(Numerus { vis: summa })
        } else {
            Err(Irritus)
        }
    }
}
