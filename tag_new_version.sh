#!/bin/bash

set -e
set -u
set -o pipefail

cargo clippy -- -D warnings
cargo +1.56.0 test
cargo +1.56.0 test --all-features
git diff --exit-code  # check if unstaged changes exist
git diff --cached --exit-code  # check if staged, uncommitted changes exist
exec cargo workspaces version --all --allow-branch main --exact --no-individual-tags --force \*
