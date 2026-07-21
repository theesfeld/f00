#!/usr/bin/env bash
# f00 installer — https://f00.sh
# Usage:
#   curl -fsSL https://f00.sh/install.sh | bash
#   curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.5.1 bash
#   curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash
#
# Env:
#   F00_VERSION      Pin a release tag (e.g. v0.5.1). Default: latest
#   INSTALL_DIR      Override install directory (default: ~/.local/bin)
#   ADD_PATH         1 = ensure install dir on PATH via shell rc (default when missing)
#                    0 = never edit shell rc (print snippet only)
#   F00_INSTALL_LS   If set to 1, also symlink ls -> f00
#   F00_INSTALL_TUI  If set to 0, skip installing f00-tui when present in the archive (default: install)
#   F00_INSTALL_MAN  If set to 0, skip installing the man page (default: install when present)
#   MAN_DIR          Override man page directory (default: ~/.local/share/man/man1 or $XDG_DATA_HOME/man/man1)
#   F00_NO_COLOR     If set, disable ANSI colors
set -euo pipefail

REPO="theesfeld/f00"
BINARY="f00"
TUI_BINARY="f00-tui"
GITHUB_API="https://api.github.com/repos/${REPO}"
GITHUB_RELEASES="https://github.com/${REPO}/releases"

# ── colors ──────────────────────────────────────────────────────────────────
if [[ -z "${F00_NO_COLOR:-}" && -t 2 ]] || [[ -z "${F00_NO_COLOR:-}" && -n "${FORCE_COLOR:-}" ]]; then
  BOLD='\033[1m'
  DIM='\033[2m'
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  YELLOW='\033[0;33m'
  CYAN='\033[0;36m'
  RESET='\033[0m'
else
  # also allow color when piped from curl if stderr is a tty-ish env; keep simple off when NO_COLOR
  if [[ -n "${NO_COLOR:-}" || -n "${F00_NO_COLOR:-}" ]]; then
    BOLD='' DIM='' RED='' GREEN='' YELLOW='' CYAN='' RESET=''
  else
    BOLD='\033[1m'
    DIM='\033[2m'
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    CYAN='\033[0;36m'
    RESET='\033[0m'
  fi
fi

info()  { printf "${CYAN}==>${RESET} ${BOLD}%s${RESET}\n" "$*" >&2; }
ok()    { printf "${GREEN}==>${RESET} %s\n" "$*" >&2; }
warn()  { printf "${YELLOW}warn:${RESET} %s\n" "$*" >&2; }
err()   { printf "${RED}error:${RESET} %s\n" "$*" >&2; }
die()   { err "$*"; exit 1; }

# ── helpers ─────────────────────────────────────────────────────────────────
need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

download() {
  local url="$1" dest="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL --retry 3 --retry-delay 1 -o "$dest" "$url"
  elif command -v wget >/dev/null 2>&1; then
    wget -q -O "$dest" "$url"
  else
    die "need curl or wget to download"
  fi
}

# ── platform ────────────────────────────────────────────────────────────────
detect_os() {
  local u
  u="$(uname -s | tr '[:upper:]' '[:lower:]')"
  case "$u" in
    linux*)  echo "linux" ;;
    darwin*) echo "darwin" ;;
    freebsd*) echo "freebsd" ;;
    msys*|mingw*|cygwin*) echo "windows" ;;
    *) die "unsupported OS: $(uname -s). See https://github.com/${REPO}/releases" ;;
  esac
}

detect_arch() {
  local m
  m="$(uname -m)"
  case "$m" in
    x86_64|amd64)  echo "x86_64" ;;
    aarch64|arm64) echo "aarch64" ;;
    armv7l|armv7)  die "32-bit ARM is not supported yet" ;;
    i386|i686)     die "32-bit x86 is not supported" ;;
    *) die "unsupported architecture: $m" ;;
  esac
}

rust_target() {
  local os="$1" arch="$2"
  case "${os}-${arch}" in
    linux-x86_64)   echo "x86_64-unknown-linux-gnu" ;;
    linux-aarch64)  echo "aarch64-unknown-linux-gnu" ;;
    darwin-x86_64)  echo "x86_64-apple-darwin" ;;
    darwin-aarch64) echo "aarch64-apple-darwin" ;;
    freebsd-x86_64) echo "x86_64-unknown-freebsd" ;;
    freebsd-aarch64) echo "aarch64-unknown-freebsd" ;;
    windows-x86_64) echo "x86_64-pc-windows-msvc" ;;
    windows-aarch64) echo "aarch64-pc-windows-msvc" ;;
    *) die "no release target for ${os}/${arch}" ;;
  esac
}

