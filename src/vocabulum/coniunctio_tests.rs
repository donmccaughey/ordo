use crate::vocabulum::coniunctio::Coniunctio;
use crate::vocabulum::ParsOrationis;
use crate::Vocabulum;

#[test]
fn test_try_from_ascii() {
    let et = Coniunctio::try_from_ascii("et").unwrap();

    assert_eq!(1, et.formae().len());
    assert_eq!("et", et.formae()[0].orthographia().to_string());
    assert_eq!("et", et.lemma().orthographia().to_string());
    assert!(matches!(et.pars_orationis(), ParsOrationis::Coniunctio(_)));
}
