# `macrotest`

[![Github Actions](https://img.shields.io/github/actions/workflow/status/eupn/macrotest/ci.yml?branch=master)](https://github.com/eupn/macrotest/actions)
[![Crates.io](https://img.shields.io/crates/v/macrotest)](https://crates.io/crates/macrotest)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/macrotest)](https://crates.io/crates/macrotest)
[![docs.rs](https://docs.rs/macrotest/badge.svg)](https://docs.rs/macrotest/)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/macrotest)](https://crates.io/crates/macrotest)
[![Crates.io (License)](https://img.shields.io/crates/l/macrotest)](https://crates.io/crates/macrotest)

Similar to [trybuild], but allows you to test how declarative or procedural macros are expanded.

*Minimal Supported Rust Version: 1.66*

----

## Documentation

Please refer to the [documentation](https://docs.rs/macrotest).

## Example

Install [`cargo expand`].

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
