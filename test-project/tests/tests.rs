#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
pub fn pass_expect_expanded() {
    // If you delete one of the `.expanded.rs` files, this test will fail.
    macrotest::expand_without_refresh("tests/expand/*.rs");
}

#[test]
#[should_panic]
pub fn fail_expect_expanded() {
    // This directory doesn't have expanded files but since they're expected, the test will fail.
    macrotest::expand_without_refresh("tests/no_expanded/*.rs");
}
