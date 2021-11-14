use crate::iter::char::Iterators;

#[test]
fn test_semivowel_i() {
    let iam = "jam";
    let i = iam.chars().semivowel_i().collect::<String>();
    assert_eq!("iam", i);

    let juppiter = "Juppiter";
    let i = juppiter.chars().semivowel_i().collect::<String>();
    assert_eq!("Iuppiter", i);
}
