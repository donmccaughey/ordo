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
    let incompositi = ["MMMM", "CCCC", "XXXX", "IIII"];
    for incompositus in incompositi {
        let result = incompositus.parse::<Numerus>();
        assert!(matches!(result, Err(Irritus)));
    }
}

#[test]
fn test_from_str_irritus_litterae() {
    let irriti = [
        "A", "B", "E", "F", "G", "H", "J", "K", "N", "O", "P", "Q", "R", "S", "T", "U", "W", "Y",
        "Z", "c", "d", "i", "l", "m", "v", "x",
    ];
    for irritus in irriti {
        let result = irritus.parse::<Numerus>();
        assert!(matches!(result, Err(Irritus)));
    }
}

#[test]
fn test_from_str_irritus_formae() {
    let irriti = ["", " I", "I ", "I.", "IA", "VX", "IL", "VL", "VIL", "CCM"];
    for irritus in irriti {
        let result = irritus.parse::<Numerus>();
        assert!(matches!(result, Err(Irritus)));
    }
}
