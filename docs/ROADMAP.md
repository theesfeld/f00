# f00 Roadmap

**Repo:** https://github.com/theesfeld/f00  
**Latest:** [v0.4.0](https://github.com/theesfeld/f00/releases)  
**Domain:** https://f00.sh · DNS guide: [DNS-f00.sh.md](DNS-f00.sh.md)

## v0.4 — Packaging + update + plugins (this release)

| Track | Deliverables |
|-------|----------------|
| **Update** | `f00 --update` / `update`, `f00 --check-update` / `check-update` (Releases API + SHA-256) |
| **Install** | Default `~/.f00/bin`; PATH guidance |
| **Packaging** | CHANGELOG, crates.io metadata, nix flake stub, Homebrew formula stub |
| **Plugins** | Host ABI v1 (`f00-plugin`, feature `plugins`, example `f00-plugin-hello`) |

Phase: [#36](https://github.com/theesfeld/f00/issues/36) · Impl: [#37](https://github.com/theesfeld/f00/issues/37) · Plugins: [#27](https://github.com/theesfeld/f00/issues/27)

## v0.3 — Ship + Trust + Speed (shipped)

Completions, man, icons auto, parallel list, `--profile`, golden tests, FreeBSD CI, HTTPS at f00.sh.

## Shipped earlier (v0.2)

GNU `ls` surface, TUI `--browse`, archives, ignore files, JSON/CSV/TSV/tree, install + Pages CI.

## Still open / later

| Item | Notes |
|------|--------|
| Plugin ecosystem | Real decorate hooks in listing path; more example plugins |
| crates.io publish | `cargo publish -p f00-cli` once token/config ready |
| Homebrew tap | Publish `Formula/f00.rb` under a tap with real sha256s |
| Performance | io_uring / getdents specialized path |
| Locale goldens | Full LC_COLLATE parity matrix |

## Tracking

https://github.com/theesfeld/f00/issues
