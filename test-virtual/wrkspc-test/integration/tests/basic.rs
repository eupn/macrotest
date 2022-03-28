use super::IntegrationTest;

fn basic_test() {
   println!("Running basic test")
}

inventory::submit!(IntegrationTest {
   name: "basic",
   test_fn: basic_test
});
