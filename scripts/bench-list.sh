#!/usr/bin/env bash
# Quick sequential vs parallel listing smoke timing (no criterion required).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

DIR="$(mktemp -d "${TMPDIR:-/tmp}/f00-bench-XXXXXX")"
trap 'rm -rf "$DIR"' EXIT

N="${1:-1000}"
echo "Creating $N files in $DIR ..."
for i in $(seq 1 "$N"); do
  printf 'x' >"$DIR/file_$(printf '%05d' "$i").txt"
done

echo "Building f00 (release)..."
cargo build -q -p f00-cli --release

BIN="$ROOT/target/release/f00"

echo
echo "=== sequential (--threads 1) ==="
/usr/bin/time -f 'wall_sec=%e' "$BIN" --threads 1 --profile -1 "$DIR" >/dev/null

echo
echo "=== parallel auto (--threads 0) ==="
/usr/bin/time -f 'wall_sec=%e' "$BIN" --threads 0 --profile -1 "$DIR" >/dev/null

echo
echo "=== parallel 4 threads ==="
/usr/bin/time -f 'wall_sec=%e' "$BIN" --threads 4 --profile -1 "$DIR" >/dev/null

echo
echo "Done. For Criterion:"
echo "  cargo bench -p f00-core --bench list_bench"