# ── windows guidance ────────────────────────────────────────────────────────
windows_instructions() {
  cat >&2 <<EOF
${BOLD}f00 on Windows${RESET}

This curl|bash installer targets Unix-like shells. On Windows you can:

  1. Download the latest .zip from:
     ${GITHUB_RELEASES}

     Asset pattern:
       f00-x86_64-pc-windows-msvc.zip

  2. Or, when available:
       winget install f00
       scoop install f00

  3. Git Bash / MSYS users: re-run this script; it will try the Windows zip
     if uname reports a Windows environment.

See https://f00.sh for details.
EOF
}

# ── version / asset ─────────────────────────────────────────────────────────
resolve_version() {
  if [[ -n "${F00_VERSION:-}" ]]; then
    local v="${F00_VERSION}"
    # accept 0.1.0 or v0.1.0
    if [[ "$v" != v* ]]; then
      v="v${v}"
    fi
    echo "$v"
    return
  fi
  # latest release tag via redirect
  local tag
  if command -v curl >/dev/null 2>&1; then
    tag="$(curl -fsSL -o /dev/null -w '%{url_effective}' "${GITHUB_RELEASES}/latest" | sed 's|.*/||')"
  else
    tag="$(wget -q -O /dev/null --server-response "${GITHUB_RELEASES}/latest" 2>&1 | awk '/^  Location: /{print $2}' | tail -1 | sed 's|.*/||' | tr -d '\r')"
  fi
  [[ -n "$tag" && "$tag" != "latest" ]] || die "could not resolve latest release. Set F00_VERSION=vX.Y.Z"
  echo "$tag"
}

asset_name() {
  local version="$1" target="$2" os="$3"
  if [[ "$os" == "windows" ]]; then
    echo "${BINARY}-${target}.zip"
  else
    echo "${BINARY}-${target}.tar.gz"
  fi
}

# ── install dirs ────────────────────────────────────────────────────────────
pick_man_dir() {
  if [[ -n "${MAN_DIR:-}" ]]; then
    echo "$MAN_DIR"
    return
  fi
  if [[ -n "${XDG_DATA_HOME:-}" ]]; then
    echo "${XDG_DATA_HOME}/man/man1"
  else
    echo "${HOME}/.local/share/man/man1"
  fi
}

pick_install_dir() {
  if [[ -n "${INSTALL_DIR:-}" ]]; then
    echo "$INSTALL_DIR"
    return
  fi
  # XDG user binaries — already on PATH for modern setups (Home Manager, etc.).
  local home_bin="${HOME}/.local/bin"
  local system_bin="/usr/local/bin"
  if mkdir -p "$home_bin" 2>/dev/null && [[ -w "$home_bin" ]]; then
    echo "$home_bin"
    return
  fi
  if [[ -w "$system_bin" ]]; then
    echo "$system_bin"
    return
  fi
  mkdir -p "$home_bin" || die "cannot create $home_bin"
  echo "$home_bin"
}

is_in_path() {
  local dir="$1"
  case ":${PATH}:" in
    *:"$dir":*) return 0 ;;
    *) return 1 ;;
  esac
}

# Idempotently ensure install_dir is on PATH for future interactive shells.
ensure_path_rc() {
  local dir="$1"
  # ADD_PATH=0 → never edit rc. Default: add when dir is not currently on PATH.
  local add="${ADD_PATH:-}"
  if [[ "$add" == "0" ]]; then
    return 0
  fi
  if is_in_path "$dir" && [[ "$add" != "1" ]]; then
    return 0
  fi
  # If already on PATH and not forced, skip.
  if is_in_path "$dir" && [[ "$add" != "1" ]]; then
    return 0
  fi

  local marker="# f00 installer: PATH (${dir})"
  local line="export PATH=\"${dir}:\$PATH\""
  local shell_name rc fish_rc
  shell_name="$(basename "${SHELL:-/bin/sh}")"

  case "$shell_name" in
    zsh)  rc="${ZDOTDIR:-$HOME}/.zshrc" ;;
    bash) rc="${HOME}/.bashrc" ;;
    fish)
      fish_rc="${HOME}/.config/fish/config.fish"
      mkdir -p "$(dirname "$fish_rc")"
      if [[ -f "$fish_rc" ]] && grep -Fq "$marker" "$fish_rc" 2>/dev/null; then
        ok "PATH already configured in ${fish_rc}"
        return 0
      fi
      {
        echo ""
        echo "$marker"
        echo "fish_add_path ${dir}"
      } >>"$fish_rc"
      ok "added ${dir} to PATH in ${fish_rc} (new shells)"
      return 0
      ;;
    *) rc="${HOME}/.profile" ;;
  esac

  touch "$rc"
  if grep -Fq "$marker" "$rc" 2>/dev/null; then
    ok "PATH already configured in ${rc}"
    return 0
  fi
  {
    echo ""
    echo "$marker"
    echo "$line"
  } >>"$rc"
  ok "added ${dir} to PATH in ${rc} (new shells)"
  printf "  ${DIM}This shell: export PATH=\"%s:\$PATH\"${RESET}\n" "$dir" >&2
}

