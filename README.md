# f00

Brand hub for **[f00.sh](https://f00.sh)** products.

Technically sharp tools. Low-fi chrome. MIT (product-specific licenses apply per repo).

## Products

| Product | Site | Repo | One-liner |
|---------|------|------|-----------|
| **f00tils** | [coreutils.f00.sh](https://coreutils.f00.sh) | [theesfeld/f00tils](https://github.com/theesfeld/f00tils) | Freestanding assembly GNU userland (`f00` multicall) |
| **clun** | [clun.sh](https://clun.sh) · [clun.f00.sh](https://clun.f00.sh) | [theesfeld/clun](https://github.com/theesfeld/clun) | JS/TS toolkit in pure Common Lisp |

## Site

Static hub under [`site/`](site/) → GitHub Pages → **https://f00.sh**

Install lives on product sites (`coreutils.f00.sh`, `clun.sh`) — not here.

```text
site/index.html   splash + product cards
site/styles.css   CRT / phosphor low-fi surface
site/app.js       canvas field + boot log
site/CNAME        f00.sh
```

Deploy: push to `main` (workflow `.github/workflows/pages.yml`).

## DNS map

| Host | Role |
|------|------|
| `f00.sh` | This hub |
| `coreutils.f00.sh` | f00tils product site + installer |
| `clun.f00.sh` | URL forward → `clun.sh` (dual domain) |
| `clun.sh` | clun product site (primary) |

Ops detail: product repos own their Pages `CNAME` files. Apex A/AAAA stay on GitHub Pages IPs.

## Org target

Product repos will move to GitHub org **`f00-sh`** (username `f00` is taken). Until transfer completes, remotes remain under `theesfeld/*`.

## License

MIT for this hub site. Product licenses are per-repo (f00tils MIT; clun GPL-3.0-or-later).
