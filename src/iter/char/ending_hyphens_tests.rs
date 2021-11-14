use crate::iter::char::Iterators;

#[test]
fn test_ending_hyphens() {
    let ending = "|us";
    let with_hyphen = ending.chars().ending_hyphens().collect::<String>();
    assert_eq!("-us", with_hyphen);

    let stem = "naut|";
    let unchanged = stem.chars().ending_hyphens().collect::<String>();
    assert_eq!("naut|", unchanged);

    let nauta = "naut|a";
    let unchanged = nauta.chars().ending_hyphens().collect::<String>();
    assert_eq!("naut|a", unchanged);
}
