use crate::vocabulum::declinatio::Declinatio;
use crate::vocabulum::genus::Genus;
use crate::vocabulum::nomen::Nomen;
use crate::vocabulum::ParsOrationis;
use crate::Vocabulum;

#[test]
fn test_try_from_ascii_stem() {
    let puella = Nomen::try_from_ascii_stem("puell-", Declinatio::Prima, Genus::Femininum).unwrap();

    assert_eq!(10, puella.formae().len());
    assert_eq!("puella", puella.formae()[0].orthographia().to_string());
    assert_eq!("puellis", puella.formae()[9].orthographia().to_string());

    assert_eq!("puella", puella.lemma().orthographia().to_string());

    assert!(matches!(puella.pars_orationis(), ParsOrationis::Nomen(_)));
}
