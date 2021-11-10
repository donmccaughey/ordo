use crate::errors::Irritus;
use crate::numerus::Numerus;

use std::str::FromStr;

static MILLIPI: [(&str, u16); 3] = [("MMM", 3000), ("MM", 2000), ("M", 1000)];

static CENTUPLI: [(&str, u16); 9] = [
    ("CM", 900),
    ("DCCC", 800),
    ("DCC", 700),
    ("DC", 600),
    ("D", 500),
    ("CD", 400),
    ("CCC", 300),
    ("CC", 200),
    ("C", 100),
];

static DECUPLI: [(&str, u16); 9] = [
    ("XC", 90),
    ("LXXX", 80),
    ("LXX", 70),
    ("LX", 60),
    ("L", 50),
    ("XL", 40),
    ("XXX", 30),
    ("XX", 20),
    ("X", 10),
];

static SIMPLI: [(&str, u16); 9] = [
    ("IX", 9),
    ("VIII", 8),
    ("VII", 7),
    ("VI", 6),
    ("V", 5),
    ("IV", 4),
    ("III", 3),
    ("II", 2),
    ("I", 1),
];

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
    /// let whitespace_is_invalid = Numerus::from_str(" XVI");
    /// assert!(matches!(whitespace_is_invalid, Err(Irritus)));
    ///
    /// let lowercase_is_invalid = Numerus::from_str("xvi");
    /// assert!(matches!(lowercase_is_invalid, Err(Irritus)));
    ///
    /// let punctuation_is_invalid = Numerus::from_str("XVI.");
    /// assert!(matches!(punctuation_is_invalid, Err(Irritus)));
    ///
    /// let malformed = Numerus::from_str("IIC");
    /// assert!(matches!(malformed, Err(Irritus)));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut summa: u16 = 0;
        let mut digiti = s;

        let mut minue = |positus: &[(&str, u16)]| {
            for (litterae, vis) in positus {
                if let Some(s) = digiti.strip_prefix(litterae) {
                    summa += vis;
                    digiti = s;
                    break;
                }
            }
        };

        minue(&MILLIPI);
        minue(&CENTUPLI);
        minue(&DECUPLI);
        minue(&SIMPLI);

        if digiti.is_empty() && summa > 0 {
            Ok(Numerus { vis: summa })
        } else {
            Err(Irritus)
        }
    }
}
