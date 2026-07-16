#!/usr/bin/env bash
# Comparative wall-time + CPU bench: GNU ls vs eza vs f00.
#
# Usage:
#   ./scripts/bench-compare.sh              # synthetic 2000 files
#   ./scripts/bench-compare.sh 5000
#   ./scripts/bench-compare.sh --dir /path  # real directory (no temp)
#   F00_BIN=./target/release/f00 ./scripts/bench-compare.sh
#
# Optional tools:
#   hyperfine  — multi-run wall stats (preferred)
#   GNU time   — user/sys CPU seconds
#
# Fairness notes (printed in output):
#   - Short listing: names only (no long metadata columns).
#   - Long listing: -l style; colors forced off for all tools.
#   - Icons/git: only eza + f00 (ls has none); separate scenario.
#   - f00 default may enable git/icons on TTY; we pin flags per scenario.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

N=5000
REAL_DIR=""
WARMUP=3
RUNS=25

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dir)
      REAL_DIR="${2:?--dir needs path}"
      shift 2
      ;;
    --runs)
      RUNS="${2:?}"
      shift 2
      ;;
    --warmup)
      WARMUP="${2:?}"
      shift 2
      ;;
    -h|--help)
      sed -n '2,20p' "$0"
      exit 0
      ;;
    *)
      if [[ "$1" =~ ^[0-9]+$ ]]; then
        N="$1"
        shift
      else
        echo "unknown arg: $1" >&2
        exit 2
      fi
      ;;
  esac
done

find_bin() {
  local name="$1"
  shift
  local c
  for c in "$@"; do
    if [[ -n "$c" && -x "$c" ]]; then
      printf '%s\n' "$c"
      return 0
    fi
  done
  if command -v "$name" >/dev/null 2>&1; then
    command -v "$name"
    return 0
  fi
  return 1
}

resolve_time() {
  local candidates=(
    /run/current-system/sw/bin/time
    /usr/bin/time
    /bin/time
  )
  local t
  for t in "${candidates[@]}"; do
    if [[ -x "$t" ]] && "$t" --version 2>&1 | head -1 | grep -qi 'GNU'; then
      printf '%s\n' "$t"
      return 0
    fi
  done
  if command -v gtime >/dev/null 2>&1; then
    command -v gtime
    return 0
  fi
  return 1
}

resolve_hyperfine() {
  if command -v hyperfine >/dev/null 2>&1; then
    command -v hyperfine
    return 0
  fi
  local f
  # shellcheck disable=SC2012
  f="$(ls -1 /nix/store/*-hyperfine-*/bin/hyperfine 2>/dev/null | head -1 || true)"
  if [[ -n "$f" && -x "$f" ]]; then
    printf '%s\n' "$f"
    return 0
  fi
  return 1
}

LS_BIN="$(find_bin ls /run/current-system/sw/bin/ls /bin/ls /usr/bin/ls || true)"
EZA_BIN="$(find_bin eza /etc/profiles/per-user/"${USER:-}/bin/eza" || true)"
F00_BIN="${F00_BIN:-}"
if [[ -z "$F00_BIN" ]]; then
  F00_BIN="$(find_bin f00 \
    "$ROOT/target/release/f00" \
    "${HOME:-}/.local/bin/f00" \
    "${HOME:-}/.cargo/bin/f00" || true)"
fi

TIME_BIN="$(resolve_time || true)"
HYPERFINE="$(resolve_hyperfine || true)"

echo "=== f00 comparative bench (ls / eza / f00) ==="
echo "date:      $(date -u +%Y-%m-%dT%H:%MZ)"
echo "host:      $(uname -srm)"
echo "ls:        ${LS_BIN:-MISSING}"
echo "eza:       ${EZA_BIN:-MISSING}"
echo "f00:       ${F00_BIN:-MISSING}"
echo "time:      ${TIME_BIN:-MISSING}"
echo "hyperfine: ${HYPERFINE:-MISSING}"
echo "runs:      $RUNS  warmup: $WARMUP"
echo

if [[ -z "$LS_BIN" || -z "$F00_BIN" ]]; then
  echo "error: need at least ls and f00" >&2
  exit 1
fi

if [[ "$F00_BIN" == "$ROOT/target/release/f00" && ! -x "$F00_BIN" ]]; then
  echo "Building release f00..."
  cargo build -q -p f00 --release
fi

