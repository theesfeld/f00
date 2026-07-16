# f00 Roadmap

**Repo:** https://github.com/theesfeld/f00  
**Latest:** [v0.5.0](https://github.com/theesfeld/f00/releases)  
**Domain:** https://f00.sh · DNS guide: [DNS-f00.sh.md](DNS-f00.sh.md)

## v0.5 — Snappy listing + TUI (this release)

| Track | Deliverables |
|-------|----------------|
| **A Speed P1** | Cheap short listings, uid/gid cache, git map reuse |
| **B TUI** | Preview pane, sort, open editor/pager, status line |
| **C Linux** | `statx` path for directory children |

Phase: [#44](https://github.com/theesfeld/f00/issues/44) · Impl: [#45](https://github.com/theesfeld/f00/issues/45)

## Shipped earlier

- **v0.4.x** — self-update, plugins ABI, `~/.f00/bin`, packaging stubs  
- **v0.3** — completions, man, parallel list, goldens, FreeBSD CI, HTTPS  
- **v0.2** — GNU surface, TUI base, archives, ignore, machine formats  

## Still open / later

| Item | Notes |
|------|--------|
| crates.io publish | Needs `CARGO_REGISTRY_TOKEN` |
| Homebrew tap content | Push formula + v0.5 sha256s to `theesfeld/homebrew-tap` |
| io_uring | Optional next Linux micro-opt |
| Locale goldens | Full `LC_COLLATE` matrix |
| Dual-pane FM | Copy/move/delete — product decision |

## Tracking

https://github.com/theesfeld/f00/issues
