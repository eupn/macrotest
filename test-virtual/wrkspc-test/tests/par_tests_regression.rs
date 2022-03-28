// The tests were interfering with each other when run in parallel.
// This regression test module will ensure that parallel use case is handled.

// Suspend parallel test execution while we implement the integration
// test harness.
//
// #[test]
// pub fn parallel_1() {
//     macrotest::expand("tests/expand/first.rs");
// }

// Suspend parallel test execution while we implement the integration
// test harness.
//
// #[test]
// pub fn parallel_2() {
//     macrotest::expand("tests/expand/second.rs");
// }
