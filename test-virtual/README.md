# `test-virtual`

A convention over configuration template for integration tests of proc-macros,
in a workspace.

This example workspace contains crates:

- `wrkspc`: A library crate.
- `wrkspc-dev`: A development crate.
- `wrkspc-macro`: The Proc-macro crate.
- `wrkspc-test`: Integration tests, and DJB's `redo` build system for `wrkspc-macro`.

Where:

- `wrkspc`: A "Hello world" style library (for release as a crate).
- `wrkspc-dev`: A "Hello world" style library for development (not for release).
- `wrkspc-macro`: The `test-promacro-project` adjusted to fit into the workspace plugin-test harness. This crate provides a declarative `test_vec![]` macro
to test its expansion under the [tests](tests) directory with [`macrostest`](https://crates.io/crates/macrotest).
- `wrkspc-test`: The `test-project` adjusted to fit into the workspace integrated test harness.

The integration test harness, `wkspc-test`, uses [DJB's redo build system](https://cr.yp.to/redo.html), as implemented by [apenwarr](https://github.com/apenwarr/redo/) and as ported to Rust by [zombiezen](https://github.com/zombiezen/redo-rs).

In this build process `all.do` flows to a generic build step `default.do`. Hence, `all.do` is responsible for iterating over all source files and `default.do` is responsible for processing each individual file. The `default.do` is kept generic by adhering to some naming conventions, shared between the macro and test crates:

In this setup:

- `all.do` is the starting point of the build process.
- `default.do` is called by `all.do` for each source file.
- `default.do` processes each file by:
  - Extracting the filename from the path using `basename`, resulting in `bin`.
  - Removing the `.expanded.rs` extension using `cut`, resulting in `bin`.

The next diagram shows the flow of data in the files between the folders `wkspc-macro` and `wkspc-test`:

- `wkspc-macro` is the source directory containing the `.rs` to be tested files.
- `all.do` processes each `.rs` file in `wkspc-macro`.
- `default.expand.do` is called by `all.do` for each `.rs` file and generates a corresponding `.expanded.rs` file if the source file has changed.
- The `.expanded.rs` files are placed in the `wkspc-test` directory.
- If redoing the expanded file fails, it records the error message.
- If the build succeeds, it removes `exp_file.staged`.

[Build data flow](./images/figure-data.png)
<!--
flowchart LR
    A[wkspc-macro] -->|Source .rs files| B[all.do]
    B -->|Processes each file| C[default.expand.do]
    C -->|Generates .expanded.rs files if source file has changed| D[wkspc-test]
    D -->|If redo fails| E[Records error message]
    E --> F[error_messages]
    D -->|If build succeeds| G[Removes exp_file.staged]
-->

Here's a Mermaid diagram that shows how the build process in `all.do` flows to `default.expand.do` and then to `default.do`. In this first diagram:

- `all.do` is the starting point of the build process.
- `default.expand.do` is called by `all.do` for each source file.
- `default.expand.do` processes each file by:
  - Extracting the filename from the path using `basename`, resulting in `bin_file`.
  - Removing the `.expanded.rs` extension using `cut`, resulting in `bin_name`.
  - Checking if the .expanded.rs file exists. If it does, it continues processing. If it doesn't, it skips the file.
  - Getting the relative path of the source file using `realpath`, resulting in `rel_path`.
  - Constructing the path of the expanded file using string concatenation, resulting in `expanded_path`.
  - Checking if the source file has changed. If it has, it redoes the expanded file. If it hasn't, it skips the file.
  - If redoing the expanded file fails, it increments the error count and records the error message.
- `default.do` is called for each file that is not skipped.
  - It copies the source file to the destination, resulting in `exp_file.staged`.
  - It builds the file using `cargo build`. If the build succeeds, it removes `exp_file.staged`.

[Build logic flow](./images/figure-logic.png)
<!--
workflow TD
    A[all.do] -->|Iterates over all source files| B[default.expand.do]
    B --> C{Processes each file}
    C -->|1. Extracts filename from path| D[Uses basename]
    D --> E[bin_file]
    C -->|2. Removes .expanded.rs extension| F[Uses cut]
    F --> G[bin_name]
    C -->|3. Checks if .expanded.rs file exists| H[If file exists]
    H -->|Yes| I[Continues processing]
    I -->|4. Gets relative path of source file| J[Uses realpath]
    J --> K[rel_path]
    I -->|5. Constructs path of expanded file| L[Uses string concatenation]
    L --> M[expanded_path]
    I -->|6. Checks if source file has changed| N[If file has changed]
    N -->|Yes| O[Redoes expanded file]
    O -->|If redo fails| P[Increments error count and records error message]
    P --> Q[error_count and error_messages]
    N -->|No| R[Skips file]
    R --> S[default.do]
    S -->|7. Copies source file to destination| T[Uses cp]
    T --> U[exp_file.staged]
    S -->|8. Builds with cargo| V[Uses cargo build]
    V -->|If build succeeds| W[Removes exp_file.staged]
-->

## Containers

To run Testcontainers-based tests,
you need a Docker-API compatible container runtime,
such as using [Testcontainers Cloud](https://www.testcontainers.cloud/) or installing [Podman](https://podman.io/)  or Docker locally.  [Testcontainers Desktop](https://testcontainers.com/desktop/) takes care of most of the manual configuration for alternative runtimes. See [Customizing Docker host detection](#customizing-docker-host-detection) for general configuration mechanisms.

### Podman

In order to run testcontainers against [podman](https://podman.io/) the env vars bellow should be set

MacOS:

```bash
{% raw %}
export DOCKER_HOST=unix://$(podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}')
export TESTCONTAINERS_DOCKER_SOCKET_OVERRIDE=/var/run/docker.sock
{% endraw %}
```

Linux:

```bash
export DOCKER_HOST=unix://${XDG_RUNTIME_DIR}/podman/podman.sock
```

If you're running Podman in rootless mode, ensure to include the following line to disable Ryuk:

```bash
export TESTCONTAINERS_RYUK_DISABLED=true
```

!!! note
    Previous to version 1.19.0, `export TESTCONTAINERS_RYUK_PRIVILEGED=true`
    was required for rootful mode. Starting with 1.19.0, this is no longer required.

### Container environment discovery

Testcontainers will try to connect to a Docker daemon using the following strategies in order:

- Environment variables:
  - `DOCKER_HOST`
  - `DOCKER_TLS_VERIFY`
  - `DOCKER_CERT_PATH`
- Defaults:
  - `DOCKER_HOST=https://localhost:2376`
  - `DOCKER_TLS_VERIFY=1`
  - `DOCKER_CERT_PATH=~/.docker`
- If Docker Machine is installed, the docker machine environment for the *first* machine found. Docker Machine needs to be on the PATH for this to succeed.
- If you're going to run your tests inside a container, please read [Patterns for running tests inside a docker container](continuous_integration/dind_patterns.md) first.

### Docker registry authentication

Testcontainers will try to authenticate to registries with supplied config using the following strategies in order:

- Environment variables:
  - `DOCKER_AUTH_CONFIG`
- Docker config
  - At location specified in `DOCKER_CONFIG` or at `{HOME}/.docker/config.json`

### Customizing Docker host detection
