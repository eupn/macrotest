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

#[test]
pub fn pass_args() {
    macrotest::expand_args("tests/expand_args/*.rs", &["--features", "test-feature"]);
}

#[test]
pub fn pass_expect_expanded_args() {
    // If you delete one of the `.expanded.rs` files, this test will fail.
    macrotest::expand_args("tests/expand_args/*.rs", &["--features", "test-feature"]);
}

#[test]
#[should_panic]
pub fn fail_expect_expanded_args() {
    // This directory doesn't have expanded files but since they're expected, the test will fail.
    macrotest::expand_without_refresh_args(
        "tests/no_expanded_args/*.rs",
        &["--features", "test-feature"],
    );
}

// https://github.com/eupn/macrotest/pull/61
#[test]
pub fn pr61() {
    macrotest::expand("tests/pr61/*/*.rs");
}


#[test]
#[should_panic]
pub fn fail_to_expand() {
    // This will fail due to syntax error caused expansion to fail
    // and that failure not being explicitly expected using `_fail` function
    macrotest::expand(
        "tests/expand_fail/*.rs"
    );
}

#[test]
pub fn test_expand_args_fail() {
    // This will fail due to syntax error caused expansion to fail
    macrotest::expand_args_fail(
        "tests/expand_fail/*.rs",
        &["--features", "test-feature"],
    );
}

#[test]
#[should_panic]
pub fn expect_expanded_fail() {
    // This directory doesn't have expanded files but since they're expected, the test will fail.
    macrotest::expand_without_refresh_fail("tests/no_expanded/*.rs");
}
