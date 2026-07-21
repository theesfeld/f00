# man pages

## Hard rule

**`man/f00.1` must always match the current state of the project.**

This is mandatory for every PR that changes:

- CLI flags or help text (`crates/f00-cli`)
- User-visible behavior described in the man page (JSON, tree, colors, modes, install, update)
- `Cargo.toml` `version` (update the `.TH` version string in the same unit)

CI enforces this via:

```bash
scripts/check-man-sync.sh
```

The check fails if any public long flag from `f00 --help` is missing from `man/f00.1`, if required sections/claims are gone, or if the man page version does not match `Cargo.toml`.

## Authoritative source

| Artifact | Role |
|----------|------|
| **`man/f00.1`** | **Source of truth** for the shipped manual (hand-maintained, comprehensive) |
| `f00 --generate-man` | Developer dump of clap options only — **not** a replacement for `man/f00.1` |
| `f00 --help` | Live flag list; must be a subset of what the man page documents |

## Install

Release archives include `f00.1`. `site/install.sh` installs it to:

- `$XDG_DATA_HOME/man/man1/f00.1` if `XDG_DATA_HOME` is set
- otherwise `~/.local/share/man/man1/f00.1`

Override with `MAN_DIR`; skip with `F00_INSTALL_MAN=0`.

## Local preview

```bash
man -l man/f00.1
# or
groff -man -Tutf8 man/f00.1 | less -R
```
