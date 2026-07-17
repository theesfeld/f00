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

# Write .SRCINFO from PKGBUILD without makepkg (CI has no Arch userland;
# docker makepkg-as-root is forbidden).
write_srcinfo() {
  local pkgbuild="$1" out="$2"
  # shellcheck disable=SC1090
  source "${pkgbuild}"

  {
    printf 'pkgbase = %s\n' "${pkgname}"
    printf '\tpkgdesc = %s\n' "${pkgdesc}"
    printf '\tpkgver = %s\n' "${pkgver}"
    printf '\tpkgrel = %s\n' "${pkgrel}"
    printf '\turl = %s\n' "${url}"
    local a
    for a in "${arch[@]}"; do
      printf '\tarch = %s\n' "${a}"
    done
    local lic
    for lic in "${license[@]}"; do
      printf '\tlicense = %s\n' "${lic}"
    done
    local dep
    for dep in "${depends[@]}"; do
      printf '\tdepends = %s\n' "${dep}"
    done
    local p
    for p in "${provides[@]}"; do
      printf '\tprovides = %s\n' "${p}"
    done
    # Arch-specific sources (binary package)
    printf '\tsource_x86_64 = %s\n' "${source_x86_64[0]}"
    printf '\tsha256sums_x86_64 = %s\n' "${sha256sums_x86_64[0]}"
    printf '\tsource_aarch64 = %s\n' "${source_aarch64[0]}"
    printf '\tsha256sums_aarch64 = %s\n' "${sha256sums_aarch64[0]}"
    printf '\n'
    printf 'pkgname = %s\n' "${pkgname}"
  } > "${out}"
}

WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

KEY="${WORKDIR}/aur_key"
# Preserve newlines in multiline secrets (GitHub may or may not include final newline).
printf '%s\n' "${AUR_SSH_PRIVATE_KEY}" > "${KEY}"
chmod 600 "${KEY}"

export GIT_SSH_COMMAND="ssh -i ${KEY} -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile=${WORKDIR}/known_hosts"

ssh-keyscan -t rsa,ecdsa,ed25519 aur.archlinux.org >> "${WORKDIR}/known_hosts" 2>/dev/null || true

if ! git clone "ssh://aur@aur.archlinux.org/${PKGNAME}.git" "${WORKDIR}/aur" 2>"${WORKDIR}/clone.err"; then
  echo "AUR clone failed (new package?); initializing local repo" >&2
  cat "${WORKDIR}/clone.err" >&2 || true
  mkdir -p "${WORKDIR}/aur"
  git -C "${WORKDIR}/aur" init
  git -C "${WORKDIR}/aur" remote add origin "ssh://aur@aur.archlinux.org/${PKGNAME}.git"
fi

cp "${PKGDIR}/PKGBUILD" "${WORKDIR}/aur/PKGBUILD"
cd "${WORKDIR}/aur"
write_srcinfo PKGBUILD .SRCINFO

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
