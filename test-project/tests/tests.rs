#[test]
pub fn pass() {
    let t = macrotest::TestCases::new();
    t.pass("tests/expand/*.rs");
}
