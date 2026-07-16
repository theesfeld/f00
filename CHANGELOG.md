# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note:** While f00 is `0.x`, minor versions may include breaking changes (documented here).

## [0.6.0] - 2026-07-16

### Added
- **Linux io_uring:** optional batch `statx` for large directories (cargo feature `io-uring`, default on; `--io-uring=false` to disable). Falls back to rayon/`statx`/std when unavailable.
- **Dual-pane TUI FM:** `f00 --browse` dual panes (on by default when terminal ≥ 80 cols); `Tab` switch; `\`/`|` toggle; `c` copy / `m` move / `d` delete with confirmation between panes
- **Locale goldens:** integration tests under `LC_ALL=C` and UTF-8 locales when present (`locale_goldens`)
- Linux `statx` via raw `SYS_statx` syscall (avoids hard-linking glibc `statx` for cross toolchains)

### Changed
- Version **0.5.0 → 0.6.0**

## [0.5.0] - 2026-07-16

### Added
- **Speed P1:** short listings skip NSS owner/group and SELinux xattr unless long/`-Z` need them
- Process-wide **uid/gid name cache** for long format
- **Git:** one porcelain map per repository root (reused across path args)
- **Linux:** `statx(2)` path for directory children when `linux_statx` is on (default)
- **TUI:** preview pane (`p`), sort cycle (`s` / reverse `S`), open in `$EDITOR` (`e`) / `$PAGER` (`v`)
- **TUI:** richer status line (counts, sort mode, marks)

### Changed
- Version **0.4.1 → 0.5.0**

## [0.4.1] - 2026-07-16

### Added
- Plugin **decorate hooks**: with `--features plugins`, loaded plugins transform entry lists before format (JSON ABI `display_name` / `name`)
- Example plugin `hello` injects a `· ` display prefix via decorate
- `scripts/publish-crates.sh` for crates.io publish order
- Homebrew formula SHA-256s for v0.4.0 multi-arch assets

### Changed
- Binary package renamed for crates.io: Cargo package name **`f00`** (path still `crates/f00-cli`, lib `f00_cli`)
- Version **0.4.0 → 0.4.1**

## [0.4.0] - 2026-07-16

### Added
- Self-update: `f00 --update` / `f00 update` downloads the latest GitHub Release asset, verifies SHA-256, and replaces the running binary
- `f00 --check-update` / `f00 check-update` reports whether a newer release is available (exit `1` if behind)
- Plugin host ABI v1 behind cargo feature `plugins` (`f00-plugin` crate + example `f00-plugin-hello`)
- `f00 --list-plugins` when built with `plugins`
- Installer default directory: `~/.f00/bin` (override with `INSTALL_DIR`)
- `CHANGELOG.md`, nix flake stub, Homebrew formula stub
- crates.io-oriented package metadata (keywords, categories, readme)

### Changed
- Version **0.3.0 → 0.4.0**
- Installer prefers `~/.f00/bin` over `~/.local/bin`

### Fixed
- CI green across Linux/macOS/Windows/FreeBSD for feature-matrix and permission tests (from 0.3.x patch train)

## [0.3.0] - 2026-07-16

### Added
- Shell completions (`--generate-completions`) and man page (`--generate-man` / `man/f00.1`)
- Icons `auto|always|never`
- Parallel metadata collection (rayon, `--threads`) and `--profile` timings
- GNU golden / smoke trust tests; FreeBSD smoke CI

## [0.2.0] - 2026-07-16

### Added
- Full GNU `ls` surface + `--gnu`, TUI `--browse`, archives, ignore files
- JSON/CSV/TSV/tree, git column, install.sh + Pages

## [0.1.0] - 2026-07-16

### Added
- Initial MVP workspace and listing core

[0.6.0]: https://github.com/theesfeld/f00/releases/tag/v0.6.0
[0.5.0]: https://github.com/theesfeld/f00/releases/tag/v0.5.0
[0.4.0]: https://github.com/theesfeld/f00/releases/tag/v0.4.0
[0.3.0]: https://github.com/theesfeld/f00/releases/tag/v0.3.0
[0.2.0]: https://github.com/theesfeld/f00/releases/tag/v0.2.0
[0.1.0]: https://github.com/theesfeld/f00/releases/tag/v0.1.0
