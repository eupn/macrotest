# `macrotest`

[![Travis-CI](https://api.travis-ci.com/eupn/macrotest.svg?branch=master)](https://travis-ci.com/eupn/macrotest)
[![Crates.io](https://img.shields.io/crates/v/macrotest)](https://crates.io/crates/macrotest)
[![docs.rs](https://docs.rs/macrotest/badge.svg)](https://docs.rs/macrotest/)
[![Crates.io](https://img.shields.io/crates/d/macrotest)](https://crates.io/crates/macrotest)
[![Crates.io](https://img.shields.io/crates/l/macrotest)](https://crates.io/crates/macrotest)

Similar to [trybuild], but allows you to write tests on how macros are expanded.

----

## Documentation

Please refer to the [documentation](https://docs.rs/macrotest).

## Example

Install nightly rust, [`cargo expand`] and [`rustfmt`].

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

Populate the `/tests/expand` directory with rust source files. Each file is a macro expansion test case.

[trybuild]: https://github.com/dtolnay/trybuild
[`cargo expand`]: https://github.com/dtolnay/cargo-expand
[`rustfmt`]: https://github.com/rust-lang/rustfmt

