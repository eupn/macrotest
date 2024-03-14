#!/usr/bin/env bash
# shellcheck shell=bash
#set -x

# Exit on error
set -e

exec >&2

# Define the source and destination directories
REPO_DIR=$(git rev-parse --show-toplevel)
SRC_DIR="${REPO_DIR}/test-virtual/wrkspc-macro"
DEST_DIR="${REPO_DIR}/test-virtual/wrkspc-test"

exp_file="${DEST_DIR}/${1}"

# Use basename to get the filename from the path
bin_file=$(basename "$exp_file")

# Use cut to remove the .expanded.rs extension
bin_name=$(echo "$bin_file" | cut -f 1 -d '.')

# Get the relative path of the source file
rel_path=$(realpath --relative-to="${DEST_DIR}/src" $exp_file)
src_file="${SRC_DIR}/${rel_path}"

if [ -e "${src_file}" ]; then
  # A source expanded file exists. Build.
  cp --force "${src_file}" "${exp_file}.staged"
  cat "${src_file}" >"$3"
else
    echo "$0: Fatal: don't know how to build '$1'" >&2
    exit 99
fi

cargo build --release --bin "${bin_name}" &>target/wrkspc-test-build.log

if [ $? -eq 0 ]; then
  rm --force "${exp_file}.staged"
fi
