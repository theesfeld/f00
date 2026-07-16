# f00 Roadmap

**Repo:** https://github.com/theesfeld/f00  
**Latest:** [v0.2.0](https://github.com/theesfeld/f00/releases) — full coreutils surface + TUI + archives  
**Domain:** https://f00.sh

## Shipped (v0.2)

| Area | Notes |
|------|--------|
| GNU `ls` flag clone | Quoting, hide, -H, -v, -w, -x, -Z, --zero, --dired, --hyperlink, --time-style, --block-size, LS_COLORS, ctime, … |
| Long format | nlink/owner/group/author/context/inode/blocks |
| TUI | `f00 --browse` (`f00-tui`) |
| Archives | zip/tar/tgz (`f00-archive`) |
| Ignore files | `--ignore-files` |
| CSV/TSV/JSON/tree | Machine formats |
| Install / Pages / CI | f00.sh path |

## Open / later

| Issue | Topic |
|-------|--------|
| #27 | Plugin host ABI |
| #30 | FreeBSD CI smoke |
| — | Pixel-perfect locale collation / every edge of coreutils golden suite |
| — | Parallel readdir (io_uring) performance pass |

## Tracking

https://github.com/theesfeld/f00/issues · parent #31
