use std::convert::TryFrom;

use crate::numerus::Numerus;

#[test]
fn test_div() {
    let viii = Numerus::try_from(8).unwrap();
    let iii = Numerus::try_from(3).unwrap();

    let ii = viii / iii;

    assert_eq!(2, ii.vis);
}

#[test]
#[should_panic]
fn test_div_nihil() {
    let iii = Numerus::try_from(3).unwrap();
    let iv = Numerus::try_from(4).unwrap();

    let _ = iii / iv;
}

#[test]
fn test_div_ref() {
    let xlii = Numerus::try_from(42).unwrap();
    let xi = Numerus::try_from(11).unwrap();

    let iii = &xlii / &xi;

    assert_eq!(3, iii.vis);
}

#[test]
fn test_div_assign() {
    let mut n = Numerus::try_from(19).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n /= vi;

    assert_eq!(3, n.vis);
}

#[test]
#[should_panic]
fn test_div_assign_nihil() {
    let mut n = Numerus::try_from(3).unwrap();
    let iv = Numerus::try_from(4).unwrap();

    n /= iv;
}

#[test]
fn test_div_assign_ref() {
    let mut n = Numerus::try_from(42).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n /= &vi;

    assert_eq!(7, n.vis);
}
