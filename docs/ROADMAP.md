# f00 Roadmap

**Repo:** https://github.com/theesfeld/f00  
**Latest:** [v0.3.0](https://github.com/theesfeld/f00/releases)  
**Domain:** https://f00.sh · DNS guide: [DNS-f00.sh.md](DNS-f00.sh.md)

## v0.3 — Ship + Trust + Speed (shipped)

| Track | Deliverables |
|-------|----------------|
| **Ship** | Completions (`--generate-completions`), man (`--generate-man` / `man/f00.1`), icons auto/always/never, feature-matrix CI, FreeBSD smoke workflow, DNS + HTTPS at f00.sh |
| **Trust** | GNU golden suite, flag smoke tests, git polish checks, recursive unreadable-dir exit 1 |
| **Speed** | Parallel metadata (rayon, `--threads`), `--profile` timings, criterion benches + `scripts/bench-list.sh` |

Post-release: **#20** apex DNS + HTTPS enforce · **#32** CI green (feature matrix / Windows / FreeBSD).

## Shipped earlier (v0.2)

GNU `ls` surface, TUI `--browse`, archives, ignore files, JSON/CSV/TSV/tree, install + Pages CI.

## Still open / later

| Item | Notes |
|------|--------|
| **#27** | Plugin host ABI (optional, icebox) |
| Packaging | crates.io, nix flake, brew formula; self-update (`--update` / `check-update`) |
| Performance | io_uring / getdents specialized path |
| Locale goldens | Full LC_COLLATE parity matrix |

## Tracking

https://github.com/theesfeld/f00/issues
