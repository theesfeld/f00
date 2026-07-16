# f00 Roadmap

**Repo:** https://github.com/theesfeld/f00  
**Latest:** [v0.6.0](https://github.com/theesfeld/f00/releases)  
**Domain:** https://f00.sh · DNS guide: [DNS-f00.sh.md](DNS-f00.sh.md)

## v0.6 — io_uring · dual-pane FM · locale goldens (this release)

| Track | Deliverables |
|-------|----------------|
| **io_uring** | Linux batch `statx` via io_uring (feature `io-uring`, threshold + fallback) |
| **Dual-pane FM** | Two panes in `--browse`, Tab, copy/move/delete with confirm |
| **Locale goldens** | `LC_ALL=C` + UTF-8 locale sort tests |

Phase: [#47](https://github.com/theesfeld/f00/issues/47) · Impl: [#48](https://github.com/theesfeld/f00/issues/48)

## Shipped earlier

- **v0.5** — cheap short path, uid cache, git map, Linux `statx`, TUI preview/sort/open  
- **v0.4.x** — self-update, plugins ABI, packaging stubs  
- **v0.3** — completions, man, parallel list, goldens, FreeBSD CI, HTTPS  
- **v0.2** — GNU surface, TUI base, archives, ignore, machine formats  

## Still open / later

| Item | Notes |
|------|--------|
| crates.io publish | Needs `CARGO_REGISTRY_TOKEN` |
| Homebrew tap content | Push formula + sha256s to `theesfeld/homebrew-tap` |
| `--update` rate-limit resilience | [#49](https://github.com/theesfeld/f00/issues/49) — redirect-first latest resolve |
| Install path / PATH UX | Prefer `~/.local/bin` + shell rc (tracked with #49) |

## Tracking

https://github.com/theesfeld/f00/issues
