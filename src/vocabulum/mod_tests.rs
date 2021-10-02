use crate::errors::Irritus;
use crate::vocabulum::Orthographia;

#[test]
fn test_try_from_ascii() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("\u{012b}nf\u{0101}ns", &og.s);

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("J\u{016b}ppiter", &og.s);

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("v\u{0113}rum", &og.s);

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("duo-decim", &og.s);

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("magn-", &og.s);

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("-que", &og.s);

    // errors

    // empty
    let og = Orthographia::try_from_ascii("");
    assert!(matches!(og, Err(Irritus)));

    // leading (')
    let og = Orthographia::try_from_ascii("'cauda");
    assert!(matches!(og, Err(Irritus)));

    // (') after consonant
    let og = Orthographia::try_from_ascii("s'ol");
    assert!(matches!(og, Err(Irritus)));

    // double hyphen
    let og = Orthographia::try_from_ascii("ex--it");
    assert!(matches!(og, Err(Irritus)));

    // non-initial capital letter
    let og = Orthographia::try_from_ascii("ITA");
    assert!(matches!(og, Err(Irritus)));

    // invalid letter (w)
    let og = Orthographia::try_from_ascii("wind");
    assert!(matches!(og, Err(Irritus)));

    // precomposed letter (i with macron '\u{012b}')
    let og = Orthographia::try_from_ascii("\u{012b}nsula");
    assert!(matches!(og, Err(Irritus)));

    // punctuation
    let og = Orthographia::try_from_ascii("S.P.Q.R.");
    assert!(matches!(og, Err(Irritus)));
}

#[test]
fn test_try_from_canonical() {
    // long vowels, combining macrons
    let og = Orthographia::try_from_canonical("i\u{0304}nfa\u{0304}ns").unwrap();
    assert_eq!("\u{012b}nf\u{0101}ns", &og.s);

    // long vowels, precomposed macron characters
    let og = Orthographia::try_from_canonical("i\u{0304}nf\u{0101}ns").unwrap();
    assert_eq!("\u{012b}nf\u{0101}ns", &og.s);

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_canonical("J\u{016b}ppiter").unwrap();
    assert_eq!("J\u{016b}ppiter", &og.s);

    // semivowel v and vowel u
    let og = Orthographia::try_from_canonical("ve\u{0304}rum").unwrap();
    assert_eq!("v\u{0113}rum", &og.s);

    // compound word
    let og = Orthographia::try_from_canonical("duo-decim").unwrap();
    assert_eq!("duo-decim", &og.s);

    // stem
    let og = Orthographia::try_from_canonical("magn-").unwrap();
    assert_eq!("magn-", &og.s);

    // suffix
    let og = Orthographia::try_from_canonical("-que").unwrap();
    assert_eq!("-que", &og.s);

    // errors

    // empty
    let og = Orthographia::try_from_canonical("");
    assert!(matches!(og, Err(Irritus)));

    // leading combining macron (\u{0304})
    let og = Orthographia::try_from_canonical("\u{0304}cauda");
    assert!(matches!(og, Err(Irritus)));

    // combining macron (\u{0304}) after consonant
    let og = Orthographia::try_from_canonical("s\u{0304}ol");
    assert!(matches!(og, Err(Irritus)));

    // double hyphen
    let og = Orthographia::try_from_canonical("ex--it");
    assert!(matches!(og, Err(Irritus)));

    // non-initial capital letter
    let og = Orthographia::try_from_canonical("ITA");
    assert!(matches!(og, Err(Irritus)));

    // invalid letter (w)
    let og = Orthographia::try_from_canonical("wind");
    assert!(matches!(og, Err(Irritus)));

    // punctuation
    let og = Orthographia::try_from_canonical("S.P.Q.R.");
    assert!(matches!(og, Err(Irritus)));
}
