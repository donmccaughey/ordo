use crate::numerus::Numerus;
use std::convert::TryFrom;

#[test]
fn test_try_from() {
    let nihil = Numerus::try_from(0);
    assert!(nihil.is_err());

    let i = Numerus::try_from(1);
    assert!(i.is_ok());
    assert_eq!(1, i.unwrap().vis);

    let mmmcmxcix = Numerus::try_from(3999);
    assert!(mmmcmxcix.is_ok());
    assert_eq!(3999, mmmcmxcix.unwrap().vis);

    let mmmm = Numerus::try_from(4000);
    assert!(mmmm.is_err());
}
