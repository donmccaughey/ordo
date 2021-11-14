use crate::iter::char::Iterators;

#[test]
fn test_stem_hyphens() {
    let ending = "|us";
    let unchanged = ending.chars().stem_hyphens().collect::<String>();
    assert_eq!("|us", unchanged);

    let stem = "naut|";
    let with_hyphen = stem.chars().stem_hyphens().collect::<String>();
    assert_eq!("naut-", with_hyphen);

    let nauta = "naut|a";
    let unchanged = nauta.chars().stem_hyphens().collect::<String>();
    assert_eq!("naut|a", unchanged);
}
