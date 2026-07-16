# f00 Roadmap

**Product:** f00 — modern `ls` / supertool (ACD)  
**Domain:** https://f00.sh  
**Repo:** https://github.com/theesfeld/f00  
**Binary:** `f00` · **Latest release:** [v0.1.0](https://github.com/theesfeld/f00/releases/tag/v0.1.0)  
**Design:** [docs/superpowers/specs/2026-07-16-f00-design.md](superpowers/specs/2026-07-16-f00-design.md)  
**MVP plan:** [docs/superpowers/plans/2026-07-16-f00-mvp.md](superpowers/plans/2026-07-16-f00-mvp.md)  
**Sync:** [docs/SYNC.md](SYNC.md)

## Quality bar (v0.1) — met

Core listing + common GNU flags + colors + **git** + **icons** + TOML config + install/site/CI/release.

## Milestones

| ID | Milestone | Status | Issues |
|----|-----------|--------|--------|
| **M0** | Workspace & CI skeleton | **Done** | #1 #2 |
| **M1** | Core listing | **Mostly done** | #3 #4 #5 |
| **M2** | Format | **Mostly done** | #6 #7 #8 #9 |
| **M3** | CLI + GNU flags | **Done (parity push)** | #10 #11 #12 #13 |
| **M4** | Git status | **Mostly done** | #14 #15 |
| **M5** | Icons | **Mostly done** | #16 #17 |
| **M6** | Ship path | **Mostly done** | #18 #19 #20 #21 |
| **M7** | Docs sync & hardening | **Mostly done** | #22 #23 #24 |
| **Icebox** | Supertool expansions | Backlog | #25–#30 |

## Open polish

- #5 recoverable error taxonomy hardening
- #7 full theme / LS_COLORS map
- #8 CSV/TSV + schema_version
- #10 `--gnu` golden parity tests
- #13 broader assert_cmd matrix
- #15 git column polish
- #17 icons auto/always/never enum
- #20 confirm custom domain DNS for `f00.sh` (Pages + CNAME configured)
- #23 all-features CI matrix

## Install

```bash
curl -fsSL https://f00.sh/install.sh | bash
# or
curl -fsSL https://raw.githubusercontent.com/theesfeld/f00/main/install.sh | bash
```

## Tracking

https://github.com/theesfeld/f00/issues

## Platforms

**Linux, macOS, FreeBSD, Windows** (GNU-style flags on all). Release binaries: linux x64/arm64, macOS x64/arm64, windows x64.
