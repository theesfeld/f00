# AGENTS.md — f00 hub

## Project name

**f00** — brand landing for https://f00.sh

- Site: https://f00.sh
- Repo: theesfeld/f00 (org target: **f00-sh/f00**)
- Not a product binary. Static hub only.

## Declared language

**Static site** (HTML/CSS/JS). No application backend. No package manager.

Shell is allowed only for bootstrap, install stubs, and packaging helpers.

## Product laws

1. **Hub only.** Link out to product domains; do not re-host installers here unless intentional mirror.
2. **Domains:** `f00.sh` hub · `coreutils.f00.sh` f00tils · `clun.f00.sh` → clun (dual with `clun.sh`).
3. **Aesthetic:** technically modern (OKLCH, canvas, progressive enhancement), visually low-fi (CRT, phosphor, mono).
4. **No secrets** in repo. DNS keys stay out of git.

## Layout

| Path | Role |
|------|------|
| `site/` | GitHub Pages root (`CNAME` = `f00.sh`) |
| `docs/` | Optional depth |
| `.github/workflows/pages.yml` | Pages deploy |

## Build and gates

```bash
# no build — open site/index.html locally
# deploy: git push origin main
```

## Sister products

| Product | Path (local) | Site |
|---------|--------------|------|
| f00tils | `/home/glenda/Projects/f00tils` | https://coreutils.f00.sh |
| clun | `/home/glenda/Projects/clun` | https://clun.sh · https://clun.f00.sh |

## License

MIT for hub. Product licenses per repo.
