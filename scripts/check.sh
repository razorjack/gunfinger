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

# The listening page's JavaScript tests (ADR 0016), with Node's own test
# runner and no packages, when Node 22.7 or newer is installed (it loads the
# page's modules without a package.json). The page itself needs no Node.
if node -e 'const [major, minor] = process.versions.node.split(".").map(Number); process.exit(major > 22 || (major === 22 && minor >= 7) ? 0 : 1)' 2>/dev/null; then
  node --test crates/gunfinger-eval/src/audition/web/tests/*.test.mjs
else
  echo "skipped: the listening page's JavaScript tests, which need Node 22.7 or newer (AGENTS.md, \"The listening tool\")"
fi

# The analysis scripts' tests (`scripts/analysis/test_*.py`), with Python's
# own unittest and no packages, when Python 3 is installed.
if command -v python3 >/dev/null 2>&1; then
  python3 -B -m unittest discover -s scripts/analysis -p 'test_*.py'
else
  echo "skipped: the analysis scripts' tests, which need Python 3"
fi
