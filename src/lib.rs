use std::fmt;
use std::fmt::Formatter;


struct RomanNumeral(u16);


impl fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            0 => "nihil",
            1 => "I",
            2 => "II",
            _ => "?",
        };
        f.write_str(s)
    }
}


#[cfg(test)]
mod tests {
    use crate::RomanNumeral;

    #[test]
    fn to_strings() {
        let actual_expected = [
            (0, "nihil"),
            (1, "I"),
            (2, "II"),
        ];
        for (actual, expected) in actual_expected {
            assert_eq!(expected, &RomanNumeral(actual).to_string());
        }
    }
}
