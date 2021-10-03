use crate::numerus::Numerus;
use std::str::FromStr;
use std::convert::TryFrom;
use std::collections::HashMap;

#[test]
fn test_copy() {
    let mut n1 = Numerus::from_str("XXII").unwrap();
    let mut n2 = n1; // this would be a move in Numerus isn't Copy

    // verify that n1 and n2 are both valid
    n1 += Numerus::from_str("I").unwrap();
    n2 += Numerus::from_str("II").unwrap();
    assert_eq!(23, n1.vis);
    assert_eq!(24, n2.vis);
}

#[test]
fn test_clone() {
    let n1 = Numerus::from_str("XXIII").unwrap();
    let n2 = n1.clone();
    assert_eq!(n1.vis, n2.vis);
}

#[test]
fn test_eq() {
    let n1 = Numerus::from_str("XXIII").unwrap();
    let n2 = Numerus::try_from(23).unwrap();
    assert_eq!(n1.vis, n2.vis);
}

#[test]
fn test_ord() {
    let xxiii = Numerus::from_str("XXIII").unwrap();
    let xxiv = Numerus::try_from(24).unwrap();
    assert!(xxiii < xxiv);
    assert!(xxiv > xxiii);
}

#[test]
fn test_hash() {
    let x = Numerus::from_str("X").unwrap();
    let xx = Numerus::try_from(20).unwrap();
    let mut dict = HashMap::new();
    dict.insert(x, "decem");
    dict.insert(xx, "viginti");
    assert_eq!(dict.len(), 2);
}
