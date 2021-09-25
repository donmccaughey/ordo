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
