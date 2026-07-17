#!/usr/bin/env bash
# Publish packaging/aur/PKGBUILD to the AUR (ssh://aur@aur.archlinux.org/f00.git).
# Requires secrets: AUR_SSH_PRIVATE_KEY, AUR_USERNAME, AUR_EMAIL
# Usage: publish-aur.sh [pkgbuild-dir]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PKGDIR="${1:-${ROOT}/packaging/aur}"
PKGNAME="${AUR_PKGNAME:-f00}"

if [[ -z "${AUR_SSH_PRIVATE_KEY:-}" ]]; then
  echo "skip aur: no AUR_SSH_PRIVATE_KEY" >&2
  exit 0
fi
if [[ -z "${AUR_USERNAME:-}" || -z "${AUR_EMAIL:-}" ]]; then
  echo "skip aur: need AUR_USERNAME and AUR_EMAIL" >&2
  exit 0
fi
if [[ ! -f "${PKGDIR}/PKGBUILD" ]]; then
  echo "missing ${PKGDIR}/PKGBUILD" >&2
  exit 1
fi

WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

KEY="${WORKDIR}/aur_key"
printf '%s\n' "${AUR_SSH_PRIVATE_KEY}" > "${KEY}"
chmod 600 "${KEY}"

export GIT_SSH_COMMAND="ssh -i ${KEY} -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile=${WORKDIR}/known_hosts"

# Ensure host key
ssh-keyscan -t rsa,ecdsa,ed25519 aur.archlinux.org >> "${WORKDIR}/known_hosts" 2>/dev/null || true

if ! git clone "ssh://aur@aur.archlinux.org/${PKGNAME}.git" "${WORKDIR}/aur" 2>"${WORKDIR}/clone.err"; then
  # Package may not exist yet — init empty and first push creates it if account can.
  echo "AUR clone failed (new package?); initializing local repo" >&2
  cat "${WORKDIR}/clone.err" >&2 || true
  mkdir -p "${WORKDIR}/aur"
  git -C "${WORKDIR}/aur" init
  git -C "${WORKDIR}/aur" remote add origin "ssh://aur@aur.archlinux.org/${PKGNAME}.git"
fi

cp "${PKGDIR}/PKGBUILD" "${WORKDIR}/aur/PKGBUILD"

# Generate .SRCINFO with makepkg if available; else minimal fallback via docker-less parse.
cd "${WORKDIR}/aur"
if command -v makepkg >/dev/null 2>&1; then
  makepkg --printsrcinfo > .SRCINFO
elif command -v docker >/dev/null 2>&1; then
  docker run --rm -v "$PWD":/pkg -w /pkg archlinux:base-devel \
    bash -c "pacman -Sy --noconfirm base-devel >/dev/null && makepkg --printsrcinfo" > .SRCINFO
else
  echo "warn: makepkg not available; writing minimal .SRCINFO" >&2
  # shellcheck disable=SC1091
  source PKGBUILD
  cat > .SRCINFO <<EOF
pkgbase = ${pkgname}
	pkgdesc = ${pkgdesc}
	pkgver = ${pkgver}
	pkgrel = ${pkgrel}
	url = ${url}
	arch = x86_64
	arch = aarch64
	license = MIT
	license = Apache
	depends = glibc
	depends = gcc-libs
	provides = f00
	conflicts = f00
	source_x86_64 = https://github.com/theesfeld/f00/releases/download/v${pkgver}/f00-x86_64-unknown-linux-gnu.tar.gz
	sha256sums_x86_64 = ${sha256sums_x86_64}
	source_aarch64 = https://github.com/theesfeld/f00/releases/download/v${pkgver}/f00-aarch64-unknown-linux-gnu.tar.gz
	sha256sums_aarch64 = ${sha256sums_aarch64}

pkgname = ${pkgname}
EOF
fi

git config user.name "${AUR_USERNAME}"
git config user.email "${AUR_EMAIL}"
git add PKGBUILD .SRCINFO
if git diff --cached --quiet; then
  echo "aur: unchanged"
  exit 0
fi

# shellcheck disable=SC1091
source PKGBUILD
git commit -m "Update to ${pkgver}"
git branch -M master 2>/dev/null || true
git push -u origin HEAD:master

echo "published AUR package ${PKGNAME} ${pkgver}"
