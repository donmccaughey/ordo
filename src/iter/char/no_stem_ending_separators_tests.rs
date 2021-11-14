use crate::iter::char::Iterators;

#[test]
fn test_no_stem_ending_separators() {
    let ending = "|us";
    let unchanged = ending
        .chars()
        .no_stem_ending_separators()
        .collect::<String>();
    assert_eq!("|us", unchanged);

    let stem = "naut|";
    let unchanged = stem.chars().no_stem_ending_separators().collect::<String>();
    assert_eq!("naut|", unchanged);

    let nauta = "naut|a";
    let no_separator = nauta
        .chars()
        .no_stem_ending_separators()
        .collect::<String>();
    assert_eq!("nauta", no_separator);
}
