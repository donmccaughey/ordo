mod add;
#[cfg(test)]
mod add_tests;
mod from_str;
#[cfg(test)]
mod from_str_tests;
mod sub;
#[cfg(test)]
mod sub_tests;
mod try_from;
#[cfg(test)]
mod try_from_tests;

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form) Roman numeral in
/// the range `1..=3999` (__I__ to __MMMCMXCIX__).
///
/// _Numerus_ is an integer value type that represents a Roman numeral like __XVII__ or __IX__.
/// _Numeri_ are unsigned integers with a limited range that display as Roman numerals.
///
/// Create _Numeri_ from integers using [`try_from()`](std::convert::TryFrom) or
/// [`try_into()`](std::convert::TryInto).  Get a string representation of a _Numerus_ using
/// [`to_string()`](ToString) or [`format!()`].
/// ```
/// use ordo::Numerus;
/// use std::convert::TryFrom;
/// use std::convert::TryInto;
///
/// let xxvi = Numerus::try_from(26).unwrap();
/// assert_eq!("XXVI", &xxvi.to_string());
///
/// let cxi: Numerus = 111.try_into().unwrap();
/// assert_eq!("CXI", &format!("{}", cxi));
/// ```
///
/// Create _Numeri_ from strings using [`from_str()`](std::str::FromStr) or [`str::parse()`].  Read the
/// integer value of a _Numerus_ using [`from()`](From) or [`into()`](Into).
/// ```
/// use ordo::Numerus;
/// use std::str::FromStr;
///
/// let xvii = Numerus::from_str("XVII").unwrap();
/// assert_eq!(17, u16::from(xvii));
///
/// let xcix = "XCIX".parse::<Numerus>().unwrap();
/// assert_eq!(99_u16, xcix.into());
/// ```
///
/// _Numeri_ act like unsigned integers, implementing traits [Eq], [Ord], [Hash], [std::ops::Add],
/// [std::ops::AddAssign], [std::ops::Sub] and [std::ops::SubAssign].
/// ```
/// use ordo::Numerus;
/// use std::convert::TryFrom;
///
/// let xxxv = Numerus::try_from(35).unwrap();
/// let xvii = "XVII".parse::<Numerus>().unwrap();
/// let ix = Numerus::try_from(9).unwrap();
///
/// assert!(xxxv > ix);
/// assert!(ix <= xvii);
/// assert_ne!(xxxv, ix);
///
/// let xxvi = xxxv - ix;
/// assert_eq!("XXVI", &xxvi.to_string());
///
/// let xxvi = xvii + ix;
/// assert_eq!("XVII", &xvii.to_string());
/// ```
///
/// Like other Rust integer types, _Numeri_ will panic on overflow in debug builds.
/// ```should_panic
/// use ordo::Numerus;
/// use std::convert::TryFrom;
///
/// let mmmcmxcix = Numerus::try_from(3999).unwrap();
/// let i = Numerus::try_from(1).unwrap();
///
/// /// panics
/// let nimis = mmmcmxcix + i;
/// ```
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Numerus {
    vis: u16,
}

impl Default for Numerus {
    fn default() -> Self {
        Numerus { vis: 1 }
    }
}

impl Display for Numerus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_assert!(self.vis != 0);
        debug_assert!(self.vis < 4000);

        let mut s = String::default();
        let mut reliquum = self.vis;

        let mut minue = |summa: u16, litterae: &str| {
            while reliquum >= summa {
                s += litterae;
                reliquum -= summa;
            }
        };

        minue(1000, "M");
        minue(900, "CM");
        minue(500, "D");
        minue(400, "CD");
        minue(100, "C");
        minue(90, "XC");
        minue(50, "L");
        minue(40, "XL");
        minue(10, "X");
        minue(9, "IX");
        minue(5, "V");
        minue(4, "IV");
        minue(1, "I");

        debug_assert_eq!(0, reliquum);

        f.write_str(&s)
    }
}

impl From<Numerus> for u16 {
    fn from(n: Numerus) -> Self {
        n.vis
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::numerus::Numerus;

    #[test]
    fn test_default() {
        let n = Numerus::default();
        assert_eq!("I", &n.to_string());
    }

    #[test]
    fn test_to_strings() {
        let verus_expectans = [
            (1, "I"),
            (2, "II"),
            (3, "III"),
            (4, "IV"),
            (5, "V"),
            (6, "VI"),
            (7, "VII"),
            (8, "VIII"),
            (9, "IX"),
            (10, "X"),
            (11, "XI"),
            (12, "XII"),
            (13, "XIII"),
            (14, "XIV"),
            (15, "XV"),
            (16, "XVI"),
            (17, "XVII"),
            (18, "XVIII"),
            (19, "XIX"),
            (20, "XX"),
            (30, "XXX"),
            (40, "XL"),
            (50, "L"),
            (60, "LX"),
            (70, "LXX"),
            (80, "LXXX"),
            (90, "XC"),
            (100, "C"),
            (200, "CC"),
            (300, "CCC"),
            (400, "CD"),
            (500, "D"),
            (600, "DC"),
            (700, "DCC"),
            (800, "DCCC"),
            (900, "CM"),
            (1000, "M"),
            (2000, "MM"),
            (3000, "MMM"),
            // examples from https://en.wikipedia.org/wiki/Roman_numerals
            // in section "Standard form"
            (39, "XXXIX"),
            (246, "CCXLVI"),
            (789, "DCCLXXXIX"),
            (2421, "MMCDXXI"),
            (160, "CLX"),
            (207, "CCVII"),
            (1009, "MIX"),
            (1066, "MLXVI"),
            (1776, "MDCCLXXVI"),
            (1918, "MCMXVIII"),
            (1954, "MCMLIV"),
            (2014, "MMXIV"),
            (3999, "MMMCMXCIX"),
        ];
        for (verus, expectans) in verus_expectans {
            let n = Numerus::try_from(verus).unwrap();
            assert_eq!(expectans, &n.to_string());
        }
    }

    #[test]
    fn test_into_u16() {
        let xlii = Numerus::try_from(42).unwrap();
        let u: u16 = xlii.into();
        assert_eq!(42, u);
    }
}
