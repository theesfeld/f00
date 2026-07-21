# Docs sync process

README, GitHub Pages (`site/`), GitHub Issues/milestones, `man/f00.1`, and `docs/ROADMAP.md` must stay consistent with **shipped** behavior.

## Hard rule — man page

**`man/f00.1` must ALWAYS match the current state of the project.**

- Source of truth for the manual is the tracked file `man/f00.1` (not `f00 --generate-man`).
- Every PR that changes public flags, user-visible modes, install/update story, or `Cargo.toml` version **must** update `man/f00.1` in the same unit.
- CI job **man page sync** runs `scripts/check-man-sync.sh` and **must stay green**.
- Local check: `scripts/check-man-sync.sh`

See `man/README.md`.

## When you change user-facing behavior

1. Update CLI help / flag surface in code.
2. Update **man/f00.1** (same PR — hard rule).
3. Update **README.md** (features table, usage, shipped vs planned labels).
4. Update **site/index.html** (feature cards, demos; refresh before releases).
5. Update **docs/ROADMAP.md** milestone status when used.
6. Comment on or close related **GitHub Issues**; open new ones for follow-ups.
7. If install/release URLs change, update **site/install.sh** (and packaging) together.

## PR checklist

- [ ] README claims match what `cargo run -p f00 -- --help` shows
- [ ] **`scripts/check-man-sync.sh` passes** (man version + every public long flag)
- [ ] Site does not advertise unshipped features as done
- [ ] ROADMAP status bits match open/closed milestones
- [ ] No broken badge/repo links (`theesfeld/f00`, `https://f00.sh`)

## Release checklist

- [ ] Version bumped in workspace `Cargo.toml` **and** `man/f00.1` `.TH` version
- [ ] `scripts/check-man-sync.sh` green
- [ ] Speed/bench tables on site re-run and updated for this version
- [ ] Site reflects current features/functionality
- [ ] Tag `vX.Y.Z` triggers release workflow (archives include `f00.1`)
- [ ] SHA256SUMS uploaded
- [ ] `curl -fsSL https://f00.sh/install.sh | bash` installs binary + man
- [ ] Close milestone issues that shipped
