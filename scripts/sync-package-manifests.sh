#!/usr/bin/env bash
# Regenerate in-repo package manifests from a version + SHA256SUMS.
# Usage: sync-package-manifests.sh <version> <SHA256SUMS-path>
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:?SHA256SUMS path required}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

"${ROOT}/scripts/gen-homebrew-formula.sh" "${VERSION}" "${SUMS}"
"${ROOT}/scripts/gen-aur-pkgbuild.sh" "${VERSION}" "${SUMS}"
"${ROOT}/scripts/gen-scoop-manifest.sh" "${VERSION}" "${SUMS}"
"${ROOT}/scripts/gen-winget-manifests.sh" "${VERSION}" "${SUMS}"

echo "synced package manifests for v${VERSION}"
