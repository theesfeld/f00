# f00

**[f00.sh](https://f00.sh)** — home for f00 products.

## Products

> **Source of truth:** [`site/catalog.json`](site/catalog.json) → https://f00.sh/catalog.json
> **Theme:** [https://f00.sh/theme/f00-theme-9.css](https://f00.sh/theme/f00-theme-9.css) (Onyx, white on black). Do not fork colors in product repos.

| Product | Site | Repo | One-liner |
|---------|------|------|-----------|
| **f00tils** | [coreutils.f00.sh](https://coreutils.f00.sh/) | [f00-sh/f00tils](https://github.com/f00-sh/f00tils) | Freestanding assembly GNU userland (`f00` multicall) |
| **clun** | [clun.f00.sh](https://clun.f00.sh/) | [f00-sh/clun](https://github.com/f00-sh/clun) | JS/TS toolkit in pure Common Lisp |
| **Cel Index** | [cel.f00.sh](https://cel.f00.sh/) | [f00-sh/cel](https://github.com/f00-sh/cel) | Femcel / Incel self-assessment (formal product model) |
| **TRN** | [trn.f00.sh](https://trn.f00.sh/) | [f00-sh/trn](https://github.com/f00-sh/trn) | Enhanced Tabular Recipe Notation converter (eTRN) |

_After editing `catalog.json`, run `python3 scripts/sync-from-catalog.py`._


## Site

Static Pages source: [`site/`](site/) → **https://f00.sh**

Layout: splash + product cards only. Installers live on each product site.

**Release rule:** a f00 product gets a landing card when it has a real release.

## License

MIT for this site. Product licenses are per-repo (f00tils MIT; clun MIT).
