#![crate_type = "lib"]
#![doc(html_root_url = "https://docs.rs/macrotest/1.0.0")]

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
//!     macrotest::expand("tests/expand/*.rs");
//! }
//! ```
//!
//! The test can be run with `cargo test`. This test will invoke the [`cargo expand`] command
//! on each of the source files that matches the glob pattern and will compare the expansion result
//! with the corresponding `*.expanded.rs` file.
//!
//! If a `*.expanded.rs` file doesn't exists, it will be created (this is how you update your tests).
//!
//! Possible test outcomes are:
//! - **Pass**: expansion succeeded and the result is the same as in the `.expanded.rs` file
//! - **Fail**: expansion was different from the `.expanded.rs` file content
//! - **Refresh**: `.expanded.rs` didn't exist and has been created
//!
//! # Workflow
//!
//! First of all, the [`cargo expand`] tool must be present. You can install it via cargo:
//!
//! ```bash
//! cargo install cargo-expand
//! ```
//!
//! A **nightly** compiler is required for this tool to work, so it must be installed as well.
//!
//! `cargo-expand` uses [`rustfmt`](https://github.com/rust-lang/rustfmt) to format expanded code.
//! It's highly recommended to install it since the examples in the `test-project/` and
//! `test-procmacro-project/` folders are using a formatted version of the expanded code
//! to compare with.
//!
//! ## Setting up a test project
//!
//! In your crate that provides procedural or declarative macros, under the `tests` directory,
//! create an `expand` directory and populate it with different expansion test cases as
//! rust source files.
//!
//! Then create a `tests.rs` file that will run the tests:
//!
//! ```rust
//! #[test]
//! pub fn pass() {
//!     macrotest::expand("tests/expand/*.rs");
//! }
//! ```
//!
//! And then you can run `cargo test`, which will
//!
//! 1. On the first run, generate the `*.expanded.rs` files for each of the test cases under
//! the `expand` directory
//! 1. On subsequent runs, compare test cases' expansion result with the
//! content of the respective `*.expanded.rs` files
//!
//! ## Updating `.expanded.rs`
//!
//! Just remove the `*.expanded.rs` files and re-run the corresponding tests. Files will be created
//! automatically; hand-writing them is not recommended.
//!
//! [trybuild]: https://github.com/dtolnay/trybuild
//! [`cargo expand`]: https://github.com/dtolnay/cargo-expand

#[macro_use]
mod path;

mod cargo;
mod dependencies;
mod error;
mod expand;
mod features;
mod manifest;
mod message;
mod rustflags;

pub use expand::expand;
pub use expand::expand_without_refresh;
