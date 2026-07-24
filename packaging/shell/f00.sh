# f00tils — default coreutils replacement via PATH
# Bare names (ls, cat, …) live in /usr/lib/f00/bin → f00 multicall.
# They never overwrite /usr/bin/cat from the coreutils package.
#
# Default: ON (prepend supersede dir). Toggle in XDG config:
#   replace = false
# or:
#   f00-config replace off
#   f00-config replace on
#
# Requires a new shell (or: source /etc/profile.d/f00.sh).

_f00_libbin="/usr/lib/f00/bin"

_f00_replace_enabled() {
  # default ON when config missing / no explicit false
  local cfg="${XDG_CONFIG_HOME:-${HOME}/.config}/f00/config"
  [ -n "${HOME:-}" ] || return 0
  [ -f "$cfg" ] || return 0
  if grep -Eiq '^[[:space:]]*replace[[:space:]]*=[[:space:]]*(false|no|0|none)([[:space:]]|#|$)' "$cfg" 2>/dev/null; then
    return 1
  fi
  return 0
}

if [ -d "$_f00_libbin" ] && _f00_replace_enabled; then
  case ":${PATH:-}:" in
    *":${_f00_libbin}:"*) ;;
    *) PATH="${_f00_libbin}${PATH:+:}${PATH:-}"; export PATH ;;
  esac
fi

unset _f00_libbin
unset -f _f00_replace_enabled 2>/dev/null || true
