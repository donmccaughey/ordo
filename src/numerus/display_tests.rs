use std::convert::TryFrom;

use crate::numerus::Numerus;

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
        (3888, "MMMDCCCLXXXVIII"),
        (3999, "MMMCMXCIX"),
    ];
    for (verus, expectans) in verus_expectans {
        let n = Numerus::try_from(verus).unwrap();
        assert_eq!(expectans, &n.to_string());
    }
}
