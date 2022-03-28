#![cfg(feature = "test-feature")]

#[macro_use]
extern crate test_project;

pub fn main() {
    test_feature_vec![1, 2, 3];
}
