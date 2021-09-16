use std::{fmt, error};
use std::convert::TryFrom;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign};


#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Numerus {
    vis: u16,
}


impl Add for Numerus {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let summa = self.vis + rhs.vis;
        debug_assert!(summa < 4000);
        Numerus {
            vis: summa,
        }
    }
}


impl Add for &Numerus {
    type Output = Numerus;

    fn add(self, rhs: Self) -> Self::Output {
        Numerus::add(*self, *rhs)
    }
}


impl AddAssign for Numerus {
    fn add_assign(&mut self, rhs: Self) {
        let summa = self.vis + rhs.vis;
        debug_assert!(summa < 4000);
        self.vis = summa;
    }
}


impl AddAssign<&Numerus> for Numerus {
    fn add_assign(&mut self, rhs: &Numerus) {
        Numerus::add_assign(self, *rhs);
    }
}


impl Default for Numerus {
    fn default() -> Self {
        Numerus { vis: 1 }
    }
}


impl fmt::Display for Numerus {
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
    fn test_add() {
        let iii = Numerus::try_from(3).unwrap();
        let viii = Numerus::try_from(8).unwrap();

        let xi = iii + viii;

        assert_eq!(11, xi.vis);
    }

    #[test]
    #[should_panic]
    fn test_add_nimis() {
        let mmmcmxcix = Numerus::try_from(3999).unwrap();
        let i = Numerus::try_from(1).unwrap();

        let _ = mmmcmxcix + i;
    }

    #[test]
    fn test_add_ref() {
        let xlii = Numerus::try_from(42).unwrap();
        let xi = Numerus::try_from(11).unwrap();

        let liii = &xlii + &xi;

        assert_eq!(53, liii.vis);
    }

    #[test]
    fn test_add_assign() {
        let mut n = Numerus::try_from(11).unwrap();
        let vi = Numerus::try_from(6).unwrap();

        n += vi;

        assert_eq!(17, n.vis);
    }

    #[test]
    #[should_panic]
    fn test_add_assign_nimis() {
        let mut n = Numerus::try_from(3999).unwrap();
        let i = Numerus::try_from(1).unwrap();

        n += i;
    }

    #[test]
    fn test_add_assign_ref() {
        let mut n = Numerus::try_from(42).unwrap();
        let vi = Numerus::try_from(6).unwrap();

        n += &vi;

        assert_eq!(48, n.vis);
    }

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

    #[test]
    fn test_try_from() {
        let nihil = Numerus::try_from(0);
        assert!(nihil.is_err());

        let i = Numerus::try_from(1);
        assert!(i.is_ok());
        assert_eq!(1, i.unwrap().vis);

        let mmmcmxcix = Numerus::try_from(3999);
        assert!(mmmcmxcix.is_ok());
        assert_eq!(3999, mmmcmxcix.unwrap().vis);

        let mmmm = Numerus::try_from(4000);
        assert!(mmmm.is_err());
    }
}
