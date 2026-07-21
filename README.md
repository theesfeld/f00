# f00

<p align="left">
  <img src="site/assets/logo.svg" width="96" height="96" alt="f00 logo" />
</p>

[![CI](https://img.shields.io/github/actions/workflow/status/theesfeld/f00/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/theesfeld/f00/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://github.com/theesfeld/f00#license)
[![Release](https://img.shields.io/github/v/release/theesfeld/f00?style=flat-square&include_prereleases&label=release)](https://github.com/theesfeld/f00/releases)
[![crates.io](https://img.shields.io/crates/v/f00?style=flat-square)](https://crates.io/crates/f00)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows%20%7C%20bsd-lightgrey?style=flat-square)](https://f00.sh)

**f00** is a fully compliant **`ls`** utility written entirely in **Rust**. It is a drop-in for GNU coreutils `ls`, and it meets and exceeds **eza** and **lsd** for modern listings (icons, git, tree, rich **JSON**) without giving up coreutils flag coverage.

| Surface | Behavior |
|---------|----------|
| **TTY** | Icons (auto), git (default), colors (auto) |
| **Non-TTY** (pipes/CI) | **Script-safe by default** (GNU-equivalent; same as `--gnu`) |
| **Force** | `--gnu` / `F00_GNU=1` · modern on pipes: `--no-gnu` / `F00_NO_GNU=1` |

**Website:** [https://f00.sh](https://f00.sh) · **Binaries:** `f00` · `f00-tui` · **Latest:** v0.11.0

<!-- agents:status:begin -->
> **Status:** v0.11.0 focus cut shipped · Phase: [#88](https://github.com/theesfeld/f00/issues/88) · Latest: `v0.11.0` · 0.x minors may include breaking changes · [MIGRATION.md](MIGRATION.md)
<!-- agents:status:end -->

---

## Install

### Quick install (recommended)

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

Installs **`f00`** (and **`f00-tui`** when present in the release) to **`~/.local/bin`** by default (`INSTALL_DIR` to override). Adds that dir to your shell rc when missing from `PATH` (`ADD_PATH=0` to skip). `F00_INSTALL_TUI=0` skips the browser binary.

Also installs the man page **`f00(1)`** to **`~/.local/share/man/man1/f00.1`** (or `$XDG_DATA_HOME/man/man1`) when the release archive includes it. Override with `MAN_DIR=…`; skip with `F00_INSTALL_MAN=0`. Then: `man f00`.

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.11.0 bash
curl -fsSL https://f00.sh/install.sh | INSTALL_DIR=$HOME/bin bash
```
### Nix

```bash
nix profile install github:theesfeld/f00
# or: nix run github:theesfeld/f00 -- -la
```

### Other package managers (community / convenience)

These track GitHub Releases when maintained; **install.sh and Nix are the first-class paths**. Prefer the package manager’s upgrade command when you use one; use `f00 --update` for installs from `install.sh`.

| Channel | Command |
|---------|---------|
| **crates.io** | `cargo install f00 --locked` |
| **Homebrew** | `brew install theesfeld/tap/f00` |
| **AUR** | `yay -S f00` |
| **Scoop / winget / deb / rpm** | See [Releases](https://github.com/theesfeld/f00/releases) |

**We never replace system `/bin/ls` by default.** The primary command is always `f00`.

### Using f00 as `ls`

Most people should keep typing **`f00`**. If you want muscle memory for `ls`, pick one opt-in path:

**1. Shell alias (recommended for interactive use)**

```bash
# modern defaults (icons, git, …)
echo "alias ls='f00'" >> ~/.bashrc    # or ~/.zshrc
echo "alias ll='f00 -la'" >> ~/.bashrc

# coreutils-shaped (no icons/git; better for scripts)
# echo "alias ls='f00 --gnu'" >> ~/.bashrc
# or: export F00_GNU=1
```

Aliases only affect interactive shells. Non-interactive scripts keep using `/bin/ls` unless they call your alias-enabled shell.

**2. Optional PATH symlink (installer opt-in)**

```bash
curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash
```

Creates `…/bin/ls` → `f00` next to the binary. Anything that finds `ls` on your `PATH` (before `/bin`) will run f00. Does **not** overwrite `/bin/ls`.

**3. Soft drop-in when the binary is named `ls`**

If you symlink or rename so argv0 is `ls`, f00 keeps full TTY defaults (icons, git, modern colors) — same as `f00`. Dirs-first stays off by default (like GNU). **Strict plain coreutils** still needs `--gnu` or `F00_GNU=1`.

More detail: [f00.sh#as-ls](https://f00.sh/#as-ls)

### Update

```bash
f00 --update          # or: f00 update
f00 --check-update    # or: f00 check-update  (exit 1 if behind)
```

---

## Features

| Area | Status | Notes |
|------|--------|--------|
| **GNU coreutils `ls`** | Shipped | Flag surface + **`--gnu` behavior parity** (CI tests vs system `ls`) |
| **Quoting** | Shipped | `-b` `-q` `-Q` `-N` `--quoting-style` + `QUOTING_STYLE` |
| **LS_COLORS** | Shipped | Via `lscolors` / env; **dotfiles** dimmed (darker grey) when color is on |

| **Speed** | Shipped | Parallel `stat` (rayon), cheap short path, uid cache, Linux `statx` + **io_uring** batch, `--threads`, `--profile` |
| **Portability** | Shipped | Linux, macOS, Windows, FreeBSD |
| **Git status** | Shipped | Default feature |
| **Icons** | Shipped | Nerd Font glyphs (eza-style special dirs + file types); `--icons[=auto\|always\|never]` (default: auto on TTY) |
| **JSON** | **Core** | Rich metadata (`--json` / `-j`); **pretty + colored** when color is on; compact plain when color is off (pipes) |
| **CSV / TSV / tree** | Shipped | Machine formats + tree view |
| **TOML config** | Shipped | XDG / AppData |
| **Shell completions** | Shipped | `f00 --generate-completions SHELL` |
| **Man page** | Shipped | `f00 --generate-man` · committed `man/f00.1` |
| **TUI browser** | Shipped | Separate **`f00-tui`** binary — dual-pane FM, syntax preview, `$EDITOR`/`$PAGER` |
| **Archives** | Opt-in feature | zip / tar / tar.gz as virtual dirs (`--features archives`) |
| **Ignore files** | Shipped | `--ignore-files` → `.gitignore` / `.f00ignore` |
| **Self-update** | Shipped | `--update` / `--check-update` via GitHub Releases |
| **Plugins** | Opt-in feature | Feature `plugins` · ABI v1 · decorate hooks |

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

# JSON is a core surface (rich metadata: inode, times, owner, permissions, …)
f00 --json             # TTY + color: pretty, syntax-colored
f00 -j                 # short for --json (not used by GNU ls)
f00 --json --color=always | less -R
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

## GNU surface (highlights)

`-aA` `-l1Cmx` `-h` `--si` `-Rr` `-tSXvUf` `-d` `-Fp` `--file-type` `-BI` `--hide` `-LH` `-goGn` `-is` `-uc` `-vw` `-Z` `--zero` `-D` `--dired` `-bQNq` `--quoting-style` `--time-style` `--block-size` `--author` `--hyperlink` `--indicator-style` `--format` `--sort` `--time` `--group-directories-first` `--full-time` `--color` **`--gnu`**

Strict `--gnu` / `F00_GNU=1`: no icons/git decorations, classic sort, script-safe.  
**Non-TTY stdout auto-enables the same mode** unless `--no-gnu` / `F00_NO_GNU=1`.

---

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
