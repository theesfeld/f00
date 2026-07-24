# f00tils configuration (XDG)

User configuration is **XDG Base Directory** compliant under **`~/.config/f00/`** (or `$XDG_CONFIG_HOME/f00/`).

There is **no** `~/.f00` user config directory.

## Search order

1. `$XDG_CONFIG_HOME/f00/config` (if `XDG_CONFIG_HOME` is set)
2. `$HOME/.config/f00/config`

Later files override earlier ones. Missing files are ignored.

### Related paths (same tree)

| Path | Role |
|------|------|
| `~/.config/f00/config` | Settings (this file) |
| `~/.config/f00/plugins/` | Optional plugin `.so` files |
| `$F00_PLUGIN_DIR` | Extra plugin directory (env override) |

Environment variables override the file. Command-line flags override everything.

## File format

Simple line-oriented `key = value` (INI-like). Comments start with `#` or `;`.

```ini
# ~/.config/f00/config

# Global defaults (or under [global])
core = false
color = auto
icons = auto
animations = true
spinner = true

[ls]
icons = always
git = true

[cat]
# util-specific keys apply only when argv0 is cat / f00-cat
# (extend as utils honor more keys)

[sha256sum]
# example: quiet chrome for scripts that still want modern color on TTY
spinner = false
```

### Keys (global + util sections)

| Key | Values | Effect |
|-----|--------|--------|
| `core` | `true`/`false`, `yes`/`no`, `1`/`0` | Force `--core` presentation |
| `color` | `auto`, `always`, `never` (also `on`/`off`) | Color when |
| `icons` | `auto`/`nerd` (default on with color), `emoji`, `glyph`, `ascii`, `never` | Nerd File Icons by default; off under `--core` |
| `animations` | bool | Master switch for motion (spinners, …) |
| `spinner` | bool | Per-spinner enable (also needs `animations`) |
| `git` | `auto`/`always`/`never` or bool | ls git decorations |

Unknown keys are ignored (forward compatible).

### Util sections

Section name is the **short util name** (`ls`, `cat`, `sha256sum`), not `f00-ls`.

Bare keys (no section) act as `[global]`.

## Environment overrides

| Env | Maps to |
|-----|---------|
| `F00_CORE` | `core` |
| `F00_COLOR` | `color` |
| `F00_ICONS` | `icons` |
| `F00_ANIMATIONS` | `animations` |
| `F00_SPINNER` | `spinner` |
| `NO_COLOR` | disables color (existing convention) |

Example:

```bash
F00_CORE=1 f00-ls /tmp          # script-safe for one shot
F00_ANIMATIONS=0 f00-sha256sum large.bin
```

## Precedence

```text
defaults → config files → environment → CLI flags
```

CLI always wins (e.g. explicit `--core` or `--icons=always`).

## Implementation

- Loader: `asm/src/ls/config.asm` (`config_load`, `config_apply`)
- Invoked from `suite_runtime_init` for every multicall util (including `ls`)
- Spinners honor `animations` + `spinner` in `suite_ux.asm`
