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


impl fmt::Display for Numerus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self.vis {
            0 => panic!(),
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            10 => "X",
            4000.. => panic!(),
            _ => "?",
        };
        f.write_str(s)
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
