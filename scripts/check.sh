#!/bin/sh
# Quality gate: formatting, lints and tests must all pass.
set -eu
cd "$(dirname "$0")/.."

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
