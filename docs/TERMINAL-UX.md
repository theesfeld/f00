# Terminal UX conventions (f00tils)

Suite-wide contract for modern chrome, icons, help text, semantic color, `--core`, and machine-readable JSON.
Product name **f00tils**; binary `f00` / tools `f00-*`.

**Law:** when **not** `--core`, interactive TTY output is **modern** — color, icons where they help, structured chrome, never a plain GNU clone.

Implementation anchors:

- Color helpers: `asm/src/ls/util.asm` (`color_path`, `color_num`, …)
- Help + chrome: `asm/src/ls/suite_ux.asm` (`ui_help_*`, `ui_file_header`, `ui_spinner_*`, bars)
- Icons: `asm/src/ls/icons.asm` (`icon_enabled`, `icon_for_entry`, `icon_for_path`)
- Buffered I/O: `out_init` / `out_*` / `out_flush`
- Dual-pane browser: `asm/src/ls/tui.asm` (`f00-ls --browse` / `--tui`)

---

## Modern vs `--core`

| Surface | Modern (default on TTY) | `--core` |
|---------|-------------------------|----------|
| Color | Semantic SGR | Never |
| Icons | Nerd Font when useful (paths, listings) | Never |
| Headers / gutters | Bat-class file chrome (`ui_file_header`) | Plain GNU style |
| Spinners | stderr braille spinner for long work | Never |
| JSON/CSV | Available; `mode: "modern"` | Available; `mode: "core"` |
| Flags / exit codes | GNU-compatible + modern extras | Track GNU presentation |

Honor **`NO_COLOR`**, non-TTY pipes (no SGR / no icons auto), and **`--core`**.

User defaults: XDG config under `~/.config/f00/config` (see [CONFIG.md](CONFIG.md)) — e.g. `core=true`, `animations=false`, `icons=always`, per-util sections like `[ls]`.

---

## Icons (suite-wide)

- **Default style: emoji / standard Unicode** — no Nerd Font pack required.
- **CLI (ls):** `--icons[=STYLE]` where `STYLE` is:
  - `auto` — on modern TTY, **emoji** style (default)
  - `emoji` — force on, emoji/Unicode table
  - `nerd` — force on, Nerd Font PUA (opt-in for people with that font)
  - `ascii` — force on, `[D]`/`[F]`/… (always works)
  - `never` — off
  - bare `--icons` — force on, keep current style (default emoji)
- **Config / env:** `icons = auto|emoji|nerd|ascii|never` · `F00_ICONS=…`
- **API:**
  - `icon_for_entry(Entry*)` — ls / tree
  - `icon_for_path(path)` — cat headers, hash paths, any path print
  - `icon_set_style_from_str` — parse style string into `g_icons_when` + `g_icons_style`
- **Where it belongs:** listings, multi-file headers, digest lines — not machine-only streams.

---

## Chromed text tools (cat / head / tail / less-like)

Standard modern frame for multi-file text tools:

1. **File banner** — `ui_file_header(path)`: dim rule + optional icon + cyan path.
2. **Gutter** — dim box-drawing bar for line-numbered views (`│ `).
3. **Markers** — colored `^I` / `$` / non-printing under `-vET` (cat already).
4. **No chrome** under `--core` or non-TTY (unless forced flags).

`f00-cat` ships bat-class multi-file headers + gutters. Other text utils should reuse the same helpers as they deepen modern mode.

There is no full `less` pager binary yet; interactive browse is `f00-ls --browse`. Pager-class chrome (status line, less keys) is a later util or opt-in mode — same token system.

---

## Spinners / long work

- **API:** `ui_spinner_start(label)` · `ui_spinner_tick` · `ui_spinner_stop`
- **Where:** stderr only; modern TTY; no-ops under `--core` / non-TTY / `g_color=0`
- **Style:** colored braille frames + label; CR + erase line on stop
- **Use:** multi-file hash, large copies, deep walks — anything that can pause a human. Fast paths stay silent (no spinner spam).

Hash suite already starts/stops a spinner around each file.

---

## Color tokens

Semantic SGR tokens (modern TTY only). Prefer helpers over raw escapes.

| Token        | Role                         | Typical SGR     | Helper        |
|--------------|------------------------------|-----------------|---------------|
| **path**     | Paths, filenames of interest | bold cyan `1;36`| `color_path`  |
| **nums**     | Counts, sizes, ids, digests  | bold yellow `1;33` | `color_num` |
| **ok**       | Success / affirmative        | bold green `1;32`  | `color_ok`  |
| **err**      | Errors / danger              | bold red `1;31`    | `color_err` |
| **sections** | Help section headers         | bold magenta `1;35`| section chrome |
| **hdr**      | Secondary headers / labels   | bold blue `1;34`   | `color_hdr` |
| **dim**      | Chrome, hints, separators    | dim `2`            | `color_dim` |
| **reset**    | Always pair with set         | `0`                | `color_reset` |

Rules:

