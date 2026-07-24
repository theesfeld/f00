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

# Prefer ASM asset names
if sha_for "f00-${VERSION}-linux-x86_64.tar.gz" >/dev/null 2>&1 \
  && [[ -n "$(sha_for "f00-${VERSION}-linux-x86_64.tar.gz")" ]]; then
  ASSET="f00-${VERSION}-linux-x86_64.tar.gz"
else
  ASSET="f00-${VERSION}-x86_64-linux.tar.gz"
fi
SHA_X86_64_LINUX="$(need "${ASSET}")"

mkdir -p "$(dirname "${OUT}")"
cat > "${OUT}" <<EOF
# Homebrew formula for f00 (pure assembly multicall coreutils suite).
#
# Install:
#   brew install theesfeld/tap/f00
#
# Official installer:
#   curl -fsSL https://f00.sh/install.sh | bash

class F00 < Formula
  desc "f00tils — pure assembly coreutils replacement (multicall, freestanding)"
  homepage "https://f00.sh"
  version "${VERSION}"
  license "MIT"

  on_linux do
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/${ASSET}"
      sha256 "${SHA_X86_64_LINUX}"
    end
  end

  def install
    bin.install "f00"
    utils = %w[
      ls cat true false yes nproc tty whoami basename dirname
      head tail wc tee seq echo pwd sleep
      env printenv realpath readlink pathchk mktemp link unlink sync truncate
      mkdir rmdir chmod touch logname hostid
      cut tr sort uniq rev tac nl fold expand unexpand paste join comm fmt od
      split csplit shuf tsort pr ptx factor numfmt expr
      cp mv rm ln chown chgrp stat df du install mkfifo mknod shred dd dir vdir
      id groups uname arch date users who pinky uptime hostname
      nice nohup timeout kill test printf
      md5sum sha1sum sha256sum sha224sum sha384sum sha512sum b2sum cksum sum
      base64 basenc base32 dircolors chroot stty stdbuf runcon chcon
    ]
    utils.each do |u|
      bin.install_symlink "f00" => "f00-#{u}"
    end
    man1.install Dir["man/man1/*.1"] if Dir.exist?("man/man1")
  end

  test do
    assert_match "f00", shell_output("#{bin}/f00-ls --version")
    system bin/"f00-true"
  end
end
EOF

echo "wrote ${OUT} for v${VERSION} (${ASSET})"
