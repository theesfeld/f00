#!/usr/bin/env bash
# Generate packaging/aur/PKGBUILD from version + SHA256SUMS.
# Usage: gen-aur-pkgbuild.sh <version> <SHA256SUMS-path> [output-path]
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:?SHA256SUMS path required}"
OUT="${3:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if [[ -z "${OUT}" ]]; then
  OUT="${ROOT}/packaging/aur/PKGBUILD"
fi

sha_for() {
  local name="$1"
  awk -v f="$name" '$2 == f || $2 == ("./" f) { print $1; exit }' "${SUMS}"
}

need() {
  local h
  h="$(sha_for "$1")"
  if [[ -z "${h}" ]]; then
    echo "missing sha256 for $1 in ${SUMS}" >&2
    exit 1
  fi
  printf '%s' "${h}"
}

SHA_X86="$(need "f00-x86_64-unknown-linux-gnu.tar.gz")"
SHA_ARM="$(need "f00-aarch64-unknown-linux-gnu.tar.gz")"

mkdir -p "$(dirname "${OUT}")"
cat > "${OUT}" <<EOF
# Maintainer: theesfeld
# Auto-updated on release by GitHub Actions.
pkgname=f00
pkgver=${VERSION}
pkgrel=1
pkgdesc="Modern, friendly directory lister (ls rewrite in Rust)"
arch=('x86_64' 'aarch64')
url="https://f00.sh"
license=('MIT' 'Apache')
depends=('glibc' 'gcc-libs')
provides=('f00')
source_x86_64=("https://github.com/theesfeld/f00/releases/download/v\${pkgver}/f00-x86_64-unknown-linux-gnu.tar.gz")
source_aarch64=("https://github.com/theesfeld/f00/releases/download/v\${pkgver}/f00-aarch64-unknown-linux-gnu.tar.gz")
sha256sums_x86_64=('${SHA_X86}')
sha256sums_aarch64=('${SHA_ARM}')

package() {
  local dir
  if [[ "\${CARCH}" == "x86_64" ]]; then
    dir="f00-x86_64-unknown-linux-gnu"
  else
    dir="f00-aarch64-unknown-linux-gnu"
  fi
  install -Dm755 "\${srcdir}/\${dir}/f00" "\${pkgdir}/usr/bin/f00"
  if [[ -f "\${srcdir}/\${dir}/LICENSE-MIT" ]]; then
    install -Dm644 "\${srcdir}/\${dir}/LICENSE-MIT" \\
      "\${pkgdir}/usr/share/licenses/\${pkgname}/LICENSE-MIT"
  fi
  if [[ -f "\${srcdir}/\${dir}/LICENSE-APACHE" ]]; then
    install -Dm644 "\${srcdir}/\${dir}/LICENSE-APACHE" \\
      "\${pkgdir}/usr/share/licenses/\${pkgname}/LICENSE-APACHE"
  fi
}
EOF

echo "wrote ${OUT} for v${VERSION}"
