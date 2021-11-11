use crate::errors::Irritus;
use crate::orthographia::Orthographia;
use std::collections::HashMap;

#[test]
fn test_clone() {
    let og1 = Orthographia::try_from_canonical("vacca").unwrap();
    let og2 = og1.clone();
    assert_eq!(og1.s, og2.s);
}

#[test]
fn test_eq() {
    let og1 = Orthographia::try_from_canonical("h\u{014d}ra").unwrap();
    let og2 = Orthographia::try_from_ascii("ho'ra").unwrap();
    assert_eq!(og1, og2);
}

#[test]
fn test_ord() {
    let dies = Orthographia::try_from_ascii("die's").unwrap();
    let hora = Orthographia::try_from_canonical("h\u{014d}ra").unwrap();
    assert!(dies < hora);
    assert!(hora > dies);
}

#[test]
fn test_hash() {
    let dies = Orthographia::try_from_ascii("die's").unwrap();
    let hora = Orthographia::try_from_ascii("ho'ra").unwrap();
    let mut dict = HashMap::new();
    dict.insert(dies, "day");
    dict.insert(hora, "hour");
    assert_eq!(dict.len(), 2);
}

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

#[test]
fn test_to_ascii_format() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("i'nfa'ns", og.to_ascii_format());

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("Ju'ppiter", og.to_ascii_format());

    // semivowel j
    let og = Orthographia::try_from_ascii("jam").unwrap();
    assert_eq!("jam", og.to_ascii_format());

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("ve'rum", og.to_ascii_format());

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("duo-decim", og.to_ascii_format());

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("magn-", og.to_ascii_format());

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("-que", og.to_ascii_format());
}

#[test]
fn test_to_classical_format() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("INFANS", og.to_classical_format());

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("IVPPITER", og.to_classical_format());

    // semivowel j
    let og = Orthographia::try_from_ascii("jam").unwrap();
    assert_eq!("IAM", og.to_classical_format());

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("VERVM", og.to_classical_format());

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("DVODECIM", og.to_classical_format());

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("MAGN-", og.to_classical_format());

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("-QVE", og.to_classical_format());
}

#[test]
fn test_to_modern_format() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("infans", og.to_modern_format());

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("Iuppiter", og.to_modern_format());

    // semivowel j
    let og = Orthographia::try_from_ascii("jam").unwrap();
    assert_eq!("iam", og.to_modern_format());

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("verum", og.to_modern_format());

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("duodecim", og.to_modern_format());

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("magn-", og.to_modern_format());

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("-que", og.to_modern_format());
}

#[test]
fn test_to_teaching_format() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("\u{012b}nf\u{0101}ns", og.to_teaching_format());

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("I\u{016b}ppiter", og.to_teaching_format());

    // semivowel j
    let og = Orthographia::try_from_ascii("jam").unwrap();
    assert_eq!("iam", og.to_teaching_format());

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("v\u{0113}rum", og.to_teaching_format());

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("duodecim", og.to_teaching_format());

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("magn-", og.to_teaching_format());

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("-que", og.to_teaching_format());
}
