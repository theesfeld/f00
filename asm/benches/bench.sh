#!/usr/bin/env bash
# Quick timing: f00-* vs system coreutils for ls, wc, sha256sum.
# Uses /usr/bin for the system tools (avoids f00 supersede links on PATH).
#
# Usage:
#   cd asm && make && ./benches/bench.sh
#   ITERS=50 ./benches/bench.sh
#   DIR=/usr/bin FILE=/usr/bin/ls ./benches/bench.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

ITERS="${ITERS:-30}"
DIR="${DIR:-/usr/bin}"
FILE="${FILE:-/usr/bin/ls}"

if [[ ! -x ./f00-ls || ! -x ./f00-wc || ! -x ./f00-sha256sum ]]; then
  echo "missing f00-* links; run: make" >&2
  exit 1
fi

SYS_LS="${SYS_LS:-/usr/bin/ls}"
SYS_WC="${SYS_WC:-/usr/bin/wc}"
SYS_SHA="${SYS_SHA:-/usr/bin/sha256sum}"

for c in "$SYS_LS" "$SYS_WC" "$SYS_SHA"; do
  if [[ ! -x "$c" ]]; then
    echo "missing system tool: $c" >&2
    exit 1
  fi
done

# Prefer TIMEFORMAT from bash built-in time
TIMEFORMAT='%R'

run_avg() {
  # $1 = label, rest = command
  local label="$1"
  shift
  local i sum=0 t
  # warm
  "$@" >/dev/null 2>&1 || true
  for ((i = 0; i < ITERS; i++)); do
    t="$( { time "$@" >/dev/null 2>&1; } 2>&1 )"
    # t is wall seconds as float
    sum="$(awk -v s="$sum" -v t="$t" 'BEGIN { printf "%.6f", s + t }')"
  done
  local avg
  avg="$(awk -v s="$sum" -v n="$ITERS" 'BEGIN { printf "%.4f", s / n }')"
  printf '%-28s  %s s (avg of %d)\n' "$label" "$avg" "$ITERS"
}

echo "f00 bench · iters=${ITERS} · dir=${DIR} · file=${FILE}"
echo "system: ls=${SYS_LS} wc=${SYS_WC} sha256sum=${SYS_SHA}"
echo

echo "== ls -1 (names only) =="
run_avg "coreutils ls -1" "$SYS_LS" -1 "$DIR"
run_avg "f00-ls -1"       ./f00-ls -1 "$DIR"
run_avg "f00-ls --core -1" ./f00-ls --core -1 "$DIR"
echo

echo "== ls -la =="
run_avg "coreutils ls -la" "$SYS_LS" -la "$DIR"
run_avg "f00-ls -la"       ./f00-ls -la "$DIR"
run_avg "f00-ls --core -la" ./f00-ls --core -la "$DIR"
echo

echo "== wc -l / wc -c =="
run_avg "coreutils wc -l" "$SYS_WC" -l "$FILE"
run_avg "f00-wc -l"       ./f00-wc -l "$FILE"
run_avg "coreutils wc -c" "$SYS_WC" -c "$FILE"
run_avg "f00-wc -c"       ./f00-wc -c "$FILE"
echo

# Prefer a larger file if present for hash timing
HASH_FILE="${HASH_FILE:-}"
if [[ -z "$HASH_FILE" ]]; then
  if [[ -r /usr/bin/f00 ]]; then
    HASH_FILE=/usr/bin/f00
  elif [[ -r ./f00 ]]; then
    HASH_FILE=./f00
  else
    HASH_FILE="$FILE"
  fi
fi

echo "== sha256sum (${HASH_FILE}) =="
run_avg "coreutils sha256sum" "$SYS_SHA" "$HASH_FILE"
run_avg "f00-sha256sum"       ./f00-sha256sum "$HASH_FILE"
run_avg "f00-sha256sum --core" ./f00-sha256sum --core "$HASH_FILE"
echo

echo "done. For hyperfine: hyperfine -w 5 -r 50 '/usr/bin/ls -1 ${DIR}' './f00-ls -1 ${DIR}'"
