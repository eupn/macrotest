#[macro_use]
extern crate test_procmacro_project;

pub fn main() {
    my_macro_panics! { struct Test; }
}
