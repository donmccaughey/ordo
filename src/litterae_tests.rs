use crate::litterae::*;

#[test]
fn test_to_long_vowel() {
    assert!(matches!(to_long_vowel('A'), Some(CAPITAL_LONG_A)));
    assert!(matches!(to_long_vowel('Y'), Some(CAPITAL_LONG_Y)));

    assert!(matches!(to_long_vowel('a'), Some(SMALL_LONG_A)));
    assert!(matches!(to_long_vowel('y'), Some(SMALL_LONG_Y)));

    assert!(matches!(to_long_vowel('B'), None));
    assert!(matches!(to_long_vowel('s'), None));
}

#[test]
fn test_remove_macron() {
    assert_eq!(remove_macron(CAPITAL_LONG_A), 'A');
    assert_eq!(remove_macron(CAPITAL_LONG_Y), 'Y');
    assert_eq!(remove_macron(SMALL_LONG_A), 'a');
    assert_eq!(remove_macron(SMALL_LONG_Y), 'y');

    assert_eq!(remove_macron('A'), 'A');
    assert_eq!(remove_macron('Y'), 'Y');
    assert_eq!(remove_macron('a'), 'a');
    assert_eq!(remove_macron('y'), 'y');

    assert_eq!(remove_macron('B'), 'B');
    assert_eq!(remove_macron('s'), 's');
}
