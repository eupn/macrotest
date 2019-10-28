use macrotest::TestCases;
use std::env::current_dir;

#[test]
pub fn run() {
    TestCases::new().pass("tests/expand/*.rs");
}
