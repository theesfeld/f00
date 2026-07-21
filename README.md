# f00

<p align="left">
  <img src="site/assets/logo.svg" width="96" height="96" alt="f00 logo" />
</p>

[![CI](https://img.shields.io/github/actions/workflow/status/theesfeld/f00/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/theesfeld/f00/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://github.com/theesfeld/f00#license)
[![Release](https://img.shields.io/github/v/release/theesfeld/f00?style=flat-square&include_prereleases&label=release)](https://github.com/theesfeld/f00/releases)
[![crates.io](https://img.shields.io/crates/v/f00?style=flat-square)](https://crates.io/crates/f00)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows%20%7C%20bsd-lightgrey?style=flat-square)](https://f00.sh)

**f00** lists directory contents. The program is written in **Rust**.

Use **`f00 --gnu`** as a drop-in for GNU coreutils **`ls`**. On a TTY, f00 also shows icons, git status, tree view, and JSON. On measured short and long workloads, f00 is faster than **eza** and **lsd**.

| Output | Behavior |
|--------|----------|
| **TTY** | Icons (auto), git (default on), colors (auto) |
| **Non-TTY** (pipe / CI) | Script-safe mode by default (same as `--gnu`) |
| **Force** | Always GNU: `--gnu` or `F00_GNU=1`. Always modern on a pipe: `--no-gnu` or `F00_NO_GNU=1` |

**Website:** [https://f00.sh](https://f00.sh) · **Programs:** `f00`, `f00-tui` · **Version:** v0.12.0 · **Manual:** `man f00`

<!-- agents:status:begin -->
> **Status:** **v0.12.0** · Latest release: [`v0.12.0`](https://github.com/theesfeld/f00/releases/tag/v0.12.0) · 0.x minor versions can include breaking changes · [MIGRATION.md](MIGRATION.md)
<!-- agents:status:end -->

---

## Install

### Install with the installer (recommended)

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

The installer does this:

1. Installs **`f00`** to **`~/.local/bin`** (override with `INSTALL_DIR`).
2. Installs **`f00-tui`** when the release archive includes it (`F00_INSTALL_TUI=0` skips it).
3. Installs **`f00(1)`** to **`~/.local/share/man/man1/f00.1`** (or `$XDG_DATA_HOME/man/man1`) when the archive includes the man page. Override with `MAN_DIR`. Skip with `F00_INSTALL_MAN=0`.
4. Adds the install directory to your shell configuration if it is not on `PATH` (`ADD_PATH=0` skips this).

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.12.0 bash
curl -fsSL https://f00.sh/install.sh | INSTALL_DIR=$HOME/bin bash
man f00
```

**Note for maintainers:** `man/f00.1` must match the live program. CI runs `scripts/check-man-sync.sh`. Update the man page in the same change as CLI or version updates.

### Nix

```bash
nix profile install github:theesfeld/f00
# or: nix run github:theesfeld/f00 -- -la
```

### Other package managers

The primary install paths are **install.sh** and **Nix**. Other channels track GitHub Releases when maintained.

| Channel | Command |
|---------|---------|
| **crates.io** | `cargo install f00 --locked` |
| **Homebrew** | `brew install theesfeld/tap/f00` |
| **AUR** | `yay -S f00` |
| **Scoop / winget / deb / rpm** | See [Releases](https://github.com/theesfeld/f00/releases) |

Use the package manager upgrade command for package installs. Use `f00 --update` for installer installs.

**Default:** f00 does **not** replace system `/bin/ls`. The primary command name is **`f00`**.

### Use f00 as `ls` (optional)

Keep the command name **`f00`** unless you need the name `ls`.

**1. Shell alias (interactive shells)**

```bash
# Modern defaults (icons, git)
echo "alias ls='f00'" >> ~/.bashrc    # or ~/.zshrc
echo "alias ll='f00 -la'" >> ~/.bashrc

# GNU-style output (no icons/git; better for scripts)
# echo "alias ls='f00 --gnu'" >> ~/.bashrc
# or: export F00_GNU=1
```

Aliases apply only in interactive shells. Scripts keep system `/bin/ls` unless they use your shell with aliases.

**2. PATH symlink (installer option)**

```bash
curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash
```

This creates `ls` → `f00` in the install directory. It does **not** overwrite `/bin/ls`.

**3. Binary named `ls`**

If the binary name is `ls` (symlink or rename), TTY defaults stay the same as `f00` (icons, git, colors). Directory-first sort stays off by default (like GNU). For strict coreutils-style output, use `--gnu` or `F00_GNU=1`.

### Update

```bash
f00 --update          # or: f00 update
f00 --check-update    # or: f00 check-update  (exit code 1 if a newer release exists)
```

---

## Features (v0.12.0)

| Area | Status | Notes |
|------|--------|--------|
| **GNU coreutils `ls` options** | Shipped | Full option surface. **`--gnu`** parity tested in CI against system `ls` |
| **Quoting** | Shipped | `-b` `-q` `-Q` `-N` `--quoting-style` and `QUOTING_STYLE` |
| **File name colors** | Shipped | **`LS_COLORS`** (dircolors / `lscolors`) |
| **Long listing colors** | Shipped | Terminal ANSI palette. Optional **`F00_COLORS` / `EZA_COLORS` / `EXA_COLORS`** |
| **Speed** | Shipped | Parallel metadata (rayon), Linux `statx`, optional **io_uring**, `--threads`, `--profile` |
| **Portability** | Shipped | Linux, macOS, Windows, FreeBSD |
| **Git status** | Shipped | Default feature |
| **Icons** | Shipped | Nerd Font glyphs; `--icons[=auto\|always\|never]` (default: auto on TTY) |
| **JSON** | Shipped | Compact `-j` / `--json`. Full metadata: **`--json-full`**. Pretty ANSI colors on TTY. Plain when color is off |
| **CSV / TSV / tree** | Shipped | `--csv`, `--tsv`, `--tree` |
| **TOML config** | Shipped | XDG / AppData paths |
| **Shell completions** | Shipped | `f00 --generate-completions SHELL` |
| **Man page** | Shipped | Tracked **`man/f00.1`**. Installed by `install.sh`. CI checks sync with CLI |
| **TUI browser** | Shipped | Separate binary **`f00-tui`** |
| **Archives** | Opt-in | zip / tar / tar.gz as virtual directories (`--features archives`) |
| **Ignore files** | Shipped | `--ignore-files` (`.gitignore` / `.f00ignore`) |
| **Self-update** | Shipped | `--update` / `--check-update` from GitHub Releases |
| **Plugins** | Opt-in | Feature `plugins` |

---

## Usage

```bash
# Classic listing (TTY: icons, git, colors)
f00 -la

# Pipes are script-safe by default (auto GNU-equivalent)
f00 -la /tmp | grep foo

# Force GNU always / force modern on a pipe
f00 --gnu -lah /tmp
F00_GNU=1 f00 -la
f00 --no-gnu -la | cat

# Quoting / NUL / version sort / width
f00 -bQ -1 .
f00 --zero -1 .
f00 -v -1 .
f00 -w 40 -C .

# Time styles / hide / hyperlink
f00 -l --time-style=long-iso
f00 --hide='*.o' -1
f00 --hyperlink=auto -1

# JSON (compact and full)
f00 --json             # TTY + color: pretty output
f00 -j                 # short form of --json
f00 --json-full        # full metadata fields
f00 --json --color=never | jq '.[].name'   # compact, no ANSI
f00 --csv
f00 --tsv

# Archives (opt-in feature `archives`; auto when path is zip/tar)
f00 project.zip
f00 --archive=false project.zip   # treat as plain file

# Ignore files
f00 --ignore-files

# Interactive dual-pane browser (separate binary)
f00-tui
f00-tui ~/src
# Optional embed: cargo build -p f00 --features tui && f00 --browse

# Icons (auto on TTY; force on/off) — needs a Nerd Font for glyphs
f00 -la --icons              # same as --icons=always
f00 -la --icons=auto
f00 -la --icons=never
f00 -la --icons=always --git
# Special dirs (Desktop/Downloads/Music/…) + file-type icons when icons on

# Speed / profiling
f00 --threads 0 -1 /large/dir   # parallel metadata (default; 0 = auto rayon)
f00 --threads 1 -1 /large/dir   # force serial stats
f00 --threads 8 -1 /large/dir   # fixed rayon pool size
f00 --profile -la /large/dir    # stderr: readdir_ms stat_ms sort_ms format_ms total_ms
f00 --io-uring=false -1 /large  # Linux: disable io_uring batch statx (default: on)
```

Large directories (**>32** entries) parallelize metadata collection with rayon. Sort order is unchanged. Benchmark:

```bash
# Comparative: GNU ls vs eza vs f00 (wall + CPU)
./scripts/bench-compare.sh           # synthetic 2000 files
./scripts/bench-compare.sh 5000
./scripts/bench-compare.sh --dir ~   # real directory
# Uses hyperfine when available; GNU time for user/sys CPU

./scripts/bench-list.sh              # f00-only sequential vs parallel + --profile
cargo bench -p f00-core --bench list_bench   # Criterion microbench
```

### Shell completions

```bash
# bash
f00 --generate-completions bash > ~/.local/share/bash-completion/completions/f00

# zsh (ensure fpath includes the directory)
f00 --generate-completions zsh > ~/.zsh/completions/_f00

# fish
f00 --generate-completions fish > ~/.config/fish/completions/f00.fish

# powershell / elvish
f00 --generate-completions powershell
f00 --generate-completions elvish
```

### Man page

```bash
# View generated man page
f00 --generate-man | man -l -

# Or install the committed page (packagers)
# man/f00.1  →  $(mandir)/man1/f00.1
# Regenerate after CLI changes:
./scripts/gen-man.sh
```

### TUI keys (`f00-tui`)

| Key | Action |
|-----|--------|
| `j`/`k` · arrows | Move (active pane) |
| Enter | Open dir / print file path & quit |
| `h`/`l` · Backspace | Parent / enter |
| `Tab` | Switch active pane |
| `\` / `\|` | Toggle dual-pane layout |
| `c` / `m` / `d` | Copy / move / delete (marked or cursor) → other pane; confirm overlay |
| Space | Mark · `y` print marks & quit (or confirm when overlay open) |
| `/` | Filter · `Esc` clear / cancel confirm |
| `s` / `S` | Cycle sort (name/size/mtime/ext) · reverse |
| `p` | Toggle preview pane (single-pane only) |
| `e` / `v` | Open in `$EDITOR` · view in `$PAGER` |
| `.` | Toggle hidden · `r` refresh · `H` help · `q` quit |

---

## Option groups

| Group | Examples | Notes |
|-------|----------|--------|
| **GNU coreutils** | `-aA -l1Cmx -h --si -Rr -tSXvUf -d -Fp -BI -LH -goGn -is -uc -Z --zero -D --dired --quoting-style --time-style --block-size --author --hyperlink --format --sort --time --group-directories-first --full-time --color` | Full option set. Use **`--gnu`** or non-TTY for script-safe output |
| **Modern TTY** | `--icons` · `--git` · `--color` | Default on a TTY. Off under `--gnu` |
| **f00-only** | `-j` / `--json` · **`--json-full`** · `--tree` · `--csv` / `--tsv` · `--update` · `--browse` / `f00-tui` | Not in GNU `ls` |

With **`--gnu`** or **`F00_GNU=1`**: no icons, no git column, script-safe output.  
If stdout is not a TTY, f00 uses the same mode unless you set **`--no-gnu`** or **`F00_NO_GNU=1`**.

Read the manual: **`man f00`**.---

## Cargo features

| Feature | Default | Description |
|---------|---------|-------------|
| `git` | **yes** | Git status column |
| `io-uring` | **yes** | Linux batch metadata via io_uring (no-op off Linux) |
| `archives` | no | zip/tar virtual directory listing |
| `tui` | no | Embed `f00 --browse` (prefer **`f00-tui`** binary) |
| `plugins` | no | Dynamic plugin host (`--list-plugins`) |
| `full` | no | `git` + `io-uring` + `archives` + `tui` + `plugins` |

```bash
cargo build -p f00 --release                 # default features
cargo build -p f00-tui --release             # dual-pane browser
cargo build -p f00 --no-default-features     # minimal
cargo build -p f00 --features full           # kitchen sink
```

---

## Configuration

Unix: `~/.config/f00/config.toml` (or `$F00_CONFIG` / `--config`)

```toml
[defaults]
all = false
long = false
human = true
icons = "auto"    # auto | always | never  (bool true/false also accepted)
color = "auto"
git = true
dirs_first = true
```

---

## Crates

| Crate | Role |
|-------|------|
| `f00-core` | readdir, sort, filter, ignore files |
| `f00-format` | long/columns/tree/json/csv, quoting, colors |
| `f00-compat` | GNU helpers |
| `f00-git` | git status |
| `f00-archive` | zip/tar virtual listing |
| `f00-tui` | dual-pane browser library + binary `f00-tui` |
| `f00-plugin` | plugin host ABI |
| `f00-plugin-hello` | example cdylib plugin |
| `f00` (path `crates/f00-cli`) | binary `f00` (crates.io) |

Upgrading from 0.10? See **[MIGRATION.md](MIGRATION.md)**.

---

## Building from source

```bash
git clone https://github.com/theesfeld/f00
cd f00
cargo build --release -p f00 -p f00-tui
./target/release/f00 --version
./target/release/f00-tui --version
```

---

## Comparison

| | GNU `ls` | eza | lsd | **f00** |
|--|----------|-----|-----|---------|
| Language | C | Rust | Rust | Rust |
| Full coreutils flags | Native | Partial | Partial | **Shipped** (+ auto non-TTY / `--gnu`) |
| Icons / git | No | Yes | Yes | Yes (TTY) |
| Script-safe pipes | Yes | Partial | Partial | **Yes (default)** |
| Speed (measured) | — | Good | Good | **Beats eza/lsd** |
| TUI | No | No | No | **`f00-tui`** |
| Archives | No | No | No | Opt-in feature |
| Windows | Weak | Strong | Strong | First-class |

---

## License

MIT OR Apache-2.0

## Links

- Issues: https://github.com/theesfeld/f00/issues  
- Design: `docs/superpowers/specs/2026-07-16-f00-design.md`  
- Sync: `docs/SYNC.md`
