use crate::tests::IntegrationTest;

fn feature_test() {
    println!("Running feature test")
}

inventory::submit!(IntegrationTest {
    name: "feature",
    test_fn: feature_test
});

pub fn pass_args() {
    macrotest::expand_args(
        "integration/tests/feature/expanded/*.rs",
        &["--features", "test-feature"],
    );
}
inventory::submit!(IntegrationTest {
    name: "feature",
    test_fn: pass_args
});

pub fn pass_expect_expanded_args() {
    // If you delete one of the `.expanded.rs` files, this test will fail.
    macrotest::expand_args(
        "integration/tests/feature/expanded/*.rs",
        &["--features", "test-feature"],
    );
}
inventory::submit!(IntegrationTest {
    name: "feature",
    test_fn: feature_test
});

// Suspended panicky tests for the moment
//
// pub fn fail_expect_expanded_args() {
//     // This directory doesn't have expanded files but since they're expected, the test will fail.
//     macrotest::expand_without_refresh_args(
//         "integration/tests/feature/unchanged/*.rs",
//         &["--features", "test-feature"],
//     );
// }
// inventory::submit!(IntegrationTest {
//     name: "feature",
//     test_fn: fail_expect_expanded_args
// });
