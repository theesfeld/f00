# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note:** While f00 is `0.x`, minor versions may include breaking changes (documented here).

## [0.10.2] - 2026-07-16

### Fixed
- **Drop-in GNU behavior** under `--gnu` (not just flag parse):
  - `-l` / `-s` print GNU `total N` for directory listings (not single-file operands)
  - `-s` uses **1024-byte** display units by default (512 with `POSIXLY_CORRECT`)
  - `-R` section order matches coreutils (all siblings, then each subdir)
  - `-v` strverscmp: `~` sorts before digits (`file~` before `file1`)
  - `--sort=width` implemented (shortest name width first)
  - `-b` / `--escape` escapes spaces (`x y` → `x\ y`)
  - `--group-directories-first` honored even with `--gnu`
- Site demos: full **tree** and **JSON** examples; drop-in story clarified

### Changed
- Version **0.10.1 → 0.10.2**

## [0.10.1] - 2026-07-16

### Fixed
- **`--color=WHEN` GNU synonyms:** accept `tty` / `if-tty` (auto), plus `yes`/`force` (always) and `no`/`none` (never), matching coreutils so distro `ls` aliases work (NixOS injects `--color=tty`)
- **`--classify[=WHEN]`** / `-F`: accept optional WHEN (same vocabulary as `--color`); bare `-F` / `--classify` still means always
- **`--hyperlink[=WHEN]`**: strict WHEN enum with the same GNU synonyms (unknown values rejected)
- Config `color = "tty"` / `"if-tty"` parsed the same way as the CLI

### Changed
- Version **0.10.0 → 0.10.1**

## [0.10.0] - 2026-07-16

### Added
- **Nerd Font icons** (eza-style): special directory glyphs for Desktop, Downloads, Music, Pictures, Videos, Documents, Projects, nixos, `.git`, `.config`, and more (case-insensitive)
- **Richer file-type icons** by extension (rs/py/js/ts/media/archives/…) and basename (Cargo.toml, Dockerfile, README, flake.nix, …)

### Changed
- Icons use **Nerd Font** code points instead of emoji (install a Nerd Font in the terminal for correct glyphs; disable with `--icons=never` / `--gnu`)
- Version **0.9.0 → 0.10.0**

## [0.9.0] - 2026-07-16

### Added
- **Modern long-format colors** (non-GNU): tinted perms (type/rwx), owner/group, size-by-magnitude, blue timestamps; symlink name + dim `→` target
- Disabled under `--gnu` / `F00_GNU` and when colors are off (`--color=never` / `NO_COLOR`)

### Changed
- Version **0.8.0 → 0.9.0**

## [0.8.0] - 2026-07-16

### Added
- **TUI:** syntax-colored file previews (syntect) for source/text files
- **TUI:** directory listing cache (mtime-aware) for snappier navigation
- **Git:** process-wide porcelain cache invalidated on `.git/index` mtime
- **Recursive walk:** parallel directory walk via **jwalk**
- **Linux:** io_uring batch `statx` for large recursive listings (same feature as flat dirs)

### Changed
- **Git:** `-R` / `--tree` use `git status -uno` (skip untracked scan) for speed; flat listings still use `-uall`
- **Long format:** less allocation when computing column widths
- Version **0.7.2 → 0.8.0**

## [0.7.2] - 2026-07-16

### Fixed
- **`--tree` connectors:** correct vertical bars / last-sibling detection (O(n) precompute; large trees no longer mis-nest)

### Changed
- **`--tree` performance:** parallel metadata after WalkDir, skip dir-section headers for tree mode; much faster format + stat phases on large trees
- Version **0.7.1 → 0.7.2**

## [0.7.1] - 2026-07-16

### Fixed
- **`--update` / `--check-update`**: resolve latest via `github.com/.../releases/latest` redirect first (same as install.sh), avoiding GitHub API **403** when unauthenticated rate limits are exhausted; optional `GITHUB_TOKEN` / `GH_TOKEN` for API fallback

### Changed
- Installer default directory: **`~/.local/bin`** (was `~/.f00/bin`); `ADD_PATH` ensures shell rc when missing; warns on legacy `~/.f00/bin`
- Version **0.7.0 → 0.7.1**

## [0.7.0] - 2026-07-16

### Added
- **Richer `--json` / `-j`:** full machine dump — timestamps (mtime/atime/ctime/btime), inode, nlink, blocks, uid/gid, owner/group/author, permissions string, readonly, extension, absolute_path, SELinux context when filled; `mode_octal` alias
- **`-j`** short flag for `--json` (GNU `ls` has no `-j`)
- JSON/CSV/TSV resolve owner/group names by default (skip with `-n`)
- CSV/TSV columns aligned with the rich JSON field set

### Changed
- Version **0.6.0 → 0.7.0**

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

[0.10.2]: https://github.com/theesfeld/f00/releases/tag/v0.10.2
[0.10.1]: https://github.com/theesfeld/f00/releases/tag/v0.10.1
[0.10.0]: https://github.com/theesfeld/f00/releases/tag/v0.10.0
[0.9.0]: https://github.com/theesfeld/f00/releases/tag/v0.9.0
[0.8.0]: https://github.com/theesfeld/f00/releases/tag/v0.8.0
[0.7.2]: https://github.com/theesfeld/f00/releases/tag/v0.7.2
[0.7.1]: https://github.com/theesfeld/f00/releases/tag/v0.7.1
[0.7.0]: https://github.com/theesfeld/f00/releases/tag/v0.7.0
[0.6.0]: https://github.com/theesfeld/f00/releases/tag/v0.6.0
[0.5.0]: https://github.com/theesfeld/f00/releases/tag/v0.5.0
[0.4.0]: https://github.com/theesfeld/f00/releases/tag/v0.4.0
[0.3.0]: https://github.com/theesfeld/f00/releases/tag/v0.3.0
[0.2.0]: https://github.com/theesfeld/f00/releases/tag/v0.2.0
[0.1.0]: https://github.com/theesfeld/f00/releases/tag/v0.1.0
