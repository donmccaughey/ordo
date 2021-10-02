use crate::litterae::to_long_vowel;

#[test]
fn test_to_long_vowel() {
    assert!(matches!(to_long_vowel('A'), Some(CAPITAL_LONG_A)));
    assert!(matches!(to_long_vowel('Y'), Some(CAPITAL_LONG_Y)));

    assert!(matches!(to_long_vowel('a'), Some(SMALL_LONG_A)));
    assert!(matches!(to_long_vowel('y'), Some(SMALL_LONG_Y)));

    assert!(matches!(to_long_vowel('B'), None));
    assert!(matches!(to_long_vowel('s'), None));
}
