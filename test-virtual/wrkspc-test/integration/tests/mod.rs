pub mod basic;
pub mod feature;

#[derive(Debug)]
pub struct IntegrationTest {
    pub name: &'static str,
    pub test_fn: fn(),
}

inventory::collect!(IntegrationTest);
