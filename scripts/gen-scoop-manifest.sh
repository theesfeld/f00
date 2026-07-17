#!/usr/bin/env bash
# Generate packaging/scoop/f00.json from version + SHA256SUMS.
# Usage: gen-scoop-manifest.sh <version> <SHA256SUMS-path> [output-path]
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:?SHA256SUMS path required}"
OUT="${3:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if [[ -z "${OUT}" ]]; then
  OUT="${ROOT}/packaging/scoop/f00.json"
fi

sha_for() {
  local name="$1"
  awk -v f="$name" '$2 == f || $2 == ("./" f) { print $1; exit }' "${SUMS}"
}

WIN="f00-x86_64-pc-windows-msvc.zip"
HASH="$(sha_for "${WIN}")"
if [[ -z "${HASH}" ]]; then
  echo "missing sha256 for ${WIN}" >&2
  exit 1
fi

mkdir -p "$(dirname "${OUT}")"
cat > "${OUT}" <<EOF
{
  "version": "${VERSION}",
  "description": "Modern, friendly directory lister (ls rewrite in Rust)",
  "homepage": "https://f00.sh",
  "license": "MIT OR Apache-2.0",
  "architecture": {
    "64bit": {
      "url": "https://github.com/theesfeld/f00/releases/download/v${VERSION}/f00-x86_64-pc-windows-msvc.zip",
      "hash": "${HASH}",
      "extract_dir": "f00-x86_64-pc-windows-msvc"
    }
  },
  "bin": "f00.exe",
  "checkver": {
    "github": "https://github.com/theesfeld/f00"
  },
  "autoupdate": {
    "architecture": {
      "64bit": {
        "url": "https://github.com/theesfeld/f00/releases/download/v\$version/f00-x86_64-pc-windows-msvc.zip",
        "extract_dir": "f00-x86_64-pc-windows-msvc"
      }
    },
    "hash": {
      "url": "https://github.com/theesfeld/f00/releases/download/v\$version/SHA256SUMS"
    }
  }
}
EOF

echo "wrote ${OUT} for v${VERSION}"
