# `macrotest`

[![Travis-CI](https://api.travis-ci.com/eupn/macrotest.svg?branch=master)](https://travis-ci.com/eupn/macrotest)
[![Crates.io](https://img.shields.io/crates/v/macrotest)](https://crates.io/crates/macrotest)
[![docs.rs](https://docs.rs/macrotest/badge.svg)](https://docs.rs/macrotest/)
[![Crates.io](https://img.shields.io/crates/d/macrotest)](https://crates.io/crates/macrotest)
[![Crates.io](https://img.shields.io/crates/l/macrotest)](https://crates.io/crates/macrotest)

Similar to [trybuild], but allows you to write tests on how macros are expanded.

----

## Macro expansion tests

A minimal `macrotest` setup looks like this:

In project's Cargo.toml:

```toml
[dev-dependencies]
macrotest = "0.1"
```

Under project's `tests/` directory create `tests.rs`:

```rust
#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}
```

The test can be run with `cargo test`. This test will invoke `cargo expand` command on each of
the source files matches the glob pattern and will compare expansion result with
corresponding `*.expanded.rs` file.

If `*.expanded.rs` file doesn't exists, it will create a new one
(this is how you update your tests).

Possible test outcomes are:
- **Pass**: expansion succeeded and a result is the same as in `*.expanded.rs` file
- **Fail**: expansion is different from the `*.expanded.rs` file content. This will print a diff
- **Refresh**: `*.expanded.rs` didn't exist and has been created

*NB*: following code is removed from the expansion result:

<details>

```rust
#![feature(prelude_import)] 
#[prelude_import] 
use std::prelude::v1::*; 
#[macro_use] 
extern crate std;
```

</details>

## Workflow

First of all, a [`cargo-expand`](https://crates.io/crates/cargo-expand) tool must be present. 
You can install it with `cargo`:

```bash
cargo install cargo-expand
```

A **nightly** compiler is required for this tool to operate, so it must be installed as well.

`cargo-expand` uses [`rustfmt`](https://github.com/rust-lang/rustfmt) to format expanded code. 
It's advised to install it, since examples in [test-project](test-project) and
[test-procmacro-project](test-procmacro-project) are using formatted version of expanded code to compare with.

### Setting up a test project

Inside your crate that provides procedural or declarative macros, create a test case
under `tests` directory.

Under the `tests` directory create an `expand` directory and populate it with
different expansion test cases as Rust source files.

Then, under the `tests` directory, create `tests.rs` file that will run the tests:

```rust
#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}
```

And then you can run `cargo test` to

1. For the first time, generate the `*.expanded.rs` files for each of the test cases under
the `expand` directory
1. After that, test cases' expansion result will be compared with the
content of `*.expanded.rs` files

### Updating `*.expanded.rs`

Just remove the `*.expanded.rs` files and re-run the tests. Files will be created
automatically; hand-writing them is not recommended.

[trybuild]: https://github.com/dtolnay/trybuild
