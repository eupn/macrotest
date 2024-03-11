// #![cfg(feature = "test-feature")]

#[macro_use]
extern crate wrkspc_macro;

pub fn main() {
    #[my_attribute]
    struct Test;
}
