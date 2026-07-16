#!/usr/bin/env bash
# Regenerate the committed man page from the current CLI surface.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
cargo build -q -p f00-cli
mkdir -p man
./target/debug/f00 --generate-man > man/f00.1
echo "wrote man/f00.1"
