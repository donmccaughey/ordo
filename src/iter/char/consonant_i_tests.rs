use crate::iter::char::Iterators;

#[test]
fn test_consonant_i() {
    let iam = "jam";
    let i = iam.chars().consonant_i().collect::<String>();
    assert_eq!("iam", i);

    let juppiter = "Juppiter";
    let i = juppiter.chars().consonant_i().collect::<String>();
    assert_eq!("Iuppiter", i);
}
