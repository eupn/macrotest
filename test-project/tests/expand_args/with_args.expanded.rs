#![cfg(feature = "test-feature")]
#[macro_use]
extern crate test_project;
pub fn main() {
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };
}

