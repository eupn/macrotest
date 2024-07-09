#![crate_type = "lib"]
#![doc(html_root_url = "https://docs.rs/macrotest/1.0.13")]

//! #### &emsp; Test harness for macro expansion.
//!
//! Similar to [trybuild], but allows you to write tests on how macros are expanded.
//!
//! *Minimal Supported Rust Version: 1.56*
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
//!     // Alternatively,
//!     macrotest::expand_without_refresh("tests/expand/*.rs");
//! }
//! ```
//!
//! The test can be run with `cargo test`. This test will invoke the [`cargo expand`] command
//! on each of the source files that matches the glob pattern and will compare the expansion result
//! with the corresponding `*.expanded.rs` file.
//!
//! If a `*.expanded.rs` file doesn't exists and it's not explicitly expected to (see [`expand_without_refresh`]),
//! it will be created (this is how you update your tests).
//!
//! Possible test outcomes are:
//! - **Pass**: expansion succeeded and the result is the same as in the `.expanded.rs` file
//! - **Fail**: expansion was different from the `.expanded.rs` file content
//! - **Refresh**: `.expanded.rs` didn't exist and has been created
//! - **Refresh-fail**: `.expanded.rs` is expected to be present, but not exists. See [`expand_without_refresh`].
//!
//! *Note:* when working with multiple expansion test files, it is recommended to
//! specify wildcard (*.rs) instead of doing a multiple calls to `expand` functions for individual files.
//! Usage of wildcards for multiple files will group them under a single temporary crate for which
//! dependencies will be built a single time. In contrast, calling `expand` functions for each
//! source file will create multiple temporary crates and that will reduce performance as depdendencies
//! will be build for each of the temporary crates.
//!
//! ## Passing additional arguments to `cargo expand`
//!
//! It's possible to specify additional arguments for [`cargo expand`] command.
//!
//! In order to do so, use the following functions with `_args` suffix:
//! - [`expand_args`]
//! - [`expand_without_refresh_args`]
//!
//! Example:
//!
//! ```rust
//! pub fn pass() {
//!     macrotest::expand_args("tests/expand/*.rs", &["--features", "my-feature"]);
//!     // Or
//!     macrotest::expand_without_refresh_args("tests/expand/*.rs", &["--features", "my-feature"]);
//! }
//! ```
//!
//! The `_args` functions will result in the following [`cargo expand`] command being run:
//!
//! ```bash
//! cargo expand --bin <test-name> --theme none --features my-feature
//! ```
//!
//! # Workflow
//!
//! First of all, the [`cargo expand`] tool must be present. You can install it via cargo:
//!
//! ```bash
//! cargo install --locked cargo-expand
//! ```
//!
//! (In CI, you'll want to pin to a particular version,
//! since
//! [cargo expand's output is not stable across versions](https://github.com/dtolnay/cargo-expand/issues/179).
//! Look up the
//! [current version](https://crates.io/crates/cargo-expand)
//! and do something like `cargo install --locked --version 1.0.81 cargo-expand`.)
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
//!     // Or:
//!     macrotest::expand_without_refresh("tests/expand/*.rs");
//! }
//! ```
//!
//! And then you can run `cargo test`, which will
//!
//! 1. Expand macros in source files that match glob pattern
//! 1. In case if [`expand`] function is used:
//!     - On the first run, generate the `*.expanded.rs` files for each of the test cases under
//!     the `expand` directory
//!     - On subsequent runs, compare test cases' expansion result with the
//!     content of the respective `*.expanded.rs` files
//! 1. In case if [`expand_without_refresh`] is used:
//!     - On each run, it will compare test cases' expansion result with the content of the
//!     respective `*.expanded.rs` files.
//!     - If one or more `*.expanded.rs` files is not found, the test will fail.
//!
//! ## Updating `.expanded.rs`
//!
//! This applicable only to tests that are using [`expand`] or [`expand_args`] function.
//!
//! Run tests with the environment variable `MACROTEST=overwrite` or remove the `*.expanded.rs`
//! files and re-run the corresponding tests. Files will be created automatically; hand-writing
//! them is not recommended.
//!
//! [`expand_without_refresh`]: expand/fn.expand_without_refresh.html
//! [`expand_without_refresh_args`]: expand/fn.expand_without_refresh_args.html
//! [`expand`]: expand/fn.expand.html
//! [`expand_args`]: expand/fn.expand_args.html
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
pub use expand::expand_args;
pub use expand::expand_without_refresh;
pub use expand::expand_without_refresh_args;
