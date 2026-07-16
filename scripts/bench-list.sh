#!/usr/bin/env bash
# Quick sequential vs parallel listing smoke timing (no criterion required).
# For ls/eza/f00 comparison see: ./scripts/bench-compare.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

TIME_BIN=""
for t in /run/current-system/sw/bin/time /usr/bin/time /bin/time; do
  if [[ -x "$t" ]] && "$t" --version 2>&1 | head -1 | grep -qi 'GNU'; then
    TIME_BIN="$t"
    break
  fi
done
if [[ -z "$TIME_BIN" ]] && command -v gtime >/dev/null 2>&1; then
  TIME_BIN="$(command -v gtime)"
fi
time_wall() {
  if [[ -n "$TIME_BIN" ]]; then
    "$TIME_BIN" -f 'wall_sec=%e user=%U sys=%S' "$@"
  else
    "$@"
  fi
}

DIR="$(mktemp -d "${TMPDIR:-/tmp}/f00-bench-XXXXXX")"
trap 'rm -rf "$DIR"' EXIT

N="${1:-1000}"
echo "Creating $N files in $DIR ..."
for i in $(seq 1 "$N"); do
  printf 'x' >"$DIR/file_$(printf '%05d' "$i").txt"
done

echo "Building f00 (release)..."
cargo build -q -p f00 --release

BIN="$ROOT/target/release/f00"

echo
echo "=== sequential (--threads 1) ==="
time_wall "$BIN" --threads 1 --profile -1 "$DIR" >/dev/null

echo
echo "=== parallel auto (--threads 0) ==="
time_wall "$BIN" --threads 0 --profile -1 "$DIR" >/dev/null

echo
echo "=== parallel 4 threads ==="
time_wall "$BIN" --threads 4 --profile -1 "$DIR" >/dev/null

echo
echo "Done. Comparative (ls vs eza vs f00):"
echo "  ./scripts/bench-compare.sh"
echo "Criterion:"
echo "  cargo bench -p f00-core --bench list_bench"
