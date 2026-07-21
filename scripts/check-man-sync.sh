#!/usr/bin/env bash
# Hard rule: man/f00.1 MUST match the current product surface.
#
# Fails if:
#   - man page missing
#   - Cargo.toml version not reflected in man .TH line
#   - any public long flag from `f00 --help` is absent from the man page
#   - required product sections / claims are missing
#
# Usage: scripts/check-man-sync.sh
# Optional: F00_BIN=/path/to/f00  (skip cargo build)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MAN="${ROOT}/man/f00.1"
CARGO_TOML="${ROOT}/Cargo.toml"

fail() { echo "check-man-sync: ERROR: $*" >&2; exit 1; }
info() { echo "check-man-sync: $*" >&2; }

# grep fixed-string that cannot eat --flags as options
has_plain() {
  local needle="$1" file="$2"
  # strip groff backslash escapes so \-\-json matches --json
  tr -d '\\' <"$file" | grep -F -q -- "$needle"
}

[[ -f "$MAN" ]] || fail "missing ${MAN} — man page is mandatory"

# ── version ─────────────────────────────────────────────────────────────────
version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "${CARGO_TOML}" | head -1)"
[[ -n "$version" ]] || fail "could not read version from Cargo.toml"
if ! grep -E '^\.TH[[:space:]]' "$MAN" | grep -F -q -- "${version}"; then
  fail "man page .TH line does not include version ${version} (must match Cargo.toml)"
fi
info "version ${version} present in man .TH"

# ── required structural sections ────────────────────────────────────────────
required_headings=(
  "NAME"
  "SYNOPSIS"
  "DESCRIPTION"
  "OUTPUT MODES"
  "FLAG TAXONOMY"
  "OPTIONS"
  "COLORS AND THEMES"
  "JSON OUTPUT"
  "TREE OUTPUT"
  "ENVIRONMENT"
  "FILES"
  "EXIT STATUS"
  "INSTALLATION"
  "UPDATING"
  "EXAMPLES"
  "SEE ALSO"
)
for h in "${required_headings[@]}"; do
  if ! grep -F -q -- ".SH ${h}" "$MAN"; then
    fail "man page missing required section: .SH ${h}"
  fi
done
info "required sections present"

# ── required product claims / surfaces ──────────────────────────────────────
required_strings=(
  "LS_COLORS"
  "json-full"
  "--gnu"
  "--no-gnu"
  "--tree"
  "--json"
  "coreutils"
  "eza"
  "lsd"
  "Rust"
  "f00-tui"
  "--update"
  "--check-update"
  "install.sh"
)
for s in "${required_strings[@]}"; do
  if ! has_plain "$s" "$MAN"; then
    fail "man page missing required string/claim: ${s}"
  fi
done
info "required product strings present"

# ── every public long flag from --help ──────────────────────────────────────
bin="${F00_BIN:-}"
if [[ -z "$bin" ]]; then
  info "building f00 for --help surface…"
  (cd "$ROOT" && cargo build -q -p f00)
  bin="${ROOT}/target/debug/f00"
fi
[[ -x "$bin" ]] || fail "f00 binary not executable: ${bin}"

help_out="$("$bin" --help 2>&1)" || fail "f00 --help failed"

# Long flags to ignore (developer dumps / clap meta, not user product surface)
ignore_flags=(
  --generate-completions
  --generate-man
  --help
  --version
)

is_ignored() {
  local f="$1" x
  for x in "${ignore_flags[@]}"; do
    [[ "$f" == "$x" ]] && return 0
  done
  return 1
}

# Only option-definition lines (indent + optional short + long flag), not prose.
# Examples:
#   -a, --all
#       --si
#   -F, --classify[=WHEN]
mapfile -t flags < <(
  printf '%s\n' "$help_out" \
    | grep -E '^[[:space:]]+(-[A-Za-z0-9],[[:space:]]*)?--[a-z0-9][a-z0-9-]*' \
    | grep -oE -- '--[a-z0-9][a-z0-9-]*' \
    | sort -u
)

[[ "${#flags[@]}" -gt 10 ]] || fail "parsed too few long flags from --help (${#flags[@]}); parser broken?"

missing=0
checked=0
for flag in "${flags[@]}"; do
  is_ignored "$flag" && continue
  checked=$((checked + 1))
  if has_plain "$flag" "$MAN"; then
    continue
  fi
  echo "check-man-sync: missing flag in man/f00.1: ${flag}" >&2
  missing=$((missing + 1))
done

if [[ "$missing" -gt 0 ]]; then
  fail "${missing} public long flag(s) from \`f00 --help\` are not documented in man/f00.1 (checked ${checked})"
fi
info "all ${checked} public long flags from --help are documented"

# ── short-flag smoke (common GNU + f00) ─────────────────────────────────────
short_required=(a A l R r t S h j Z)
plain_man="$(tr -d '\\' <"$MAN")"
for s in "${short_required[@]}"; do
  if ! printf '%s\n' "$plain_man" | grep -E -q -- "(^|[^a-zA-Z0-9])-${s}([^a-zA-Z0-9]|$)"; then
    fail "man page missing short flag documentation for -${s}"
  fi
done
info "core short flags present"

echo "check-man-sync: OK — man/f00.1 matches current project surface" >&2
