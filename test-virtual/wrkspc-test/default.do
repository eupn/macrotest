#!/usr/bin/env bash

exec >&2

MACRO_DIR="test-virtual/wrkspc-macro"

# Define the source and destination directories
REPO_DIR=$(git rev-parse --show-toplevel)
SRC_DIR="${REPO_DIR}/${MACRO_DIR}"
DEST_DIR="${REPO_DIR}/zno-test"

exp_file="${DEST_DIR}/$1"
src_file="${SRC_DIR}/$(realpath --relative-to="${DEST_DIR}/src" "$exp_file")"

# Get the filename without the extension
bin_name=$(basename "$exp_file" | cut -f 1 -d '.')

# redo when the expanded file changes
redo-ifchange "${src_file}"

# Check if source file exists and copy it to the destination
if [ -e "${src_file}" ]; then
  cp --force "${src_file}" "${exp_file}.staged"
  cat "${src_file}" >"$3"
else
  echo "$0: Fatal: don't know how to build '$1'" >&2
  exit 99
fi

# Build and log
if cargo build --release --bin "${bin_name}" > "target/zno-test-build-${bin_name}.log" 2>&1; then
  rm --force "${exp_file}.staged"
fi