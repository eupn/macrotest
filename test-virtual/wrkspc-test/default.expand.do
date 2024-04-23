#!/usr/bin/env bash

exec >&2

MACRO_DIR="test-virtual/wrkspc-macro"

REPO_DIR=$(git rev-parse --show-toplevel)
SRC_DIR="${REPO_DIR}/${MACRO_DIR}"

test_name="${1%.*}"

# Run the test and log the output
cargo test --test "${test_name}" --manifest-path ${SRC_DIR}/Cargo.toml > "target/zno-test-expansion-${test_name}.log" 2>&1

# If the newly produced file disappears, we need to redo.
redo-ifchange "${SRC_DIR}/tests/expand/${test_name}.rs"
