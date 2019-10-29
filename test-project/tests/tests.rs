use macrotest::TestCases;

#[test]
pub fn run() {
    TestCases::new().pass("tests/expand/*.rs");
}
