use std::convert::TryFrom;

use crate::numerus::Numerus;

#[test]
fn test_rem() {
    let viii = Numerus::try_from(8).unwrap();
    let iii = Numerus::try_from(3).unwrap();

    let ii = viii % iii;

    assert_eq!(2, ii.vis);
}

#[test]
#[should_panic]
fn test_rem_nihil() {
    let ix = Numerus::try_from(9).unwrap();
    let iii = Numerus::try_from(3).unwrap();

    let _ = ix % iii;
}

#[test]
fn test_rem_ref() {
    let xlii = Numerus::try_from(42).unwrap();
    let xi = Numerus::try_from(11).unwrap();

    let ix = &xlii % &xi;

    assert_eq!(9, ix.vis);
}

#[test]
fn test_rem_assign() {
    let mut n = Numerus::try_from(11).unwrap();
    let vi = Numerus::try_from(6).unwrap();

    n %= vi;

    assert_eq!(5, n.vis);
}

#[test]
#[should_panic]
fn test_rem_assign_nimis() {
    let mut n = Numerus::try_from(25).unwrap();
    let v = Numerus::try_from(5).unwrap();

    n %= v;
}

#[test]
fn test_rem_assign_ref() {
    let mut n = Numerus::try_from(42).unwrap();
    let v = Numerus::try_from(5).unwrap();

    n %= &v;

    assert_eq!(2, n.vis);
}
