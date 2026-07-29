# AGENTS.md — f00 hub

## Project name

**f00** — brand landing for https://f00.sh

- Site: https://f00.sh
- Repo: f00-sh/f00 (org: **f00-sh/f00**)
- Not a product binary. Static hub only.

## Declared language

**Static site** (HTML/CSS/JS). No application backend. No package manager.

Shell is allowed only for bootstrap, install stubs, and packaging helpers.

## Product laws

1. **Hub only.** Splash + product cards. No hero shortcut buttons, no fake terminal/boot log, no install blocks. Install lives on product sites.
2. **Release → card.** Every f00 product that ships a real **release** gets one product card on the landing grid (`site/index.html`). No card until there is a release. Update the card when the product ships again if copy or links change.
3. **One set of links.** Cards are the only product navigation (site + repo). Do not duplicate with hero buttons or other chrome.
4. **Public copy is product-only.** Never put agent process, org plans, DNS wiring, or internal preferences on the site or public README (house rule `10-user-facing-language.md`).
5. **Domains (ops, not site copy):** `f00.sh` hub · `coreutils.f00.sh` f00tils · `clun.f00.sh` → clun (with `clun.sh`).
6. **Aesthetic:** technically modern (OKLCH, canvas, progressive enhancement), visually low-fi (CRT, phosphor, mono).
7. **No secrets** in repo. DNS keys stay out of git.

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