1. **Content bright, chrome dim** — paths/numbers/names stand out; rules, key hints, and separators stay muted.
2. **Always reset** after a colored span (`color_reset` / `ESC[0m`).
3. **Selection (TUI)** uses reverse video (`ESC[7m` … `ESC[0m`), not a permanent color.
4. Honor **`NO_COLOR`** (any value) and non-TTY: no SGR.
5. Severity bars (e.g. disk use): green ≤70%, yellow ≤90%, red >90% (`suite_ux.asm`).

---

## `--core` never emits SGR

`--core` is the GNU coreutils-compatible presentation mode:

- No color, icons, git chrome, hyperlinks, or decorative Unicode bars.
- Byte-oriented parity targets (`asm/benches/parity.sh`) compare against system coreutils.
- JSON may still be requested with `--json --core`; envelope `mode` is `"core"`.
- Color helpers are gated on `g_color`; under `--core` keep `g_color = 0`.

```text
# ok: plain
f00-id --core
# modern TTY may color numbers; --core must not
f00-ls --core -la /tmp
```

---

## Help structure

Every util’s `--help` should follow this section order (blank line between major blocks):

1. **Usage** — `Usage: f00-<util> [OPTION]... …`
2. **Description** — one short paragraph (optional if the util is trivial)
3. **Coreutils flags** — GNU-compatible short/long options
4. **Modern flags** — f00 extensions (`--core`, `--json`, …)
5. **Examples** — 1–3 realistic invocations
6. **Footer** — `f00 suite · pure assembly · MIT · https://f00.sh`

Section titles (exact spelling when using chrome helpers):

- `Coreutils flags:`
- `Modern flags:`
- `Examples:`

Version lines (`--version`) stay plain single-block text (name + version + license/url as already shipped). Do not SGR-decorate `--version` under `--core` (and generally keep version monochrome).

### TUI / `--browse`

Status chrome (top):

- Title (dim) · **path** (cyan) · **count** (yellow) · selection index
- Key help line (dim): `j/k or arrows  enter  …  q`
- Confirm prompts: reverse video
- List: reverse-video selection; dim mark column; bold names
- Redraw via `out_*` buffer + one `out_flush` per frame (no flicker)

---

## JSON envelope (`f00/v1`)

Modern `--json` uses a pretty, 2-space-indented object. Shared open/close helpers:

`json_meta_open` / `json_meta_close` in `util.asm`.

### Top-level fields

| Field         | Type    | Notes                                      |
|---------------|---------|--------------------------------------------|
| `schema`      | string  | Always `"f00/v1"`                          |
| `suite`       | string  | `"f00"`                                    |
| `version`     | string  | Suite version (e.g. `"0.14.0"`)            |
| `util`        | string  | Utility basename (`"id"`, `"md5sum"`, …)   |
| `mode`        | string  | `"modern"` or `"core"`                     |
| `color`       | bool    | Whether SGR would be used on this run      |
| `tty`         | bool    | stdout is a TTY                            |
| `platform`    | object  | `os`, `arch`, `bits`                       |
| `invocation`  | object  | See below                                  |
| `result`      | object  | Util-specific payload                      |
| `exit`        | number  | Process exit code                          |
| `ok`          | bool    | `exit == 0`                                |

### `platform`

```json
"platform": {
  "os": "linux",
  "arch": "x86_64",
  "bits": 64
}
```

### `invocation`

| Field       | Type   | Notes                          |
|-------------|--------|--------------------------------|
| `argc`      | number |                                |
| `argv0`     | string | Raw argv[0]                    |
| `util_name` | string | Resolved multicall name        |
| `pid`       | number |                                |
| `uid`       | number |                                |
| `euid`      | number |                                |
| `cwd`       | string |                                |
| `epoch_sec` | number | Wall clock at start            |

### `result`

Util-defined. Examples:

- Hashes: digests, paths, algorithm
- `id` / `uname`: identity fields
- Text tools: counts, lines, status

Callers emit result keys with `json_key_str` / `json_key_u64` / `json_key_bool`, then `json_meta_close`.

### Rules

1. **Stable schema id** — bump only with intentional compatibility breaks (`f00/v2`, …).
2. **No SGR inside JSON** — even on a TTY.
3. **`--core --json`** still uses the envelope; set `"mode": "core"` and keep result fields close to GNU semantics where applicable.
4. **Pretty print** for humans; do not minify by default in ASM port.

---

## Output buffering

- Accumulate with `out_byte` / `out_str` / `out_u64` into `g_out_buf` (256 KiB).
- `out_flush` once per logical unit (help page, result, TUI frame).
- Avoid per-character `write(1)` — that is the main interactive flicker/speed footgun.

---

## Checklist for new utils

- [ ] Help sections in the order above
- [ ] Footer line present
- [ ] `--core` path emits zero SGR
- [ ] Modern TTY uses path/num/ok/err tokens consistently
- [ ] `--json` goes through `json_meta_*` when practical
- [ ] Buffered writes; flush at end
- [ ] Smoke + parity case if GNU-comparable
