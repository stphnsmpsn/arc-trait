extern crate arc_trait;

#[test]
fn test_auto_arc_trait() {
    let t = trybuild::TestCases::new();
    t.pass("tests/arc_trait_valid.rs");
}