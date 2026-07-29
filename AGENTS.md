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
5. **Domains (ops, not site copy):** `f00.sh` hub · `coreutils.f00.sh` f00tils · `clun.f00.sh` clun · `cel.f00.sh` Cel Index · `trn.f00.sh` TRN · `heartbox.f00.sh` Heartbox · `dist.f00.sh` R2 packages.
6. **Aesthetic:** white on black, Onyx type. Single global theme at `site/theme/f00-theme.css` → https://f00.sh/theme/f00-theme.css (all product Pages must link this; do not redefine brand colors/fonts in product CSS). Layout/chrome may stay low-fi (CRT, scanlines) but monochrome only — no phosphor green.
7. **No secrets** in repo. DNS keys stay out of git.

## Layout

| Path | Role |
|------|------|
| `site/` | Cloudflare Pages project `f00` (custom domain `f00.sh`) |
| `docs/` | Optional depth |
| `.github/workflows/pages.yml` | Pages deploy (wrangler) |

## Edge (Cloudflare)

- **DNS + edge host:** Cloudflare (registrar may stay Porkbun)
- **Site:** Cloudflare Pages project `f00` → https://f00.sh
- **Packages:** R2 bucket `f00-releases` → https://dist.f00.sh/{product}/current/
- **Code:** GitHub `f00-sh/*` only (no GitHub Pages)
- Deploy sites: push `site/**` → workflow `pages.yml` (wrangler pages deploy)

## Build and gates

```bash
# no build — open site/index.html locally
# deploy: git push origin main  (Cloudflare Pages via Actions)
```

## Sister products

| Product | Path (local) | Site | Packages |
|---------|--------------|------|----------|
| f00tils | `/home/glenda/Projects/f00tils` | https://coreutils.f00.sh | https://dist.f00.sh/f00tils/current/ |
| clun | `/home/glenda/Projects/clun` | https://clun.f00.sh | https://dist.f00.sh/clun/current/ |
| Cel Index | `/home/glenda/Projects/cel` | https://cel.f00.sh | n/a (Pages `f00-cel`) |
| TRN | `/home/glenda/Projects/trn` | https://trn.f00.sh | n/a (Pages `f00-trn`) |
| Heartbox | `/home/glenda/Projects/heartbox` | https://heartbox.f00.sh | n/a (Pages `f00-heartbox`) |

## License

MIT for hub. Product licenses per repo.
