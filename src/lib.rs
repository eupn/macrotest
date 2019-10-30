//! #### &emsp; Test harness for macro expansion.
//!
//! Similar to [trybuild], but allows you to write tests on how macros are expanded.
//!
//! <br>
//!
//! # Macro expansion tests
//!
//! A minimal `macrotest` setup looks like this:
//!
//! ```rust
//! #[test]
//! pub fn pass() {
//!     let t = macrotest::TestCases::new();
//!     t.pass("tests/expand/*.rs");
//! }
//! ```
//!
//! The test can be run with `cargo test`. This test will invoke `cargo expand` command on each of
//! the source files matches the glob pattern and will compare expansion result with
//! corresponding `*.expanded.rs` file.
//!
//! If `*.expanded.rs` file doesn't exists, it will create a new one
//! (this is how you update your tests).
//!
//! Possible test outcomes are:
//! - **Pass**: expansion succeeded and result is the same as in `.expanded.rs` file
//! - **Fail**: expansion is different from the `.expanded.rs` file content. This will print a diff
//! - **Refresh**: `.expanded.rs` didn't exist and has been created
//!
//! # Workflow
//!
//! First of all, the `cargo-expand` tool must be present. You can install it via cargo:
//!
//! ```bash
//! cargo install cargo-expand
//! ```
//!
//! A **nigthly** compiler is required for this tool to operate, so it must be installed as well.
//!
//! `cargo-expand` uses [`rustfmt`](https://github.com/rust-lang/rustfmt) to format expanded code.
//! It's advised to install it, since examples in `test-project/` and `test-procmacro-project/`
//! folders are using formatted version of expanded code to compare with.
//!
//! ## Setting up a test project
//!
//! Inside your crate that provides procedural or declarative macros, create a test case
//! under `tests` directory.
//!
//! Under the `tests` directory create an `expand` directory and populate it with
//! different expansion test cases as Rust source files.
//!
//! Then, udner the `tests` directory, create `tests.rs` file that will run the tests:
//!
//! ```rust
//! #[test]
//! pub fn pass() {
//!     let t = macrotest::TestCases::new();
//!     t.pass("tests/expand/*.rs");
//! }
//! ```
//!
//! And then you can run `cargo test` to
//!
//! 1. For the first time, generate the `.expanded.rs` files for each of the test cases under
//! the `expand` directory
//! 1. After that, test cases' expansion result will be compared with the
//! content of `.expanded.rs` files
//!
//! ## Updating `.expanded.rs`
//!
//! Just remove the `.expanded.rs` files and re-run the corresponding tests. Files will be created
//! automatically; hand-writing them is not recommended.
//!
//! [trybuild]: https://github.com/dtolnay/trybuild

#![crate_type = "lib"]

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::thread;

mod error;
mod expand;
mod message;

#[derive(Debug)]
enum ExpansionOutcome {
    Same,
    Different(Vec<u8>, Vec<u8>),
    New(Vec<u8>),
    ExpandError(Vec<u8>),
}

#[derive(Debug)]
pub struct TestCases {
    inner: RefCell<Expander>,
}

#[derive(Debug)]
struct Expander {
    tests: Vec<Test>,
}

#[derive(Clone, Debug)]
struct Test {
    path: PathBuf,
}

impl TestCases {
    pub fn new() -> Self {
        TestCases {
            inner: RefCell::new(Expander { tests: Vec::new() }),
        }
    }

    pub fn pass<P: AsRef<Path>>(&self, path: P) {
        self.inner.borrow_mut().tests.push(Test {
            path: path.as_ref().to_owned(),
        });
    }
}

#[doc(hidden)]
impl Drop for TestCases {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.inner.borrow_mut().expand();
        }
    }
}
