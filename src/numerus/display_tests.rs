use std::convert::TryFrom;

use crate::numerus::test::ROMANI_ARABICI;
use crate::numerus::Numerus;

#[test]
fn test_to_strings() {
    for (romani, arabici) in ROMANI_ARABICI {
        let n = Numerus::try_from(arabici).unwrap();
        assert_eq!(romani, &n.to_string());
    }
}

#[test]
fn test_to_string_nihil() {
    let n = Numerus { vis: 0 };
    assert_eq!("nihil (0)", &n.to_string());
}

#[test]
fn test_to_string_nimis() {
    let n = Numerus { vis: 4000 };
    assert_eq!("nimis (4000)", &n.to_string());
}
