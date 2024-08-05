#!/usr/bin/env bash

exec >&2

MACRO_DIR="test-virtual/wrkspc-macro"

REPO_DIR=$(git rev-parse --show-toplevel)
DEST_DIR="${REPO_DIR}/test-virtual/wrkspc-test/src/tests"

mkdir -p target

# Find .do files, exclude certain ones, remove .do extension, and call redo-ifchange
find . -maxdepth 6 -type f -name '*.do' | \
  grep -vE 'default\.do|all\.do|default\.expand\.do' | \
  sed -r 's/\.do$//' | \
  xargs redo-ifchange

# Find all .rs files not ending in .expanded.rs
find "${REPO_DIR}/${MACRO_DIR}/tests" -type f -name "*.rs" ! -name "*.expanded.rs" -print0 | while IFS= read -r -d '' file; do
    source_file="${file%.rs}.expanded.rs"

    # Check if there is a corresponding .expanded.rs file
    if [ ! -f "$source_file" ]; then
      continue
    fi

    redo-ifchange "${source_file}"

    test_name=$(basename "${file}" | cut -f 1 -d '.')

    # Invoke virtual target for test_name
    redo-ifchange "${test_name}.expand"

    # Get the relative path of the source file
    rel_path=$(realpath --relative-to="${REPO_DIR}/${MACRO_DIR}/tests" "$file")
    expanded_file="${rel_path%.rs}.expanded.rs"
    expanded_path="${DEST_DIR}/${expanded_file}"

    # Fall back to the default.do script
    if ! redo-ifchange "${expanded_path}"; then
      printf "Error in file: %s\n" "${source_file}" >&2
      exit 1
    fi
done