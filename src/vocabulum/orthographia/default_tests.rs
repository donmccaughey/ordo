use crate::vocabulum::orthographia::Orthographia;

#[test]
fn test_default() {
    let forma = Orthographia::default();
    assert_eq!("n\u{016b}llus", &forma.s);
}
