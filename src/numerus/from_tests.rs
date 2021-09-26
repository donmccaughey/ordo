use std::convert::TryFrom;

use crate::numerus::Numerus;

#[test]
fn test_into_u16() {
    let xlii = Numerus::try_from(42).unwrap();
    let u: u16 = xlii.into();
    assert_eq!(42, u);
}
