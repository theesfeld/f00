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

1. **Hub only.** Splash + project cards. No hero shortcut buttons, no fake terminal/boot log, no install blocks. Install lives on project sites.
2. **Release → card.** Set `status: "released"` on the product in `site/catalog.json` and run `scripts/sync-from-catalog.py`. No card until released. Update catalog blurb/facts/links when the project ships again.
3. **One set of links.** Cards are the only project navigation (site + repo). Do not duplicate with hero buttons or other chrome.
4. **Public copy is user-facing only.** Never put agent process, org plans, DNS wiring, or internal preferences on the site or public README (house rule `10-user-facing-language.md`).
5. **Domains (ops, not site copy):** `f00.sh` hub · `coreutils.f00.sh` f00tils · `clun.f00.sh` clun · `cel.f00.sh` Cel Index · `trn.f00.sh` TRN · `heartbox.f00.sh` Heartbox · `dist.f00.sh` R2 packages.
6. **Aesthetic (STANDARD THEME) — ONE shared CSS.** Every f00 project loads `https://f00.sh/theme/f00-theme.css` from the hub domain first (no per-project brand CSS). Heart-Shaped Box contrasts + Bleach boxes. Path `site/theme/f00-theme.css`. Source palette: https://heartbox.f00.sh/. Three fonts: **Onyx** logo only · **zine mono** body · **chip mono** chrome. Project CSS is layout-only; never invent brand hex or soft radii.
7. **No secrets** in repo. DNS keys stay out of git.

## Layout

| Path | Role |
|------|------|
| `site/` | Cloudflare Pages project `f00` (custom domain `f00.sh`) |
| `site/catalog.json` | **SSOT** project list + theme pointer → https://f00.sh/catalog.json |
| `site/theme/` | Global theme CSS + Onyx font → https://f00.sh/theme/… |
| `scripts/sync-from-catalog.py` | Regen hub cards + README + this file from catalog |
| `scripts/enroll-project.py` | Add/update a project in catalog + inject theme link into a project repo |
| `docs/` | Optional depth |
| `.github/workflows/pages.yml` | Pages deploy (wrangler) |

## Edge (Cloudflare)

- **DNS + edge host:** Cloudflare (registrar may stay Porkbun)
- **Site:** Cloudflare Pages project `f00` → https://f00.sh
- **Packages:** R2 bucket `f00-releases` → https://dist.f00.sh/{project}/current/
- **Code:** GitHub `f00-sh/*` only (no GitHub Pages)
- Deploy sites: push `site/**` → workflow `pages.yml` (wrangler pages deploy)

## Build and gates

```bash
# after editing site/catalog.json:
python3 scripts/sync-from-catalog.py
# no app build — open site/ with a local server (or deploy)
# deploy: git push origin main  (Cloudflare Pages via Actions)
```

## Catalog (single source of truth)

**Edit only** [`site/catalog.json`](site/catalog.json) (live: https://f00.sh/catalog.json).

Then run:

```bash
python3 scripts/sync-from-catalog.py
```

That regenerates hub product cards, this table, and README projects.

| Field | URL / path |
|-------|------------|
| Catalog | `site/catalog.json` → https://f00.sh/catalog.json |
| Theme CSS | `site/theme/f00-theme.css` → https://f00.sh/theme/f00-theme-32.css |
| Org | github.com/f00-sh |

`$PROJECTS` is the developer machines' projects root (here: `/home/glenda/Projects`).

| Project | Status | Path (local) | Site | Packages |
|---------|--------|--------------|------|----------|
| f00tils | `released` | `$PROJECTS/f00tils` | https://coreutils.f00.sh/ | https://dist.f00.sh/f00tils/current/ |
| clun | `released` | `$PROJECTS/clun` | https://clun.f00.sh/ | https://dist.f00.sh/clun/current/ |
| Cel Index | `released` | `$PROJECTS/cel` | https://cel.f00.sh/ | n/a (Pages `f00-cel`) |
| TRN | `released` | `$PROJECTS/trn` | https://trn.f00.sh/ | n/a (Pages `f00-trn`) |
| Heartbox | `released` | `$PROJECTS/heartbox` | https://heartbox.f00.sh/ | n/a (Pages `f00-heartbox`) |

**Card rule:** `status=released` → hub card. `wip` / other → listed here, not on f00.sh grid.


## License

MIT for hub. Product licenses per repo.
