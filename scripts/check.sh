#!/bin/sh
# Checks: formatting, lints and tests must all pass.
set -eu
cd "$(dirname "$0")/.."

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings

# The other supported platform's code (ADR 0020), type-checked without
# linking when that target's standard library is installed (AGENTS.md).
case "$(rustc -vV | sed -n 's/^host: //p')" in
  *-apple-darwin) other=x86_64-unknown-linux-gnu ;;
  *) other=aarch64-apple-darwin ;;
esac
if [ -d "$(rustc --print target-libdir --target "$other")" ]; then
  cargo clippy --workspace --all-targets --target "$other" -- -D warnings
else
  echo "skipped: clippy for $other, whose standard library is not installed (AGENTS.md, \"Platforms\")"
fi

cargo test --workspace
