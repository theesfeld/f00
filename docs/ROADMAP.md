# f00 Roadmap

**Product:** f00 — modern `ls` / supertool (ACD)  
**Domain:** https://f00.sh  
**Binary:** `f00`  
**Design:** [docs/superpowers/specs/2026-07-16-f00-design.md](superpowers/specs/2026-07-16-f00-design.md)  
**MVP plan:** [docs/superpowers/plans/2026-07-16-f00-mvp.md](superpowers/plans/2026-07-16-f00-mvp.md)

## Quality bar (v0.1)

Core listing + common GNU flags + colors + **git** + **icons** + install/site/CI.  
TUI, archives, and plugins are **not** required for MVP.

## Milestones

| ID | Milestone | Status | Summary |
|----|-----------|--------|---------|
| **M0** | Workspace & CI skeleton | Planned | Cargo workspace, crate stubs, GitHub Actions matrix |
| **M1** | Core listing | Planned | Entry, readdir, filter, sort, recursive errors |
| **M2** | Format | Planned | Long/columns, colors, tree, json/csv/tsv |
| **M3** | CLI + GNU flags | Planned | clap, TOML config, `--gnu`, exit codes |
| **M4** | Git status | Planned | `f00-git` + status column (default feature) |
| **M5** | Icons | Planned | Nerd Font map, auto TTY, `--no-icons` |
| **M6** | Ship path | Planned | install.sh, releases, f00.sh Pages, README |
| **M7** | Docs sync & hardening | Planned | SYNC process, PR template, full test matrix |
| **Icebox** | Supertool expansions | Backlog | `f00-tui`, `f00-archive`, plugins, full LS_COLORS |

## Tracking

All planning work is tracked as **GitHub Issues** under the milestones above.  
Issue titles and bodies to create are listed in the MVP plan section **GitHub Issues to Open**.

## Docs sync

User-facing changes must update README + `site/` together. See plan Task 19 / `docs/SYNC.md` (added during M7).

## Platforms

Day one: **Linux, macOS, FreeBSD, Windows** (GNU-style flags on all).
