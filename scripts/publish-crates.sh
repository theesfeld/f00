#!/usr/bin/env bash
# Publish f00 workspace crates to crates.io in dependency order.
# Requires: cargo login (or CARGO_REGISTRY_TOKEN).
set -euo pipefail
cd "$(dirname "$0")/.."

ORDER=(
  f00-core
  f00-format
  f00-compat
  f00-git
  f00-archive
  f00-tui
  f00-plugin
  f00
)

DRY="${1:-}"

for crate in "${ORDER[@]}"; do
  echo "==> publishing ${crate}"
  if [[ "$DRY" == "--dry-run" ]]; then
    cargo publish -p "$crate" --dry-run --locked
  else
    cargo publish -p "$crate" --locked
  fi
done

echo "done."
