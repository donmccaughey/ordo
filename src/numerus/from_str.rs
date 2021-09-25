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

#[cfg(test)]
mod tests {
    use crate::numerus::Numerus;

    #[test]
    fn test_from_str() {
        let verus_expectans = [
            ("I", 1),
            ("II", 2),
            ("III", 3),
            ("IV", 4),
            ("V", 5),
            ("VI", 6),
            ("VII", 7),
            ("VIII", 8),
            ("IX", 9),
            ("X", 10),
            ("XI", 11),
            ("XII", 12),
            ("XIII", 13),
            ("XIV", 14),
            ("XV", 15),
            ("XVI", 16),
            ("XVII", 17),
            ("XVIII", 18),
            ("XIX", 19),
            ("XX", 20),
            ("XXX", 30),
            ("XL", 40),
            ("L", 50),
            ("LX", 60),
            ("LXX", 70),
            ("LXXX", 80),
            ("XC", 90),
            ("C", 100),
            ("CC", 200),
            ("CCC", 300),
            ("CD", 400),
            ("D", 500),
            ("DC", 600),
            ("DCC", 700),
            ("DCCC", 800),
            ("CM", 900),
            ("M", 1000),
            ("MM", 2000),
            ("MMM", 3000),
            // examples from https://en.wikipedia.org/wiki/Roman_numerals
            // in section "Standard form"
            ("XXXIX", 39),
            ("CCXLVI", 246),
            ("DCCLXXXIX", 789),
            ("MMCDXXI", 2421),
            ("CLX", 160),
            ("CCVII", 207),
            ("MIX", 1009),
            ("MLXVI", 1066),
            ("MDCCLXXVI", 1776),
            ("MCMXVIII", 1918),
            ("MCMLIV", 1954),
            ("MMXIV", 2014),
            ("MMMCMXCIX", 3999),
        ];
        for (verus, expectans) in verus_expectans {
            let n = verus.parse::<Numerus>().unwrap();
            assert_eq!(expectans, n.vis);
        }
    }

    #[test]
    fn test_from_str_incompositus() {
        let incompositi = [("IIII", 4), ("XXXX", 40), ("CCCC", 400)];
        for (verus, expectans) in incompositi {
            let n = verus.parse::<Numerus>().unwrap();
            assert_eq!(expectans, n.vis);
        }
    }

    #[test]
    fn test_from_str_irritus() {
        let irriti = ["", "A", "B", "E", "F", "G", "H", " I", "I ", "VX"];
        for irritus in irriti {
            assert!(irritus.parse::<Numerus>().is_err());
        }
    }
}
