use crate::vocabulum::orthographia::Orthographia;

#[test]
fn test_debug_format() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("\"i'nfa'ns\"", format!("{:?}", og));

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("\"Ju'ppiter\"", format!("{:?}", og));

    // semivowel j
    let og = Orthographia::try_from_ascii("jam").unwrap();
    assert_eq!("\"jam\"", format!("{:?}", og));

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("\"ve'rum\"", format!("{:?}", og));

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("\"duo-decim\"", format!("{:?}", og));

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("\"magn-\"", format!("{:?}", og));

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("\"-que\"", format!("{:?}", og));
}
