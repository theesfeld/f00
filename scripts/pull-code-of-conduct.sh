#!/usr/bin/env bash
# Pull the org Code of Conduct into one or more project roots.
# Canonical: https://f00.sh/CODE_OF_CONDUCT.md  (hub site/CODE_OF_CONDUCT.md)
set -euo pipefail

CANONICAL_URL="${F00_COC_URL:-https://f00.sh/CODE_OF_CONDUCT.md}"
HUB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOCAL_CANON="${HUB_ROOT}/site/CODE_OF_CONDUCT.md"

usage() {
  cat <<'EOF'
Usage: pull-code-of-conduct.sh [DIR ...]

Copies the f00 Code of Conduct into each DIR (default: current directory).
Prefers the local hub file when run from a clone of f00-sh/f00; otherwise
fetches the live canonical URL.

  F00_COC_URL   override fetch URL (default https://f00.sh/CODE_OF_CONDUCT.md)

Examples:
  ./scripts/pull-code-of-conduct.sh
  ./scripts/pull-code-of-conduct.sh ~/Projects/f00tils ~/Projects/trn
  find ~/Projects -maxdepth 1 -type d | xargs ./scripts/pull-code-of-conduct.sh
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ $# -eq 0 ]]; then
  set -- .
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

if [[ -f "$LOCAL_CANON" ]]; then
  cp "$LOCAL_CANON" "$tmp"
  src="local:${LOCAL_CANON}"
else
  curl -fsSL "$CANONICAL_URL" -o "$tmp"
  src="url:${CANONICAL_URL}"
fi

for dir in "$@"; do
  if [[ ! -d "$dir" ]]; then
    echo "skip (not a directory): $dir" >&2
    continue
  fi
  dest="${dir%/}/CODE_OF_CONDUCT.md"
  cp "$tmp" "$dest"
  echo "wrote $dest  ($src)"
done