cleanup() {
  if [[ -n "${TMP_DIR:-}" && -d "${TMP_DIR:-}" ]]; then
    rm -rf "$TMP_DIR"
  fi
}
trap cleanup EXIT

if [[ -n "$REAL_DIR" ]]; then
  DIR="$(cd "$REAL_DIR" && pwd)"
  ENTRY_COUNT="$(find "$DIR" -mindepth 1 -maxdepth 1 2>/dev/null | wc -l | tr -d ' ')"
  echo "target:    $DIR  (existing, ~$ENTRY_COUNT entries)"
else
  TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/f00-cmp-XXXXXX")"
  DIR="$TMP_DIR"
  echo "Creating $N synthetic files in $DIR ..."
  (
    cd "$DIR"
    i=1
    while [[ $i -le $N ]]; do
      : >"file_$(printf '%05d' "$i").txt"
      i=$((i + 1))
    done
  )
  mkdir -p "$DIR/Desktop" "$DIR/Downloads" "$DIR/Music" "$DIR/Pictures" "$DIR/Videos"
  : >"$DIR/main.rs"
  : >"$DIR/Cargo.toml"
  : >"$DIR/readme.md"
  ENTRY_COUNT="$(find "$DIR" -mindepth 1 -maxdepth 1 | wc -l | tr -d ' ')"
  echo "target:    $DIR  ($ENTRY_COUNT entries)"
fi
echo

# Scenario: name, then space-separated argv for each tool (no shell).
# We store as bash arrays of argv; empty first element means skip.
# Format via parallel arrays of "shell-safe" command strings used only for display
# and for hyperfine (which runs via shell).

declare -a NAMES=()
declare -a LS_CMDS=()
declare -a EZA_CMDS=()
declare -a F00_CMDS=()

add_scenario() {
  NAMES+=("$1")
  LS_CMDS+=("$2")
  EZA_CMDS+=("$3")
  F00_CMDS+=("$4")
}

# Short names
add_scenario \
  "short (-1 names)" \
  "$LS_BIN -1 --color=never" \
  "${EZA_BIN:+$EZA_BIN -1 --color=never --icons=never --no-git}" \
  "$F00_BIN -1 --color=never --icons=never --git=false --gnu"

# Long, plain
add_scenario \
  "long (-l no color)" \
  "$LS_BIN -l --color=never" \
  "${EZA_BIN:+$EZA_BIN -l --color=never --icons=never --no-git}" \
  "$F00_BIN -l --color=never --icons=never --git=false --gnu"

# Product-ish UX (eza + f00)
add_scenario \
  "long + icons (+git)" \
  "" \
  "${EZA_BIN:+$EZA_BIN -l --color=never --icons=always --git}" \
  "$F00_BIN -l --color=never --icons=always --git=true"

# Almost-all short
add_scenario \
  "almost-all short (-A -1)" \
  "$LS_BIN -A -1 --color=never" \
  "${EZA_BIN:+$EZA_BIN -a -1 --color=never --icons=never --no-git}" \
  "$F00_BIN -A -1 --color=never --icons=never --git=false --gnu"

# Average wall/user/sys over CPU_SAMPLES runs using GNU time -o (clean capture).
CPU_SAMPLES=5

run_cpu() {
  local label="$1"
  local base_cmd="$2"
  if [[ -z "$base_cmd" ]]; then
    printf '  %-8s  (skipped)\n' "$label"
    return 0
  fi

  local shell_cmd
  shell_cmd="$base_cmd $(printf %q "$DIR") >/dev/null"

  if [[ -z "$TIME_BIN" ]]; then
    local start end
    start=$(date +%s%N)
    bash -c "$shell_cmd"
    end=$(date +%s%N)
    awk -v l="$label" -v s="$start" -v e="$end" \
      'BEGIN{printf "  %-8s  wall=%.4fs  (no GNU time for CPU)\n", l, (e-s)/1e9}'
    return 0
  fi

  local stats_file sum_e=0 sum_u=0 sum_s=0 ok=0 i wall user sys
  stats_file="$(mktemp)"
  for ((i = 0; i < CPU_SAMPLES; i++)); do
    if "$TIME_BIN" -o "$stats_file" -f '%e %U %S' bash -c "$shell_cmd" 2>/dev/null; then
      # shellcheck disable=SC2034
      read -r wall user sys <"$stats_file" || true
      sum_e="$(awk -v a="$sum_e" -v b="$wall" 'BEGIN{printf "%.6f", a+b}')"
      sum_u="$(awk -v a="$sum_u" -v b="$user" 'BEGIN{printf "%.6f", a+b}')"
      sum_s="$(awk -v a="$sum_s" -v b="$sys" 'BEGIN{printf "%.6f", a+b}')"
      ok=$((ok + 1))
    fi
  done
  rm -f "$stats_file"

  if [[ "$ok" -eq 0 ]]; then
    printf '  %-8s  FAILED\n' "$label"
    return 0
  fi
  awk -v l="$label" -v n="$ok" -v e="$sum_e" -v u="$sum_u" -v s="$sum_s" \
    'BEGIN{
      e/=n; u/=n; s/=n;
      printf "  %-8s  wall=%8.4fs  user=%8.4fs  sys=%8.4fs  cpu=%8.4fs  (avg of %d)\n",
        l, e, u, s, u+s, n
    }'
}

