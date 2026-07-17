#!/usr/bin/env bash
# Generate Formula/f00.rb from version + SHA256SUMS file.
# Usage: gen-homebrew-formula.sh <version> <SHA256SUMS-path> [output-path]
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:?SHA256SUMS path required}"
OUT="${3:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if [[ -z "${OUT}" ]]; then
  OUT="${ROOT}/Formula/f00.rb"
fi

sha_for() {
  local name="$1"
  # SHA256SUMS lines: "<hash>  <filename>" or "<hash> <filename>"
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

SHA_AARCH64_DARWIN="$(need "f00-aarch64-apple-darwin.tar.gz")"
SHA_X86_64_DARWIN="$(need "f00-x86_64-apple-darwin.tar.gz")"
SHA_AARCH64_LINUX="$(need "f00-aarch64-unknown-linux-gnu.tar.gz")"
SHA_X86_64_LINUX="$(need "f00-x86_64-unknown-linux-gnu.tar.gz")"

mkdir -p "$(dirname "${OUT}")"
cat > "${OUT}" <<EOF
# Homebrew formula for f00.
#
# Install:
#   brew install theesfeld/tap/f00
#
# Official installer (any platform):
#   curl -fsSL https://f00.sh/install.sh | bash
#
# This file is auto-updated on release (Refs: packaging phase).

class F00 < Formula
  desc "Modern, friendly directory lister (ls rewrite in Rust)"
  homepage "https://f00.sh"
  version "${VERSION}"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      sha256 "${SHA_AARCH64_DARWIN}"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
      sha256 "${SHA_X86_64_DARWIN}"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "${SHA_AARCH64_LINUX}"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${SHA_X86_64_LINUX}"
    end
  end

  def install
    # Release tarball root is f00-<target-triple>/f00
    bin.install Dir["f00-*/f00"].first
  end

  test do
    assert_match "f00", shell_output("#{bin}/f00 --version")
  end
end
EOF

echo "wrote ${OUT} for v${VERSION}"
