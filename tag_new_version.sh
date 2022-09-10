#!/bin/bash

set -e
set -u
set -o pipefail

cargo clippy -- -D warnings
cargo test
cargo test --all-features
git diff --exit-code  # check if unstaged changes exist
git diff --cached --exit-code  # check if staged, uncommitted changes exist
exec cargo workspaces version --all --allow-branch main --exact --no-individual-tags --force \*
