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

# Write .SRCINFO without makepkg.
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
    if declare -p depends &>/dev/null; then
      local dep
      for dep in "${depends[@]+"${depends[@]}"}"; do
        [[ -n "${dep}" ]] && printf '\tdepends = %s\n' "${dep}"
      done
    fi
    if declare -p makedepends &>/dev/null; then
      local md
      for md in "${makedepends[@]+"${makedepends[@]}"}"; do
        [[ -n "${md}" ]] && printf '\tmakedepends = %s\n' "${md}"
      done
    fi
    if declare -p provides &>/dev/null; then
      local p
      for p in "${provides[@]+"${provides[@]}"}"; do
        [[ -n "${p}" ]] && printf '\tprovides = %s\n' "${p}"
      done
    fi
    if declare -p conflicts &>/dev/null; then
      local c
      for c in "${conflicts[@]+"${conflicts[@]}"}"; do
        [[ -n "${c}" ]] && printf '\tconflicts = %s\n' "${c}"
      done
    fi
    # Plain source= or arch-specific source_x86_64=
    if declare -p source_x86_64 &>/dev/null; then
      printf '\tsource_x86_64 = %s\n' "${source_x86_64[0]}"
      printf '\tsha256sums_x86_64 = %s\n' "${sha256sums_x86_64[0]}"
    fi
    if declare -p source_aarch64 &>/dev/null; then
      printf '\tsource_aarch64 = %s\n' "${source_aarch64[0]}"
      printf '\tsha256sums_aarch64 = %s\n' "${sha256sums_aarch64[0]}"
    fi
    if declare -p source &>/dev/null; then
      local s i=0
      for s in "${source[@]}"; do
        printf '\tsource = %s\n' "${s}"
      done
      if declare -p sha256sums &>/dev/null; then
        for s in "${sha256sums[@]}"; do
          printf '\tsha256sums = %s\n' "${s}"
        done
      fi
    fi
    printf '\n'
    printf 'pkgname = %s\n' "${pkgname}"
  } > "${out}"
}

WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

KEY="${WORKDIR}/aur_key"
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
