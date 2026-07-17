#!/usr/bin/env bash
# Submit or update theesfeld.f00 on microsoft/winget-pkgs via a fork + PR.
# Requires: WINGET_TOKEN (PAT that can fork public repos and open PRs)
# Usage: publish-winget.sh <version> [SHA256SUMS-path]
# If SHA256SUMS is omitted, regenerates from packaging/winget if present, or fails.
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

# Ensure fork exists
if ! gh api "repos/${FORK}" >/dev/null 2>&1; then
  echo "forking ${UPSTREAM} → ${FORK_OWNER}"
  gh repo fork "${UPSTREAM}" --clone=false --default-branch-only=false 2>/dev/null \
    || gh api -X POST "repos/${UPSTREAM}/forks" >/dev/null
  # Wait for fork availability
  for _ in $(seq 1 30); do
    gh api "repos/${FORK}" >/dev/null 2>&1 && break
    sleep 2
  done
fi

WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

git clone --depth 1 "https://x-access-token:${TOKEN}@github.com/${FORK}.git" "${WORKDIR}/winget-pkgs"
cd "${WORKDIR}/winget-pkgs"

# Sync main from upstream (best-effort)
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
  echo "winget: manifests unchanged"
  # Still try to open PR if branch exists remotely with changes
else
  git commit -m "New version: ${ID} version ${VERSION}"
fi

git push -u origin "${BRANCH}" --force

# Open PR against upstream if none open
EXISTING="$(gh pr list --repo "${UPSTREAM}" --head "${FORK_OWNER}:${BRANCH}" --json number -q '.[0].number' 2>/dev/null || true)"
if [[ -n "${EXISTING}" ]]; then
  echo "winget: PR #${EXISTING} already open"
  exit 0
fi

gh pr create --repo "${UPSTREAM}" \
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
)"

echo "winget: opened PR for ${ID} ${VERSION}"
