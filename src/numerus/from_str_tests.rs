use crate::errors::Irritus;
use crate::numerus::test::ROMANI_ARABICI;
use crate::numerus::Numerus;

#[test]
fn test_from_str() {
    for (romani, arabici) in ROMANI_ARABICI {
        let n = romani.parse::<Numerus>().unwrap();
        assert_eq!(arabici, n.vis);
    }
}

#[test]
fn test_from_str_incompositus() {
    let incompositi = [("IIII", 4), ("XXXX", 40), ("CCCC", 400)];
    for (verus, expectans) in incompositi {
        let n = verus.parse::<Numerus>().unwrap();
        assert_eq!(expectans, n.vis);
    }
}

#[test]
fn test_from_str_irritus() {
    let irriti = ["", "A", "B", "E", "F", "G", "H", " I", "I ", "VX"];
    for irritus in irriti {
        let result = irritus.parse::<Numerus>();
        assert!(matches!(result, Err(Irritus)));
    }
}
