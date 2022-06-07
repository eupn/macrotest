# `macrotest`

[![Travis-CI](https://api.travis-ci.com/eupn/macrotest.svg?branch=master)](https://travis-ci.com/eupn/macrotest)
[![Crates.io](https://img.shields.io/crates/v/macrotest)](https://crates.io/crates/macrotest)
![MSRV 1.34.0](https://img.shields.io/badge/MSRV-1.34.0-orange.svg)
[![docs.rs](https://docs.rs/macrotest/badge.svg)](https://docs.rs/macrotest/)
[![Crates.io](https://img.shields.io/crates/d/macrotest)](https://crates.io/crates/macrotest)
[![Crates.io](https://img.shields.io/crates/l/macrotest)](https://crates.io/crates/macrotest)

Similar to [trybuild], but allows you to test how declarative or procedural macros are expanded.

*Minimal Supported Rust Version: 1.34.0*

----

## Documentation

Please refer to the [documentation](https://docs.rs/macrotest).

## Example

Install nightly rust and [`cargo expand`].

Add to your crate's Cargo.toml:

```toml
[dev-dependencies]
macrotest = "1"
```

Under your crate's `tests/` directory, create `tests.rs` file containing the following code:

```rust
#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}
```

Populate the `tests/expand/` directory with rust source files. Each source file is a macro expansion test case.

See [test-project](test-project) and [test-procmacro-project](test-procmacro-project) for the reference.

[trybuild]: https://github.com/dtolnay/trybuild
[`cargo expand`]: https://github.com/dtolnay/cargo-expand
