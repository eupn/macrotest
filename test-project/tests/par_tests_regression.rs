// The tests were interfering with each other when run in parallel.
// This regression test module will ensure that parallel use case is handled.

#[test]
pub fn parallel_1() {
    macrotest::expand("tests/expand/first.rs");
}

#[test]
pub fn parallel_2() {
    macrotest::expand("tests/expand/second.rs");
}
