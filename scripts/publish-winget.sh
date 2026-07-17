#!/usr/bin/env bash
# Submit or update theesfeld.f00 on microsoft/winget-pkgs via a fork + PR.
# Requires:
#   - WINGET_TOKEN: classic PAT with `public_repo` (recommended) OR fine-grained
#     with write access to theesfeld/winget-pkgs after a manual fork
#   - Fork https://github.com/microsoft/winget-pkgs → theesfeld/winget-pkgs (once)
# Usage: publish-winget.sh <version> [SHA256SUMS-path]
set -euo pipefail

VERSION="${1:?version required (no v prefix)}"
SUMS="${2:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TOKEN="${WINGET_TOKEN:-}"
ID="theesfeld.f00"
PUB="theesfeld"
NAME="f00"
UPSTREAM="microsoft/winget-pkgs"
FORK_OWNER="${WINGET_FORK_OWNER:-${PUB}}"
FORK="${FORK_OWNER}/winget-pkgs"

if [[ -z "${TOKEN}" ]]; then
  echo "skip winget: no WINGET_TOKEN" >&2
  exit 0
fi

export GH_TOKEN="${TOKEN}"
export GITHUB_TOKEN="${TOKEN}"

if [[ -n "${SUMS}" ]]; then
  "${ROOT}/scripts/gen-winget-manifests.sh" "${VERSION}" "${SUMS}"
fi

MANIFEST_DIR="${ROOT}/packaging/winget/manifests/t/${PUB}/${NAME}/${VERSION}"
if [[ ! -d "${MANIFEST_DIR}" ]]; then
  echo "missing manifests at ${MANIFEST_DIR}" >&2
  exit 1
fi

# Prefer an existing fork. Auto-fork only as a best-effort (classic PAT).
if ! gh api "repos/${FORK}" >/dev/null 2>&1; then
  echo "fork ${FORK} not found; attempting create from ${UPSTREAM}"
  if ! gh repo fork "${UPSTREAM}" --clone=false 2>"${TMPDIR:-/tmp}/winget-fork.err"; then
    cat "${TMPDIR:-/tmp}/winget-fork.err" >&2 || true
    echo "" >&2
    echo "winget: cannot create fork with this token." >&2
    echo "Do this once in the browser, then re-run:" >&2
    echo "  1) Open https://github.com/microsoft/winget-pkgs" >&2
    echo "  2) Click Fork → create theesfeld/winget-pkgs" >&2
    echo "  3) Use a classic PAT (public_repo) as WINGET_TOKEN, or a fine-grained" >&2
    echo "     token with Contents+PR write on theesfeld/winget-pkgs" >&2
    exit 1
  fi
  for _ in $(seq 1 30); do
    gh api "repos/${FORK}" >/dev/null 2>&1 && break
    sleep 2
  done
fi

if ! gh api "repos/${FORK}" >/dev/null 2>&1; then
  echo "winget: fork ${FORK} still missing" >&2
  exit 1
fi

WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

git clone --depth 1 "https://x-access-token:${TOKEN}@github.com/${FORK}.git" "${WORKDIR}/winget-pkgs"
cd "${WORKDIR}/winget-pkgs"

git remote add upstream "https://github.com/${UPSTREAM}.git" 2>/dev/null || true
git fetch --depth 1 upstream master 2>/dev/null || git fetch --depth 1 upstream main 2>/dev/null || true
BASE_BRANCH="master"
if git show-ref --verify --quiet refs/remotes/upstream/master; then
  git checkout -B master upstream/master
elif git show-ref --verify --quiet refs/remotes/upstream/main; then
  BASE_BRANCH="main"
  git checkout -B main upstream/main
else
  git checkout master 2>/dev/null || git checkout main
  BASE_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
fi

BRANCH="f00-${VERSION}"
git checkout -B "${BRANCH}"

DEST="manifests/t/${PUB}/${NAME}/${VERSION}"
mkdir -p "${DEST}"
cp -f "${MANIFEST_DIR}/"* "${DEST}/"

git config user.name "f00-release-bot"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
git add "${DEST}"
if git diff --cached --quiet; then
  echo "winget: manifests unchanged on branch"
else
  git commit -m "New version: ${ID} version ${VERSION}"
fi

git push -u origin "${BRANCH}" --force

EXISTING="$(gh pr list --repo "${UPSTREAM}" --head "${FORK_OWNER}:${BRANCH}" --json number -q '.[0].number' 2>/dev/null || true)"
if [[ -n "${EXISTING}" ]]; then
  echo "winget: PR #${EXISTING} already open → https://github.com/${UPSTREAM}/pull/${EXISTING}"
  exit 0
fi

URL="$(gh pr create --repo "${UPSTREAM}" \
  --base "${BASE_BRANCH}" \
  --head "${FORK_OWNER}:${BRANCH}" \
  --title "New version: ${ID} version ${VERSION}" \
  --body "$(cat <<EOF
# ${ID} ${VERSION}

Portable zip installer from GitHub Releases.

- Package: https://f00.sh
- Release: https://github.com/theesfeld/f00/releases/tag/v${VERSION}
- Publisher: theesfeld

## Checklist
- [x] Manifests generated from release SHA-256
- [x] Installer is nested portable \`f00.exe\`

Submitted by automated release packaging for theesfeld/f00.
EOF
)")"

echo "winget: opened ${URL}"
