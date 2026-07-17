#!/usr/bin/env bash
# Open/update a winget-pkgs PR for f00 using wingetcreate (when installed) or print instructions.
# Requires: WINGET_TOKEN (GitHub PAT that can fork/PR microsoft/winget-pkgs)
# Usage: publish-winget.sh <version> <SHA256SUMS-path>
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:?SHA256SUMS path required}"
TOKEN="${WINGET_TOKEN:-}"
ID="${WINGET_PACKAGE_ID:-theesfeld.f00}"
URL="https://github.com/theesfeld/f00/releases/download/v${VERSION}/f00-x86_64-pc-windows-msvc.zip"

if [[ -z "${TOKEN}" ]]; then
  echo "skip winget: no WINGET_TOKEN" >&2
  exit 0
fi

sha_for() {
  local name="$1"
  awk -v f="$name" '$2 == f || $2 == ("./" f) { print $1; exit }' "${SUMS}"
}
HASH="$(sha_for "f00-x86_64-pc-windows-msvc.zip")"
if [[ -z "${HASH}" ]]; then
  echo "missing windows zip hash" >&2
  exit 1
fi

if ! command -v wingetcreate >/dev/null 2>&1; then
  echo "wingetcreate not installed; attempting dotnet tool install" >&2
  if command -v dotnet >/dev/null 2>&1; then
    dotnet tool install --global Microsoft.Winget.Create || true
    export PATH="${PATH}:${HOME}/.dotnet/tools"
  fi
fi

if command -v wingetcreate >/dev/null 2>&1; then
  # Update existing package or create new.
  # --submit opens PR against winget-pkgs.
  wingetcreate update "${ID}" \
    --version "${VERSION}" \
    --urls "${URL}" \
    --token "${TOKEN}" \
    --submit \
    || wingetcreate new \
      --urls "${URL}" \
      --version "${VERSION}" \
      --token "${TOKEN}" \
      --submit \
      || {
        echo "wingetcreate failed; manifests left for manual PR" >&2
        exit 0
      }
  echo "winget: submitted ${ID} ${VERSION}"
  exit 0
fi

echo "winget: wingetcreate unavailable on this runner; writing packaging/winget/notes.txt"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "${ROOT}/packaging/winget"
cat > "${ROOT}/packaging/winget/notes.txt" <<EOF
PackageIdentifier: ${ID}
PackageVersion: ${VERSION}
InstallerUrl: ${URL}
InstallerSha256: ${HASH}
EOF
echo "skip auto-PR; use wingetcreate on Windows or install wingetcreate in CI"
exit 0
