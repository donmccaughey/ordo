use crate::vocabulum::orthographia::Orthographia;

#[test]
fn test_to_string() {
    // long vowels
    let og = Orthographia::try_from_ascii("i'nfa'ns").unwrap();
    assert_eq!("infans", og.to_string());

    // semivowel j, vowel i and initial capital
    let og = Orthographia::try_from_ascii("Ju'ppiter").unwrap();
    assert_eq!("Iuppiter", og.to_string());

    // semivowel j
    let og = Orthographia::try_from_ascii("jam").unwrap();
    assert_eq!("iam", og.to_string());

    // semivowel v and vowel u
    let og = Orthographia::try_from_ascii("ve'rum").unwrap();
    assert_eq!("verum", og.to_string());

    // compound word
    let og = Orthographia::try_from_ascii("duo-decim").unwrap();
    assert_eq!("duodecim", og.to_string());

    // stem
    let og = Orthographia::try_from_ascii("magn-").unwrap();
    assert_eq!("magn-", og.to_string());

    // suffix
    let og = Orthographia::try_from_ascii("-que").unwrap();
    assert_eq!("-que", og.to_string());
}