# ── checksum ────────────────────────────────────────────────────────────────
verify_checksum() {
  local file="$1" sums_file="$2" asset="$3"
  [[ -f "$sums_file" ]] || { warn "SHA256SUMS not found; skipping verification"; return 0; }

  local expected
  expected="$(grep -E "[[:space:]]${asset}\$" "$sums_file" | awk '{print $1}' | head -1 || true)"
  if [[ -z "$expected" ]]; then
    # try basename match without path
    expected="$(grep -F "$asset" "$sums_file" | awk '{print $1}' | head -1 || true)"
  fi
  [[ -n "$expected" ]] || { warn "no checksum entry for ${asset}; skipping"; return 0; }

  local actual
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$file" | awk '{print $1}')"
  elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "$file" | awk '{print $1}')"
  else
    warn "sha256sum/shasum not found; skipping verification"
    return 0
  fi

  if [[ "$actual" != "$expected" ]]; then
    die "checksum mismatch for ${asset}
  expected: ${expected}
  actual:   ${actual}"
  fi
  ok "checksum verified"
}

# ── main ────────────────────────────────────────────────────────────────────
main() {
  printf "\n${BOLD}f00${RESET} ${DIM}installer${RESET}\n" >&2
  printf "${DIM}https://f00.sh${RESET}\n\n" >&2

  need_cmd uname
  need_cmd mktemp
  need_cmd mkdir
  need_cmd tar
  # unzip only required on windows path

  local os arch target version install_dir asset url tmp
  os="$(detect_os)"
  arch="$(detect_arch)"

  if [[ "$os" == "windows" ]]; then
    # If we have a proper shell + unzip, still try the zip asset.
    if ! command -v unzip >/dev/null 2>&1; then
      windows_instructions
      die "unzip not available; download a release manually"
    fi
  fi

  target="$(rust_target "$os" "$arch")"
  info "platform: ${os}/${arch} (${target})"

  version="$(resolve_version)"
  info "version:  ${version}"

  asset="$(asset_name "$version" "$target" "$os")"
  url="${GITHUB_RELEASES}/download/${version}/${asset}"
  install_dir="$(pick_install_dir)"
  mkdir -p "$install_dir"

  tmp="$(mktemp -d 2>/dev/null || mktemp -d -t f00-install)"
  # shellcheck disable=SC2064
  trap "rm -rf '$tmp'" EXIT

  info "downloading ${asset}"
  if ! download "$url" "${tmp}/${asset}"; then
    die "download failed: ${url}
Is ${version} published? Check ${GITHUB_RELEASES}"
  fi

  # optional checksums
  local sums_url="${GITHUB_RELEASES}/download/${version}/SHA256SUMS"
  if download "$sums_url" "${tmp}/SHA256SUMS" 2>/dev/null; then
    verify_checksum "${tmp}/${asset}" "${tmp}/SHA256SUMS" "$asset"
  else
    warn "SHA256SUMS not available for ${version}; skipping verification"
  fi

  info "extracting"
  local extract_dir="${tmp}/out"
  mkdir -p "$extract_dir"
  case "$asset" in
    *.tar.gz) tar -xzf "${tmp}/${asset}" -C "$extract_dir" ;;
    *.zip)    need_cmd unzip; unzip -q "${tmp}/${asset}" -d "$extract_dir" ;;
    *) die "unknown archive format: $asset" ;;
  esac

  # find binary (archive may be flat or have a top-level dir)
  local bin_src
  bin_src="$(find "$extract_dir" -type f \( -name "$BINARY" -o -name "${BINARY}.exe" \) | head -1)"
  [[ -n "$bin_src" ]] || die "binary '${BINARY}' not found inside ${asset}"

  local bin_dst="${install_dir}/${BINARY}"
  if [[ "$os" == "windows" ]]; then
    bin_dst="${install_dir}/${BINARY}.exe"
  fi

  # idempotent replace
  if [[ -f "$bin_dst" ]]; then
    info "replacing existing ${bin_dst}"
  fi
  install -m 755 "$bin_src" "$bin_dst" 2>/dev/null || {
    cp "$bin_src" "$bin_dst"
    chmod 755 "$bin_dst"
  }

  if [[ "${F00_INSTALL_LS:-0}" == "1" ]]; then
    local ls_link="${install_dir}/ls"
    if [[ -e "$ls_link" && ! -L "$ls_link" ]]; then
      warn "not linking ls: ${ls_link} exists and is not a symlink"
    else
      ln -sfn "$bin_dst" "$ls_link"
      ok "symlinked ls -> ${BINARY} (${ls_link})"
    fi
  fi

  ok "installed ${BINARY} to ${bin_dst}"

  # Companion dual-pane browser (optional; present in v0.11+ release archives)
  local tui_src=""
  if [[ "${F00_INSTALL_TUI:-1}" != "0" ]]; then
    tui_src="$(find "$extract_dir" -type f \( -name "$TUI_BINARY" -o -name "${TUI_BINARY}.exe" \) | head -1 || true)"
  fi
  local tui_dst=""
  if [[ -n "${tui_src}" ]]; then
    tui_dst="${install_dir}/${TUI_BINARY}"
    if [[ "$os" == "windows" ]]; then
      tui_dst="${install_dir}/${TUI_BINARY}.exe"
    fi
    install -m 755 "$tui_src" "$tui_dst" 2>/dev/null || {
      cp "$tui_src" "$tui_dst"
      chmod 755 "$tui_dst"
    }
    ok "installed ${TUI_BINARY} to ${tui_dst}"
  fi

  # Man page (f00.1) — present in release archives from the 0.12 train onward.
  # Skip on Windows; opt out with F00_INSTALL_MAN=0.
  if [[ "$os" != "windows" && "${F00_INSTALL_MAN:-1}" != "0" ]]; then
    local man_src=""
    man_src="$(find "$extract_dir" -type f \( -name 'f00.1' -o -path '*/man/f00.1' \) | head -1 || true)"
    if [[ -n "$man_src" ]]; then
      local man_dir man_dst
      man_dir="$(pick_man_dir)"
      mkdir -p "$man_dir"
      man_dst="${man_dir}/f00.1"
      install -m 644 "$man_src" "$man_dst" 2>/dev/null || {
        cp "$man_src" "$man_dst"
        chmod 644 "$man_dst"
      }
      ok "installed man page to ${man_dst}"
      if ! man -w f00 >/dev/null 2>&1; then
        warn "man cannot find f00 yet; try: export MANPATH=\"${man_dir%/man1}${MANPATH:+:\$MANPATH}\""
      fi
    else
      warn "man page not in archive (older release); see https://github.com/${REPO}/blob/main/man/f00.1"
    fi
  fi

  # Migrate away from legacy ~/.f00/bin if present and we installed elsewhere.
  local legacy="${HOME}/.f00/bin/${BINARY}"
  if [[ -x "$legacy" && "$bin_dst" != "$legacy" ]]; then
    warn "legacy install found at ${legacy} — prefer: ${bin_dst}"
    warn "you can remove the old one: rm -f '${legacy}'"
  fi

  if is_in_path "$install_dir"; then
    ok "${install_dir} is on PATH"
  else
    warn "${install_dir} is not on PATH in this shell"
    printf "  export PATH=\"%s:\$PATH\"\n" "$install_dir" >&2
    ensure_path_rc "$install_dir"
  fi
  # Even if currently on PATH, allow force ADD_PATH=1
  if [[ "${ADD_PATH:-}" == "1" ]]; then
    ensure_path_rc "$install_dir"
  fi

  # smoke check
  if [[ -x "$bin_dst" ]]; then
    if "$bin_dst" --version >/dev/null 2>&1; then
      ok "$("$bin_dst" --version 2>/dev/null || echo "f00 ${version}")"
    fi
  fi

  printf "\n${GREEN}${BOLD}Done.${RESET} Run ${BOLD}f00 --help${RESET} or ${BOLD}f00 -la${RESET}.\n" >&2
  if [[ -n "${tui_dst}" ]]; then
    printf "Dual-pane browser: ${BOLD}f00-tui${RESET} (or ${BOLD}f00 --browse${RESET} if built with --features tui).\n" >&2
  fi
  printf "${DIM}Docs: https://f00.sh · https://github.com/${REPO}${RESET}\n" >&2
  printf "\n${BOLD}Using f00 as ls?${RESET} (optional — we never replace /bin/ls by default)\n" >&2
  printf "  Interactive alias (recommended):\n" >&2
  printf "    echo \"alias ls='f00'\" >> ~/.bashrc    # or ~/.zshrc\n" >&2
  printf "    echo \"alias ll='f00 -la'\" >> ~/.bashrc\n" >&2
  printf "  Coreutils-shaped:  alias ls='f00 --gnu'   or   export F00_GNU=1\n" >&2
  printf "  Non-TTY (pipes) is script-safe by default; force modern with --no-gnu.\n" >&2
  if [[ "${F00_INSTALL_LS:-0}" != "1" ]]; then
    printf "  PATH symlink next time:  curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash\n" >&2
  else
    printf "  PATH symlink: enabled (ls -> f00 in %s)\n" "$install_dir" >&2
  fi
  printf "  Guide: ${BOLD}https://f00.sh/#as-ls${RESET}\n\n" >&2
}

main "$@"
