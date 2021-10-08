use crate::litterae::*;

#[test]
fn test_is_long_vowel() {
    assert!(!is_long_vowel('A'));
    assert!(is_long_vowel(CAPITAL_LONG_A));
    assert!(!is_long_vowel('a'));
    assert!(is_long_vowel(SMALL_LONG_A));

    assert!(!is_long_vowel('Y'));
    assert!(is_long_vowel(CAPITAL_LONG_Y));
    assert!(!is_long_vowel('y'));
    assert!(is_long_vowel(SMALL_LONG_Y));
}

#[test]
fn test_is_short_vowel() {
    assert!(is_short_vowel('A'));
    assert!(!is_short_vowel(CAPITAL_LONG_A));
    assert!(is_short_vowel('a'));
    assert!(!is_short_vowel(SMALL_LONG_A));

    assert!(is_short_vowel('Y'));
    assert!(!is_short_vowel(CAPITAL_LONG_Y));
    assert!(is_short_vowel('y'));
    assert!(!is_short_vowel(SMALL_LONG_Y));
}

#[test]
fn test_to_capital() {
    assert_eq!(to_capital('A'), 'A');
    assert_eq!(to_capital(CAPITAL_LONG_A), CAPITAL_LONG_A);

    assert_eq!(to_capital('a'), 'A');
    assert_eq!(to_capital(SMALL_LONG_A), CAPITAL_LONG_A);

    assert_eq!(to_capital('Z'), 'Z');
    assert_eq!(to_capital('z'), 'Z');

    assert_eq!(to_capital('-'), '-');
}

#[test]
fn test_to_long_vowel() {
    assert_eq!(to_long_vowel('A'), CAPITAL_LONG_A);
    assert_eq!(to_long_vowel('Y'), CAPITAL_LONG_Y);

    assert_eq!(to_long_vowel('a'), SMALL_LONG_A);
    assert_eq!(to_long_vowel('y'), SMALL_LONG_Y);

    assert_eq!(to_long_vowel('B'), 'B');
    assert_eq!(to_long_vowel('s'), 's');
    assert_eq!(to_long_vowel('-'), '-');
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
    assert_eq!(remove_macron('-'), '-');
}
