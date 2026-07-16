# f00

[![CI](https://img.shields.io/github/actions/workflow/status/theesfeld/f00/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/theesfeld/f00/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://github.com/theesfeld/f00#license)
[![Release](https://img.shields.io/github/v/release/theesfeld/f00?style=flat-square&include_prereleases&label=release)](https://github.com/theesfeld/f00/releases)
[![crates.io](https://img.shields.io/crates/v/f00?style=flat-square)](https://crates.io/crates/f00)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows%20%7C%20bsd-lightgrey?style=flat-square)](https://f00.sh)

**f00** — a next-generation, cross-platform **coreutils `ls` clone** in Rust, with modern UX and a supertool layer.

**Website:** [https://f00.sh](https://f00.sh) · **Binary:** `f00` · **Latest:** v0.2.0

---

## Install

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.2.0 bash
curl -fsSL https://f00.sh/install.sh | INSTALL_DIR=$HOME/bin bash
curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash   # optional ls symlink
```

---

## Features

| Area | Status | Notes |
|------|--------|--------|
| **GNU coreutils `ls`** | Shipped | Full flag surface + `--gnu` strict mode |
| **Quoting** | Shipped | `-b` `-q` `-Q` `-N` `--quoting-style` + `QUOTING_STYLE` |
| **LS_COLORS** | Shipped | Via `lscolors` / env |
| **Speed** | Shipped | Rust, layered crates, optional features |
| **Portability** | Shipped | Linux, macOS, Windows, FreeBSD |
| **Git status** | Shipped | Default feature |
| **Icons** | Shipped | `--icons` |
| **JSON / CSV / TSV / tree** | Shipped | Machine formats |
| **TOML config** | Shipped | XDG / AppData |
| **TUI browser** | Shipped | `f00 --browse` (feature `tui`, default on) |
| **Archives** | Shipped | zip / tar / tar.gz as virtual dirs |
| **Ignore files** | Shipped | `--ignore-files` → `.gitignore` / `.f00ignore` |
| **Plugins** | Planned | #27 |

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

# Icons + git (modern defaults)
f00 -la --icons --git
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

```bash
cargo build -p f00-cli --release
cargo build -p f00-cli --no-default-features   # minimal
cargo build -p f00-cli --features "git,archives,tui"
```

---

## Configuration

Unix: `~/.config/f00/config.toml` (or `$F00_CONFIG` / `--config`)

```toml
[defaults]
all = false
long = false
human = true
icons = true
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
