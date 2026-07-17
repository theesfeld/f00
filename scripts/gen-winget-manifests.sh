#!/usr/bin/env bash
# Generate winget manifests under packaging/winget/manifests/...
# Usage: gen-winget-manifests.sh <version> <SHA256SUMS-path> [release-date YYYY-MM-DD]
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:?SHA256SUMS path required}"
DATE="${3:-$(date -u +%Y-%m-%d)}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ID="theesfeld.f00"
PUB="theesfeld"
NAME="f00"
OUTDIR="${ROOT}/packaging/winget/manifests/t/${PUB}/${NAME}/${VERSION}"

sha_for() {
  local name="$1"
  awk -v f="$name" '$2 == f || $2 == ("./" f) { print toupper($1); exit }' "${SUMS}"
}

WIN="f00-x86_64-pc-windows-msvc.zip"
HASH="$(sha_for "${WIN}")"
if [[ -z "${HASH}" ]]; then
  echo "missing sha256 for ${WIN}" >&2
  exit 1
fi

mkdir -p "${OUTDIR}"

cat > "${OUTDIR}/${ID}.yaml" <<EOF
# yaml-language-server: \$schema=https://aka.ms/winget-manifest.version.1.9.0.schema.json

PackageIdentifier: ${ID}
PackageVersion: ${VERSION}
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.9.0
EOF

cat > "${OUTDIR}/${ID}.locale.en-US.yaml" <<EOF
# yaml-language-server: \$schema=https://aka.ms/winget-manifest.defaultLocale.1.9.0.schema.json

PackageIdentifier: ${ID}
PackageVersion: ${VERSION}
PackageLocale: en-US
Publisher: ${PUB}
PublisherUrl: https://github.com/${PUB}
PublisherSupportUrl: https://github.com/${PUB}/f00/issues
Author: ${PUB}
PackageName: ${NAME}
PackageUrl: https://f00.sh
License: MIT OR Apache-2.0
LicenseUrl: https://github.com/${PUB}/f00/blob/main/LICENSE-MIT
Copyright: Copyright (c) ${PUB}
ShortDescription: Modern, friendly directory lister (ls rewrite in Rust)
Description: |-
  f00 is a next-generation, cross-platform coreutils ls clone in Rust:
  modern UX by default, exact GNU behavior under --gnu for scripts,
  plus tree, JSON, icons, and git status.
Moniker: f00
Tags:
  - cli
  - coreutils
  - filesystem
  - ls
  - rust
ReleaseNotesUrl: https://github.com/${PUB}/f00/releases/tag/v${VERSION}
ManifestType: defaultLocale
ManifestVersion: 1.9.0
EOF

cat > "${OUTDIR}/${ID}.installer.yaml" <<EOF
# yaml-language-server: \$schema=https://aka.ms/winget-manifest.installer.1.9.0.schema.json

PackageIdentifier: ${ID}
PackageVersion: ${VERSION}
Platform:
  - Windows.Desktop
MinimumOSVersion: 10.0.17763.0
InstallerType: zip
NestedInstallerType: portable
NestedInstallerFiles:
  - RelativeFilePath: f00-x86_64-pc-windows-msvc\\f00.exe
    PortableCommandAlias: f00
InstallModes:
  - silent
UpgradeBehavior: install
ReleaseDate: ${DATE}
Installers:
  - Architecture: x64
    InstallerUrl: https://github.com/${PUB}/f00/releases/download/v${VERSION}/f00-x86_64-pc-windows-msvc.zip
    InstallerSha256: ${HASH}
ManifestType: installer
ManifestVersion: 1.9.0
EOF

echo "wrote winget manifests in ${OUTDIR}"
