pub mod tests;

use tests::IntegrationTest;

fn setup() {
    println!("Setup")
}

fn teardown() {
    println!("Teardown")
}
// NOTE:
// When this code is in src/main.rs, it is executed by `cargo test -- --list`.
// In such cases you could guard it:
// #[cfg(feature = "integration")]
fn main() {
    // Setup test environment
    setup();

    // Run the tests
    for t in inventory::iter::<IntegrationTest> {
        (t.test_fn)()
    }

    // Teardown test environment
    teardown();
}
