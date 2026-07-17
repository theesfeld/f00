#!/usr/bin/env bash
# Build .deb and .rpm packages from release tarballs using nfpm.
# Usage: build-linux-packages.sh <version> <assets-dir> <out-dir>
# Example: build-linux-packages.sh 0.10.4 ./release-assets ./release-assets
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
ASSETS="${2:?assets dir required}"
OUT="${3:?out dir required}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TEMPLATE="${ROOT}/packaging/nfpm/f00.yaml"

if ! command -v nfpm >/dev/null 2>&1; then
  echo "nfpm not found; install from https://nfpm.goreleaser.com/" >&2
  exit 1
fi

mkdir -p "${OUT}"
WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

render_nfpm() {
  local arch="$1" bin_path="$2" cfg="$3"
  # Avoid envsubst dependency; only substitute known placeholders.
  sed \
    -e "s|\${VERSION}|${VERSION}|g" \
    -e "s|\${ARCH}|${arch}|g" \
    -e "s|\${BIN_PATH}|${bin_path}|g" \
    "${TEMPLATE}" > "${cfg}"
}

build_one() {
  local target="$1" arch="$2"
  local tarball="${ASSETS}/f00-${target}.tar.gz"
  if [[ ! -f "${tarball}" ]]; then
    echo "skip missing ${tarball}" >&2
    return 0
  fi

  local stage="${WORKDIR}/${target}"
  mkdir -p "${stage}"
  tar -xzf "${tarball}" -C "${stage}"
  local bin="${stage}/f00-${target}/f00"
  if [[ ! -f "${bin}" ]]; then
    bin="$(find "${stage}" -type f -name f00 | head -n1)"
  fi
  if [[ ! -f "${bin}" ]]; then
    echo "binary not found in ${tarball}" >&2
    return 1
  fi
  chmod +x "${bin}"

  local cfg="${WORKDIR}/nfpm-${arch}.yaml"
  render_nfpm "${arch}" "${bin}" "${cfg}"

  nfpm package --config "${cfg}" --packager deb --target "${OUT}"
  nfpm package --config "${cfg}" --packager rpm --target "${OUT}"
  echo "built deb/rpm for ${target} (${arch})"
}

build_one "x86_64-unknown-linux-gnu" "amd64"
build_one "aarch64-unknown-linux-gnu" "arm64"

echo "linux packages in ${OUT}:"
ls -la "${OUT}"/*.{deb,rpm} 2>/dev/null || true
