#!/usr/bin/env bash
#
# $1 is the target name, eg. pie.init
# $2 is the same as $1.
# $3 is the temporary output file to create.
#    If a .do script succeeds, redo replaces $1 with $3.
#    If a .do script fails, redo leaves $1 alone.

# shellcheck shell=bash
#set -x

# Exit on error
set -e

exec >&2

error_count=0
error_messages=""

REPO_DIR=$(git rev-parse --show-toplevel)
DEST_DIR="src/tests"

cargo test --manifest-path ./../wrkspc-macro/Cargo.toml &>target/wrkspc-test-test.log

# Find files with the extension '.do' within a maximum depth of 2 directories.
# Remove the '.do' extension from the file names.
# Exclude the current script file.
# Finally, invoke the 'redo-ifchange' command for each file.
SELF=$(basename "${0##*/}" .do)
find . -maxdepth 2 -type f -name '*.do' -print0 | \
  xargs -0 echo | \
  sed -e 's/\.do//g' -e "s/\.\/$SELF//g" | \
  sed -e 's/\.do//g' -e "s/\.\/default//g" | \
  xargs redo-ifchange

while IFS= read -r -d '' file; do

    source_file="${file%.rs}.expanded.rs"

    # Check if there is a corresponding .expanded.rs file
    if [ ! -f "$source_file" ]; then
        continue
    fi

    # Get the relative path of the source file
    rel_path=$(realpath --relative-to=${REPO_DIR}/test-virtual/wrkspc-macro/tests $file)

    expanded_file="${rel_path%.rs}.expanded.rs"
    expanded_path="${DEST_DIR}/${expanded_file}"

    redo-ifchange "${source_file}"

    # If the source file has changed, redo the expanded file
    if [ $? -eq 0 ]; then
      redo "${expanded_path}"
      if [ $? -eq 1 ]; then
        error_count=$((error_count+1))
        error_messages="${error_messages}Error in file: ${source_file}\n"
      fi
    fi

# Find all .rs files in the specified directory not ending in .expanded.rs.
# The -print0 option with find and -d '' option with read are used together
# to handle file names that contain spaces, newlines or other special characters.
# The ! -name "*.expanded.rs" is excluding any .rs files that end with .expanded.rs.
# This is important because we only want to process original .rs files, not
# ones that have been expanded.
#
# Use process substitution (`<(command)`) instead of a pipe.
# This will allow the `while` loop to run in the current shell and preserve
# changes to the variables `${error_count}` and `${error_messages}` .
done < <(find "${REPO_DIR}/test-virtual/wrkspc-macro/tests" -type f -name "*.rs" ! -name "*.expanded.rs" -print0)

# Print all error messages
if [ $error_count -gt 0 ]; then
    echo "Errors occurred in the incoming files:\n${error_messages}" >&2
    exit 1
fi
