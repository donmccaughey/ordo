use crate::errors::Irritus;
use crate::verbum::Verbum;

#[test]
fn test_try_from_ascii() {
    let v = Verbum::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("i\u{0304}nfa\u{0304}ns", &v.s);

    // empty
    let v = Verbum::try_from_ascii("");
    assert!(matches!(v, Err(Irritus)));

    // leading (')
    let v = Verbum::try_from_ascii("'cauda");
    assert!(matches!(v, Err(Irritus)));

    // (') after consonant
    let v = Verbum::try_from_ascii("s'ol");
    assert!(matches!(v, Err(Irritus)));

    // invalid letter (w)
    let v = Verbum::try_from_ascii("wind");
    assert!(matches!(v, Err(Irritus)));

    // precomposed letter (i with macron '\u{012b}')
    let v = Verbum::try_from_ascii("\u{012b}nsula");
    assert!(matches!(v, Err(Irritus)));

    // punctuation
    let v = Verbum::try_from_ascii("S.P.Q.R.");
    assert!(matches!(v, Err(Irritus)));
}

#[test]
fn test_try_from_canonical() {
    let v = Verbum::try_from_canonical("i\u{0304}nfa\u{0304}ns").unwrap();
    assert_eq!("i\u{0304}nfa\u{0304}ns", &v.s);

    // empty
    let v = Verbum::try_from_canonical("");
    assert!(matches!(v, Err(Irritus)));

    // leading combining macron (\u{0304})
    let v = Verbum::try_from_canonical("\u{0304}cauda");
    assert!(matches!(v, Err(Irritus)));

    // combining macron (\u{0304}) after consonant
    let v = Verbum::try_from_canonical("s\u{0304}ol");
    assert!(matches!(v, Err(Irritus)));

    // invalid letter (w)
    let v = Verbum::try_from_canonical("wind");
    assert!(matches!(v, Err(Irritus)));

    // precomposed letter (i with macron '\u{012b}')
    let v = Verbum::try_from_canonical("\u{012b}nsula");
    assert!(matches!(v, Err(Irritus)));

    // punctuation
    let v = Verbum::try_from_canonical("S.P.Q.R.");
    assert!(matches!(v, Err(Irritus)));
}
