# f00 Roadmap

**Product:** f00 — modern `ls` / supertool (ACD)  
**Domain:** https://f00.sh  
**Repo:** https://github.com/theesfeld/f00  
**Binary:** `f00`  
**Design:** [docs/superpowers/specs/2026-07-16-f00-design.md](superpowers/specs/2026-07-16-f00-design.md)  
**MVP plan:** [docs/superpowers/plans/2026-07-16-f00-mvp.md](superpowers/plans/2026-07-16-f00-mvp.md)  
**Sync:** [docs/SYNC.md](SYNC.md)

## Quality bar (v0.1)

Core listing + common GNU flags + colors + **git** + **icons** + TOML config + install/site/CI.  
TUI, archives, and plugins are **not** required for MVP (see Icebox).

## Milestones

| ID | Milestone | Status | Summary | Issues |
|----|-----------|--------|---------|--------|
| **M0** | Workspace & CI skeleton | **Done** | Cargo workspace, crates, CI matrix | #1 #2 |
| **M1** | Core listing | **Mostly done** | Entry, readdir, filter, sort, `-R` | #3 #4 #5 |
| **M2** | Format | **Mostly done** | Long/columns, colors, tree, json | #6 #7 #8 #9 |
| **M3** | CLI + GNU flags | **In progress** | clap, TOML, `--gnu`, exit codes, argv0 | #10 #11 #12 #13 |
| **M4** | Git status | **Mostly done** | `f00-git` + annotations | #14 #15 |
| **M5** | Icons | **Mostly done** | Icon map; auto enum polish open | #16 #17 |
| **M6** | Ship path | **In progress** | install.sh, releases, Pages, README | #18 #19 #20 #21 |
| **M7** | Docs sync & hardening | **In progress** | SYNC, PR template, release | #22 #23 #24 |
| **Icebox** | Supertool expansions | Backlog | TUI, archives, plugins | #25–#30 |

## Open work (priority)

1. #20 Pages live + DNS for `f00.sh`
2. #24 Tag `v0.1.0` release once CI green
3. #11 / #12 polish (TOML + argv0 landed — close when verified)
4. #7 #8 #10 remaining format/compat parity
5. Icebox only after v0.1 quality bar

## Tracking

All planning and bugs are **GitHub Issues**:  
https://github.com/theesfeld/f00/issues

## Docs sync

User-facing changes must update README + `site/` together. See [SYNC.md](SYNC.md).

## Platforms

Day one: **Linux, macOS, FreeBSD, Windows** (GNU-style flags on all).
