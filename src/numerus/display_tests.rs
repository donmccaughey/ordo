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
