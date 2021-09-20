use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;
use std::{error, fmt};

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Numerus {
    vis: u16,
}

impl Add for Numerus {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let summa = self.vis + rhs.vis;
        debug_assert!(summa < 4000);
        Numerus { vis: summa }
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

#[derive(Debug)]
struct Irritus;

impl Display for Irritus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "irritus")
    }
}

impl error::Error for Irritus {}

impl FromStr for Numerus {
    type Err = Irritus;

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

impl Sub for Numerus {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let summa = self.vis - rhs.vis;
        debug_assert!(summa > 0);
        Numerus { vis: summa }
    }
}

impl Sub for &Numerus {
    type Output = Numerus;

    fn sub(self, rhs: Self) -> Self::Output {
        Numerus::sub(*self, *rhs)
    }
}

impl SubAssign for Numerus {
    fn sub_assign(&mut self, rhs: Self) {
        let summa = self.vis - rhs.vis;
        debug_assert!(summa > 0);
        self.vis = summa;
    }
}

impl SubAssign<&Numerus> for Numerus {
    fn sub_assign(&mut self, rhs: &Numerus) {
        Numerus::sub_assign(self, *rhs);
    }
}

#[derive(Debug)]
struct Nimis;

impl Display for Nimis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "nimis")
    }
}

impl error::Error for Nimis {}

impl TryFrom<u16> for Numerus {
    type Error = Nimis;

    fn try_from(vis: u16) -> Result<Self, Self::Error> {
        if (1..=3999).contains(&vis) {
            Ok(Numerus { vis })
        } else {
            Err(Nimis)
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

    #[test]
    fn test_sub() {
        let viii = Numerus::try_from(8).unwrap();
        let iii = Numerus::try_from(3).unwrap();

        let v = viii - iii;

        assert_eq!(5, v.vis);
    }

    #[test]
    #[should_panic]
    fn test_sub_nimis_nihil() {
        let a = Numerus::try_from(7).unwrap();
        let b = Numerus::try_from(7).unwrap();

        let _ = a - b;
    }

    #[test]
    #[should_panic]
    fn test_sub_nimis() {
        let a = Numerus::try_from(7).unwrap();
        let b = Numerus::try_from(8).unwrap();

        let _ = a - b;
    }

    #[test]
    fn test_sub_ref() {
        let xlii = Numerus::try_from(42).unwrap();
        let xi = Numerus::try_from(11).unwrap();

        let xxxi = &xlii - &xi;

        assert_eq!(31, xxxi.vis);
    }

    #[test]
    fn test_sub_assign() {
        let mut n = Numerus::try_from(11).unwrap();
        let vi = Numerus::try_from(6).unwrap();

        n -= vi;

        assert_eq!(5, n.vis);
    }

    #[test]
    #[should_panic]
    fn test_sub_assign_nimis_nihil() {
        let mut n = Numerus::try_from(10).unwrap();
        let x = Numerus::try_from(10).unwrap();

        n -= x;
    }

    #[test]
    #[should_panic]
    fn test_sub_assign_nimis() {
        let mut n = Numerus::try_from(10).unwrap();
        let ix = Numerus::try_from(11).unwrap();

        n -= ix;
    }

    #[test]
    fn test_sub_assign_ref() {
        let mut n = Numerus::try_from(42).unwrap();
        let vi = Numerus::try_from(6).unwrap();

        n -= &vi;

        assert_eq!(36, n.vis);
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
