# f00 Roadmap

**Repo:** https://github.com/theesfeld/f00  
**Latest:** [v0.4.1](https://github.com/theesfeld/f00/releases)  
**Domain:** https://f00.sh · DNS guide: [DNS-f00.sh.md](DNS-f00.sh.md)

## v0.4.1 — Decorate hooks + crates/brew polish

| Track | Deliverables |
|-------|----------------|
| **Plugins** | Decorate hooks apply JSON transforms before format; hello plugin demo prefix |
| **Homebrew** | `Formula/f00.rb` with live SHA-256 for v0.4.0 assets |
| **crates.io** | Package name `f00` + publish script for library graph |

Issue: [#39](https://github.com/theesfeld/f00/issues/39)

## v0.4.0 — Packaging + update + plugins (shipped)

| Track | Deliverables |
|-------|----------------|
| **Update** | `f00 --update` / `update`, `f00 --check-update` / `check-update` |
| **Install** | Default `~/.f00/bin` |
| **Packaging** | CHANGELOG, nix flake, Homebrew formula stub |
| **Plugins** | Host ABI v1 scaffold |

Phase: [#36](https://github.com/theesfeld/f00/issues/36) · Impl: [#37](https://github.com/theesfeld/f00/issues/37)

## v0.3 — Ship + Trust + Speed (shipped)

Completions, man, icons auto, parallel list, `--profile`, golden tests, FreeBSD CI, HTTPS at f00.sh.

## Shipped earlier (v0.2)

GNU `ls` surface, TUI `--browse`, archives, ignore files, JSON/CSV/TSV/tree, install + Pages CI.

## Still open / later

| Item | Notes |
|------|--------|
| Plugin ecosystem | More plugins; filter/sort hooks beyond display rename |
| crates.io | Keep versions in lockstep; `./scripts/publish-crates.sh` |
| Homebrew tap | Publish `theesfeld/homebrew-tap` (formula has sha256s) |
| Performance | io_uring / getdents specialized path |
| Locale goldens | Full LC_COLLATE parity matrix |

## Tracking

https://github.com/theesfeld/f00/issues
