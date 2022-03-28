# `test-project`

This is an example virtual-workspace that contains crates:

- `wrkspc`
- `wrkspc-dev`
- `wrkspc-macro`
- `wrkspc-test`

Where each crate is:

- `wrkspc`: A "Hello world" style library (for release as a crate).
- `wrkspc-dev`: A "Hello world" style library for development (not for release).
- `wrkspc-macro`: The `test-promacro-project` adjusted to fit into the workspace plugin-test harness.
- `wrkspc-test`: The `test-project` adjusted to fit into the workspace plugin-test harness.

Test harness for integration tests as plugins, is per the [Infinyon/Fluvio](https://www.infinyon.com/blog/2021/04/rust-custom-test-harness/) setup.
