# f00

[![CI](https://img.shields.io/github/actions/workflow/status/theesfeld/f00/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/theesfeld/f00/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](https://github.com/theesfeld/f00#license)
[![Release](https://img.shields.io/github/v/release/theesfeld/f00?style=flat-square&include_prereleases&label=release)](https://github.com/theesfeld/f00/releases)
[![crates.io](https://img.shields.io/crates/v/f00?style=flat-square)](https://crates.io/crates/f00)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows%20%7C%20bsd-lightgrey?style=flat-square)](https://f00.sh)

**f00** — a next-generation, cross-platform `ls` rewrite in Rust.

Fast directory listings with modern UX (colors, icons, tree, JSON, git status) and an optional GNU-compatible mode when you need drop-in muscle memory.

**Website:** [https://f00.sh](https://f00.sh) · **Binary:** `f00`

---

## Install

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

Pin a version or choose an install directory:

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.1.0 bash
curl -fsSL https://f00.sh/install.sh | INSTALL_DIR=$HOME/bin bash
```

Optionally install a `ls` symlink (off by default):

```bash
curl -fsSL https://f00.sh/install.sh | F00_INSTALL_LS=1 bash
```

Or build from source (see [Building from source](#building-from-source)).

---

## Features

| Area | Status | Notes |
|------|--------|--------|
| **Speed** | Shipped | Rust, minimal allocations, parallel-friendly design |
| **Portability** | Shipped | Linux, macOS, Windows, FreeBSD targets |
| **GNU mode** | Shipped | `--gnu` / compat path for familiar flags & output shape |
| **Modern UX** | Shipped | Colors, human sizes, recursive listing, sensible defaults |
| **Git status** | Shipped | Inline dirty/clean indicators in repos |
| **Icons** | Shipped | File-type icons when the terminal supports them |
| **JSON / tree** | Shipped | Machine-readable JSON and tree views |
| **Config (TOML)** | Shipped | `~/.config/f00/config.toml` (XDG on Unix) |
| **TUI browser** | Planned | Interactive directory browser |
| **Archives** | Planned | List inside zip/tar without extracting |

MVP ships: plain listing, broad **GNU `ls` flag parity**, color, tree, JSON, icons, git status, recursive, human sizes, TOML config, and strict `--gnu` mode. Planned (TUI/archives) tracked on GitHub Issues.

### GNU-compatible flags (subset)

| Flag | Meaning |
|------|---------|
| `-a` `-A` | all / almost-all |
| `-l` `-1` `-C` `-m` `-x` | long / one-column / columns / commas |
| `-h` `--si` | human sizes (1024 / 1000) |
| `-r` `-t` `-S` `-X` `-U` `-f` | reverse / time / size / extension / unsorted / `-a -U` |
| `--sort=` `--time=` `-u` `-c` | sort key / timestamp selection |
| `-R` `-d` | recursive / list directory itself |
| `-F` `-p` `--file-type` | classify indicators |
| `-B` `-I PATTERN` | ignore backups / ignore pattern |
| `-L` | dereference symlinks |
| `-g` `-o` `-G` `-n` | long without owner/group / numeric IDs |
| `-i` `-s` | inode / blocks |
| `--full-time` `--group-directories-first` `--color` `--format` | extras |
| `--gnu` / `F00_GNU=1` | strict mode: no icons/git, classic sorting |

---

## Usage

```bash
# List current directory
f00

# Long listing (permissions, nlink, owner, group, size, mtime)
f00 -la

# Human-readable sizes
f00 -lah

# List the directory itself (not contents)
f00 -ld /var/log

# Recursive
f00 -R ~/projects

# Tree / JSON
f00 --tree
f00 --json

# Icons + git status (when available)
f00 -la --icons --git

# Strict GNU-shaped behavior (script-safe)
f00 --gnu -la /tmp

# Ignore patterns
f00 -I '*.o' -B

# Help
f00 --help
```

---

## Comparison

| | GNU `ls` | [eza](https://github.com/eza-community/eza) | [lsd](https://github.com/lsd-rs/lsd) | **f00** |
|--|----------|---------------------------------------------|--------------------------------------|---------|
| Language | C | Rust | Rust | Rust |
| Cross-platform | Unix-first | Strong | Strong | First-class (incl. Windows) |
| GNU flag compat | Native | Partial | Partial | Explicit `--gnu` mode |
| Icons | No | Yes | Yes | Yes |
| Git status | No | Yes | Yes | Yes |
| Tree | No (needs `tree`) | Yes | Yes | Yes |
| JSON output | No | Limited | No | Yes |
| TOML config | No | Yes | Yes | Yes |
| Install size / deps | System | Binary | Binary | Single static-ish binary |
| TUI / archives | No | No | No | Planned |

f00 aims to be the “modern defaults + escape hatch to GNU” option rather than a pure clone of either classic `ls` or existing Rust replacements.

---

## Configuration

Default config path (Unix): `~/.config/f00/config.toml`

```toml
# ~/.config/f00/config.toml

[display]
color = "auto"          # auto | always | never
icons = true
human_sizes = true
git = true

[listing]
all = false             # like -a
long = false            # like -l
recursive = false
tree = false

[compat]
gnu = false             # default to GNU-shaped flags/output

[theme]
# Optional named accents; terminals without truecolor fall back gracefully
# accent = "green"
```

CLI flags always override config.

---

## Building from source

Requirements: [Rust](https://rustup.rs/) stable (edition 2021+).

```bash
git clone https://github.com/theesfeld/f00.git
cd f00
cargo build --release
./target/release/f00 -la
```

Install the binary into your cargo bin dir:

```bash
cargo install --path crates/f00-cli
```

### Cargo features

Defined on `f00-cli`:

| Feature | Default | Enables |
|---------|---------|---------|
| **`git`** | yes | `f00-git` integration (status column / indicators) |

Core listing, color, tree, JSON, icons, human sizes, and GNU mode ship without extra feature flags (icons/format live in `f00-format`).

```bash
# Default (includes git)
cargo build --release -p f00-cli

# Without git
cargo build --release -p f00-cli --no-default-features
```

Check each crate’s `Cargo.toml` for the source of truth as features evolve.

---

## Project structure

Workspace layout:

```
f00/
├── crates/
│   ├── f00-cli/      # Binary entrypoint, clap args, UX
│   ├── f00-core/     # Directory walk, metadata, sorting
│   ├── f00-format/   # Color, columns, tree, JSON, icons
│   ├── f00-git/      # Git status integration
│   └── f00-compat/   # GNU ls mode / flag translation
├── site/             # https://f00.sh (GitHub Pages)
├── install.sh        # curl|bash installer
└── docs/             # Design notes & specs
```

---

## Roadmap

Near-term focus:

1. Solid MVP: listing, `-la`, color, tree, JSON, icons, git, recursive, human sizes, `--gnu`
2. Cross-platform release binaries + `curl | bash` installer
3. Config polish and shell completions
4. **Planned:** interactive TUI browser
5. **Planned:** archive listing (zip/tar/…)

Feature tracking and design discussion live on **[GitHub Issues](https://github.com/theesfeld/f00/issues)** — not in a separate board.

---

## Contributing

Contributions welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, style, and PR expectations.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in f00 shall be dual-licensed as above, without additional terms or conditions.
