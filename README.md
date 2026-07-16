# f00

[![CI](https://img.shields.io/github/actions/workflow/status/theesfeld/f00/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/theesfeld/f00/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://github.com/theesfeld/f00#license)
[![Release](https://img.shields.io/github/v/release/theesfeld/f00?style=flat-square&include_prereleases&label=release)](https://github.com/theesfeld/f00/releases)
[![crates.io](https://img.shields.io/crates/v/f00?style=flat-square)](https://crates.io/crates/f00)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows%20%7C%20bsd-lightgrey?style=flat-square)](https://f00.sh)

**f00** — a next-generation, cross-platform **coreutils `ls` clone** in Rust, with modern UX and a supertool layer.

**Website:** [https://f00.sh](https://f00.sh) · **Binary:** `f00` · **Latest:** v0.4.0

<!-- agents:status:begin -->
> **Status:** v0.4 packaging + self-update + plugin ABI · Phase: [#36](https://github.com/theesfeld/f00/issues/36) · Latest: `v0.4.0` · 0.x minors may include breaking changes
<!-- agents:status:end -->

---

## Install

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

Installs to **`~/.f00/bin`** by default (override with `INSTALL_DIR`). Add to `PATH` if needed:

```bash
export PATH="$HOME/.f00/bin:$PATH"
```

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.4.0 bash
curl -fsSL https://f00.sh/install.sh | INSTALL_DIR=$HOME/bin bash
curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash   # optional ls symlink
```

### Update

```bash
f00 --update          # or: f00 update
f00 --check-update    # or: f00 check-update  (exit 1 if behind)
```

---

## Features

| Area | Status | Notes |
|------|--------|--------|
| **GNU coreutils `ls`** | Shipped | Full flag surface + `--gnu` strict mode |
| **Quoting** | Shipped | `-b` `-q` `-Q` `-N` `--quoting-style` + `QUOTING_STYLE` |
| **LS_COLORS** | Shipped | Via `lscolors` / env |
| **Speed** | Shipped | Parallel `stat` (rayon), `--threads`, `--profile` |
| **Portability** | Shipped | Linux, macOS, Windows, FreeBSD |
| **Git status** | Shipped | Default feature |
| **Icons** | Shipped | `--icons[=auto\|always\|never]` (default: auto on TTY) |
| **JSON / CSV / TSV / tree** | Shipped | Machine formats |
| **TOML config** | Shipped | XDG / AppData |
| **Shell completions** | Shipped | `f00 --generate-completions SHELL` |
| **Man page** | Shipped | `f00 --generate-man` · committed `man/f00.1` |
| **TUI browser** | Shipped | `f00 --browse` (feature `tui`, default on) |
| **Archives** | Shipped | zip / tar / tar.gz as virtual dirs |
| **Ignore files** | Shipped | `--ignore-files` → `.gitignore` / `.f00ignore` |
| **Self-update** | Shipped | `--update` / `--check-update` via GitHub Releases |
| **Plugins** | Scaffold | Feature `plugins` · ABI v1 · #27 |

---

## Usage

```bash
# Classic listing
f00 -la

# Drop-in GNU shape (scripts)
f00 --gnu -lah /tmp
F00_GNU=1 f00 -la

# Quoting / NUL / version sort / width
f00 -bQ -1 .
f00 --zero -1 .
f00 -v -1 .
f00 -w 40 -C .

# Time styles / hide / hyperlink
f00 -l --time-style=long-iso
f00 --hide='*.o' -1
f00 --hyperlink=auto -1

# Machine output
f00 --json
f00 --csv
f00 --tsv

# Archives (auto when path is zip/tar)
f00 project.zip
f00 --archive=false project.zip   # treat as plain file

# Ignore files
f00 --ignore-files

# Interactive browser
f00 --browse
f00 --tui ~/src

# Icons (auto on TTY; force on/off)
f00 -la --icons              # same as --icons=always
f00 -la --icons=auto
f00 -la --icons=never
f00 -la --icons=always --git

# Speed / profiling
f00 --threads 0 -1 /large/dir   # parallel metadata (default; 0 = auto rayon)
f00 --threads 1 -1 /large/dir   # force serial stats
f00 --threads 8 -1 /large/dir   # fixed rayon pool size
f00 --profile -la /large/dir    # stderr: readdir_ms stat_ms sort_ms format_ms total_ms
```

Large directories (**>32** entries) parallelize metadata collection with rayon. Sort order is unchanged. Benchmark:

```bash
cargo bench -p f00-core --bench list_bench
./scripts/bench-list.sh          # quick wall-clock + --profile
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

### TUI keys (`--browse`)

| Key | Action |
|-----|--------|
| `j`/`k` · arrows | Move |
| Enter | Open dir / print file path & quit |
| `h`/`l` · Backspace | Parent / enter |
| Space | Mark · `y` print marks & quit |
| `/` | Filter · `Esc` clear |
| `.` | Toggle hidden · `r` refresh · `H` help · `q` quit |

---

## GNU surface (highlights)

`-aA` `-l1Cmx` `-h` `--si` `-Rr` `-tSXvUf` `-d` `-Fp` `--file-type` `-BI` `--hide` `-LH` `-goGn` `-is` `-uc` `-vw` `-Z` `--zero` `-D` `--dired` `-bQNq` `--quoting-style` `--time-style` `--block-size` `--author` `--hyperlink` `--indicator-style` `--format` `--sort` `--time` `--group-directories-first` `--full-time` `--color` **`--gnu`**

Strict `--gnu` / `F00_GNU=1`: no icons/git decorations, classic sort, script-safe.

---

## Cargo features

| Feature | Default | Description |
|---------|---------|-------------|
| `git` | yes | Git status column |
| `archives` | yes | zip/tar listing |
| `tui` | yes | `--browse` / `--tui` |
| `plugins` | no | Dynamic plugin host (`--list-plugins`) |

```bash
cargo build -p f00-cli --release
cargo build -p f00-cli --no-default-features   # minimal
cargo build -p f00-cli --features "git,archives,tui,plugins"
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
| `f00-tui` | interactive browser |
| `f00-plugin` | plugin host ABI |
| `f00-plugin-hello` | example cdylib plugin |
| `f00-cli` | binary `f00` |

---

## Building from source

```bash
git clone https://github.com/theesfeld/f00
cd f00
cargo build --release -p f00-cli
./target/release/f00 --version
```

---

## Comparison

| | GNU `ls` | eza | lsd | **f00** |
|--|----------|-----|-----|---------|
| Language | C | Rust | Rust | Rust |
| Full coreutils flags | Native | Partial | Partial | **Goal: full clone** |
| Icons / git | No | Yes | Yes | Yes |
| TUI | No | No | No | **Yes** |
| Archives | No | No | No | **Yes** |
| Windows | Weak | Strong | Strong | First-class |

---

## License

MIT OR Apache-2.0

## Links

- Issues: https://github.com/theesfeld/f00/issues  
- Design: `docs/superpowers/specs/2026-07-16-f00-design.md`  
- Sync: `docs/SYNC.md`