run_hyperfine() {
  local ls_cmd="$1"
  local eza_cmd="$2"
  local f00_cmd="$3"

  if [[ -z "$HYPERFINE" ]]; then
    echo "  hyperfine: not found (install hyperfine for multi-run wall stats)"
    return 0
  fi

  # Each -n NAME COMMAND is two args; COMMAND is one shell string.
  # Use bash -c so redirects work; pass DIR via env.
  # Prefer shell=none (no shell overhead) + --output=null (discard tool stdout).
  # Commands are split into argv: tool flag… DIR
  local -a hf=(
    "$HYPERFINE"
    --shell=none
    --output=null
    --warmup "$WARMUP"
    --runs "$RUNS"
    --style full
  )

  # hyperfine with shell=none: pass program + args as one string that it splits?
  # Actually with --shell=none, the command string is split on spaces — so paths
  # with spaces break. Our DIR is under /tmp without spaces.
  if [[ -n "$ls_cmd" ]]; then
    hf+=(-n ls "$ls_cmd $DIR")
  fi
  if [[ -n "$eza_cmd" ]]; then
    hf+=(-n eza "$eza_cmd $DIR")
  fi
  if [[ -n "$f00_cmd" ]]; then
    hf+=(-n f00 "$f00_cmd $DIR")
  fi

  echo "  hyperfine (wall + user/sys CPU, $RUNS runs, $WARMUP warmup, shell=none):"
  set +e
  "${hf[@]}" 2>&1 | sed 's/^/    /'
  set -e
}

i=0
while [[ $i -lt ${#NAMES[@]} ]]; do
  name="${NAMES[$i]}"
  ls_cmd="${LS_CMDS[$i]}"
  eza_cmd="${EZA_CMDS[$i]}"
  f00_cmd="${F00_CMDS[$i]}"

  echo "────────────────────────────────────────────────────────"
  echo "scenario: $name"
  [[ -n "$ls_cmd" ]]  && echo "  cmd ls:  $ls_cmd <dir>"
  [[ -n "$eza_cmd" ]] && echo "  cmd eza: $eza_cmd <dir>"
  [[ -n "$f00_cmd" ]] && echo "  cmd f00: $f00_cmd <dir>"
  echo
  echo "  CPU via GNU time (avg of $CPU_SAMPLES; user+sys):"
  # Warm once so first sample is not pure cold-start for dynlink.
  if [[ -n "$f00_cmd" ]]; then
    # shellcheck disable=SC2086
    $f00_cmd "$DIR" >/dev/null 2>/dev/null || true
  fi
  run_cpu "ls"  "$ls_cmd"
  run_cpu "eza" "$eza_cmd"
  run_cpu "f00" "$f00_cmd"
  echo
  run_hyperfine "$ls_cmd" "$eza_cmd" "$f00_cmd"
  echo
  i=$((i + 1))
done

echo "────────────────────────────────────────────────────────"
echo "Notes"
echo "  • Prefer hyperfine numbers: wall mean ± σ and [User / System] CPU."
echo "  • GNU time often has ~10ms resolution — coarse for small dirs; use it as a cross-check."
echo "  • Lower wall is better. cpu = user+sys (can exceed wall if multi-threaded)."
echo "  • f00 --gnu matches coreutils-shaped work (no icons/git)."
echo "  • Modern UX scenario is eza vs f00 only (ls has no icons/git)."
echo "  • Results vary with FS cache, disk, and CPU governor."
echo "  • f00-only parallel smoke: ./scripts/bench-list.sh"
echo "  • Criterion: cargo bench -p f00-core --bench list_bench"
echo "Done."
