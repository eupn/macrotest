#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate test_project;
pub fn main() {
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec
    };
}
