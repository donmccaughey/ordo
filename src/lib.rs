use std::{fmt, error};
use std::convert::TryFrom;
use std::fmt::Formatter;


#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Numerus {
    vis: u16,
}


impl Default for Numerus {
    fn default() -> Self {
        Numerus { vis: 1 }
    }
}


fn minue(summa: u16, litterae: &str, reliquum: &mut u16, s: &mut String) {
    while *reliquum >= summa {
        *s += litterae;
        *reliquum -= summa;
    }
}


impl fmt::Display for Numerus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_assert!(self.vis != 0);
        debug_assert!(self.vis < 4000);

        let mut s = String::default();
        let mut reliquum = self.vis;

        minue(1000, "M", &mut reliquum, &mut s);
        minue(900, "CM", &mut reliquum, &mut s);
        minue(500, "D", &mut reliquum, &mut s);
        minue(400, "CD", &mut reliquum, &mut s);
        minue(100, "C", &mut reliquum, &mut s);
        minue(90, "XC", &mut reliquum, &mut s);
        minue(50, "L", &mut reliquum, &mut s);
        minue(40, "XL", &mut reliquum, &mut s);
        minue(10, "X", &mut reliquum, &mut s);
        minue(9, "IX", &mut reliquum, &mut s);
        minue(5, "V", &mut reliquum, &mut s);
        minue(4, "IV", &mut reliquum, &mut s);
        minue(1, "I", &mut reliquum, &mut s);

        debug_assert_eq!(0, reliquum);

        f.write_str(&s)
    }
}


impl From<Numerus> for u16 {
    fn from(n: Numerus) -> Self {
        n.vis
    }
}


#[derive(Debug)]
struct Nimis;


impl fmt::Display for Nimis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "nimis")
    }
}


impl error::Error for Nimis {}


impl TryFrom<u16> for Numerus {
    type Error = Nimis;

    fn try_from(vis: u16) -> Result<Self, Self::Error> {
        if vis < 1 || vis > 3999 {
            Err(Nimis)
        } else {
            Ok(Numerus { vis })
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Numerus;
    use std::convert::TryFrom;

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
        let n = Numerus::try_from(42).unwrap();
        let u: u16 = n.into();
        assert_eq!(42, u);
    }

    #[test]
    fn test_try_from() {
        let n0 = Numerus::try_from(0);
        assert!(n0.is_err());

        let n1 = Numerus::try_from(1);
        assert!(n1.is_ok());
        assert_eq!(1, n1.unwrap().vis);

        let n3999 = Numerus::try_from(3999);
        assert!(n3999.is_ok());
        assert_eq!(3999, n3999.unwrap().vis);

        let n4000 = Numerus::try_from(4000);
        assert!(n4000.is_err());
    }
}
