# `macrotest`

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
    let t = macrotest::TestCases::new();
    t.pass("tests/expand/*.rs");
}
```

The test can be run with `cargo test`. It will individually extract each of
the source files matches the glob pattern as `main.rs` in a separate `cargo` crate in
temporary folder and will invoke `cargo expand` to expand macro invocations.

Project's crate will be listed under `[dependencies]` section of temporary crates and will be available
from the test cases.

Expansion result is compared with the corresponding `.expanded.rs` file (same file name as
the test except with a different extension). If file doesn't exists, it will create a new one
(this is how you update your tests).

Possible test outcomes are:
- **Pass**: expansion succeeded and a result is the same as in `.expanded.rs` file
- **Fail**: expansion is different from the `.expanded.rs` file content. This will print a diff
- **Refresh**: `.expanded.rs` didn't exist and has been created

*NB*: after execution of each test, a temporary folder with the crate is removed automatically.

## Workflow

First of all, a [`cargo-expand`](https://crates.io/crates/cargo-expand) tool must be present. 
You can install it with `cargo`:

```bash
cargo install cargo-expand
```

A **nightly** compiler is required for this tool to operate, so it must be installed as well.

### Setting up a test project

Inside your crate that provides procedural or declarative macros, create a test case
under `tests` directory.

Under the `tests` directory create an `expand` directory and populate it with
different expansion test cases as Rust source files.

Then, under the `tests` directory, create `tests.rs` file that will run the tests:

```rust
#[test]
pub fn pass() {
    let t = macrotest::TestCases::new();
    t.pass("tests/expand/*.rs");
}
```

And then you can run `cargo test` to

1. For the first time, generate the `.expanded.rs` files for each of the test cases under
the `expand` directory
1. After that, test cases' expansion result will be compared with the
content of `.expanded.rs` files

### Updating `.expanded.rs`

Just remove the `.expanded.rs` files and re-run the corresponding tests. Files will be created
automatically; hand-writing them is not recommended.

[trybuild]: https://github.com/dtolnay/trybuild
