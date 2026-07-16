# Docs sync process

README, GitHub Pages (`site/`), GitHub Issues/milestones, and `docs/ROADMAP.md` must stay consistent with **shipped** behavior.

## When you change user-facing behavior

1. Update CLI help / flag surface in code.
2. Update **README.md** (features table, usage, shipped vs planned labels).
3. Update **site/index.html** (feature cards, demos, roadmap strip).
4. Update **docs/ROADMAP.md** milestone status.
5. Comment on or close related **GitHub Issues**; open new ones for follow-ups.
6. If install/release URLs change, update **install.sh** and **site/install.sh** together (keep them identical).

## PR checklist

- [ ] README claims match what `cargo run -p f00 -- --help` shows
- [ ] Site does not advertise unshipped features as done
- [ ] ROADMAP status bits match open/closed milestones
- [ ] `install.sh` and `site/install.sh` are byte-identical (or intentionally generated from one source)
- [ ] No broken badge/repo links (`theesfeld/f00`, `https://f00.sh`)

## Release checklist

- [ ] Version bumped in workspace `Cargo.toml`
- [ ] Tag `vX.Y.Z` triggers release workflow
- [ ] SHA256SUMS uploaded
- [ ] `curl -fsSL https://f00.sh/install.sh | bash` works after DNS/Pages point at this repo
- [ ] Close milestone issues that shipped
