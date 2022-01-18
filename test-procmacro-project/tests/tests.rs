#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
#[should_panic]
pub fn pass_fail() {
    macrotest::expand("tests/expand-fail/*.rs");
}

#[test]
pub fn fail() {
    macrotest::expand_fail("tests/expand-fail/*.rs");
}
