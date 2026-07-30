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
8. **Projection Specimen Engine (org-wide goal).** f00 is an organic experience projected onto the display — not pages that “load.” **Every object is a projection:** field, boxes, header/footer rules, body type, chrome, splash plate. Scope is **everything** — hub + all project sites + all scenes.
   - **Vocabulary:** say *projected onto the display* / *this throw* / *this specimen* — not “page load,” “onLoad,” “reset to default,” or “same every refresh.”
   - **Entropy (zen band):** every projection is slightly different — field seat, box axes, line weight, tracking — like film loaded a millimeter off, emulsion blend, bulb age. Physics-based organic variation only. **Usability is law:** never so chaotic that copy is unreadable or targets unusable.
   - **Order in disorder (key tenet):** chaos is **not uniform**. **Uniformity is impossible outside mathematics** — the only fully standard, reproducible concept in nature. No flat global blur (CSS `blur(r)` is manufactured), no identical offsets, no cloned noise. Emulsion line may be nearly straight on the cel; the *view* under light is irregular. Species without clones.
   - **Every object is its own projection onto the display:** logo, cards, header, footer, text, lines, letters, background — each is a complete throw (light → film/object → lens → screen) with private seed and private optical stack (gate, keystone, defocus, lamp, emulsion). Not layout jitter and not one shared wind. An entropic *view* of a natural thing; species recognisable, specimen never a clone. No primary object.
   - **No manufactured perfection:** no rest pose that is pure identity; no exact-duplicate throws; no razor CAD edges as the aesthetic goal; no timer choreography that feels like a metronome. Nature = pattern + chaos + entropy.
   - **What stays fixed:** brand tokens (theme, palette, type roles), structure, copy intent, product identity — so it still reads as f00.
   - **What lives:** optics and material presence — soft edges, continuous organic motion where motion exists, specimen uniqueness when a surface is first projected, hospital-flower light not UI chrome polish.
   - **Implementation:** shared `https://f00.sh/theme/f00-theme-34.css` + `https://f00.sh/theme/f00-entropy.js` (Worker injects on `*.f00.sh`). **throw** (`$PROJECTS/throw`, wip) is the develop→project engine — not blur filters. Hub mark uses `site/throw/` + `throw-plate.js`.
   - **Art law:** digital is the frame; the picture is imperfect organic reality (medical models, poppies, foil, dye, innards). Do not ship static boilerplate text-on-flat as the emotional register of a scene.

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
| Theme CSS | `site/theme/f00-theme.css` → https://f00.sh/theme/f00-theme-34.css |
| Org | github.com/f00-sh |

`$PROJECTS` is the developer machines' projects root (here: `/home/glenda/Projects`).

| Project | Status | Path (local) | Site | Packages |
|---------|--------|--------------|------|----------|
| f00tils | `released` | `$PROJECTS/f00tils` | https://coreutils.f00.sh/ | https://dist.f00.sh/f00tils/current/ |
| clun | `released` | `$PROJECTS/clun` | https://clun.f00.sh/ | https://dist.f00.sh/clun/current/ |
| Cel Index | `released` | `$PROJECTS/cel` | https://cel.f00.sh/ | n/a (Pages `f00-cel`) |
| TRN | `released` | `$PROJECTS/trn` | https://trn.f00.sh/ | n/a (Pages `f00-trn`) |
| Heartbox | `released` | `$PROJECTS/heartbox` | https://heartbox.f00.sh/ | n/a (Pages `f00-heartbox`) |
| throw | `wip` | — | https://throw.f00.sh/ | n/a |

**Card rule:** `status=released` → hub card. `wip` / other → listed here, not on f00.sh grid.


## License

MIT for hub. Product licenses per repo.
