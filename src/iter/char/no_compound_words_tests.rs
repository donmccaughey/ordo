use crate::iter::char::Iterators;

#[test]
fn test_no_compound_words() {
    let ab_sunt = "ab-sunt";
    let absunt = ab_sunt.chars().no_compound_words().collect::<String>();
    assert_eq!("absunt", absunt);

    let suffix = "-que";
    let unchanged = suffix.chars().no_compound_words().collect::<String>();
    assert_eq!("-que", unchanged);
}
