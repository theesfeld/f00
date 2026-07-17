#!/usr/bin/env bash
# Push packaging/scoop/f00.json to theesfeld/scoop-bucket.
# Requires: SCOOP_BUCKET_TOKEN or HOMEBREW_TAP_TOKEN or GH_TOKEN with write access.
# Usage: publish-scoop.sh [manifest-path]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="${1:-${ROOT}/packaging/scoop/f00.json}"
TOKEN="${SCOOP_BUCKET_TOKEN:-${HOMEBREW_TAP_TOKEN:-${GH_TOKEN:-${GITHUB_TOKEN:-}}}}"
REPO="${SCOOP_BUCKET_REPO:-theesfeld/scoop-bucket}"
BRANCH="${SCOOP_BUCKET_BRANCH:-main}"

if [[ -z "${TOKEN}" ]]; then
  echo "skip scoop: no SCOOP_BUCKET_TOKEN / HOMEBREW_TAP_TOKEN / GH_TOKEN" >&2
  exit 0
fi
if [[ ! -f "${MANIFEST}" ]]; then
  echo "missing ${MANIFEST}" >&2
  exit 1
fi

# Create repo if missing (idempotent).
if ! curl -fsS -H "Authorization: Bearer ${TOKEN}" \
  "https://api.github.com/repos/${REPO}" >/dev/null 2>&1; then
  echo "creating ${REPO}"
  curl -fsS -X POST \
    -H "Authorization: Bearer ${TOKEN}" \
    -H "Accept: application/vnd.github+json" \
    https://api.github.com/user/repos \
    -d "{\"name\":\"scoop-bucket\",\"description\":\"Scoop bucket for theesfeld projects (f00, …)\",\"homepage\":\"https://f00.sh\",\"private\":false,\"auto_init\":true}" \
    >/dev/null || {
      echo "could not create ${REPO}; create it manually and re-run" >&2
      exit 0
    }
  sleep 2
fi

WORKDIR="$(mktemp -d)"
trap 'rm -rf "${WORKDIR}"' EXIT

git clone --depth 1 "https://x-access-token:${TOKEN}@github.com/${REPO}.git" "${WORKDIR}/bucket" \
  || {
    mkdir -p "${WORKDIR}/bucket"
    git -C "${WORKDIR}/bucket" init
    git -C "${WORKDIR}/bucket" remote add origin "https://x-access-token:${TOKEN}@github.com/${REPO}.git"
  }

cp "${MANIFEST}" "${WORKDIR}/bucket/f00.json"
cd "${WORKDIR}/bucket"
git config user.name "f00-release-bot"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

if [[ ! -f README.md ]]; then
  printf '# scoop-bucket\n\n```powershell\nscoop bucket add theesfeld https://github.com/theesfeld/scoop-bucket\nscoop install f00\n```\n' > README.md
fi

git add f00.json README.md
if git diff --cached --quiet; then
  echo "scoop: unchanged"
  exit 0
fi

VER="$(python3 -c "import json;print(json.load(open('f00.json'))['version'])" 2>/dev/null || true)"
git commit -m "chore(f00): bump to v${VER:-unknown}"
git branch -M "${BRANCH}" 2>/dev/null || true
git push -u origin "HEAD:${BRANCH}"

echo "published scoop manifest to ${REPO}"
