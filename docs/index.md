# f00

Brand hub for [f00.sh](https://f00.sh) products under org [f00-sh](https://github.com/f00-sh).

Keep this page aligned with [README.md](../README.md), [man/f00.1.md](../man/f00.1.md),
and the live catalog at https://f00.sh/catalog.json.

## Why this project exists

f00 is the public front door and shared brand layer for freestanding tools and
related products. The hub is intentionally thin: splash + product cards.
Install UX lives on product sites.

## Default theme (Heartbox)

| Token | Hex | Role |
|-------|-----|------|
| Background | `#1A1214` | Hospital-night underpaint |
| Foreground | `#F4EBE0` | Cream light |
| Accent | `#E02030` | Poppy red |
| Metal | `#B8C0C8` | Silver chrome |
| Sky | `#5EC8E8` | Secondary / links |

- Theme CSS: https://f00.sh/theme/f00-theme-13.css
- Canonical palette: [f00-sh/heartbox](https://github.com/f00-sh/heartbox) `palette/heartbox.json`
- Product showcase: https://heartbox.f00.sh

Fonts (unchanged roles): **Onyx** logo only · **zine mono** body · **chip mono** chrome.

## Products

See the README products table (generated from `site/catalog.json`).

## Operator docs

| Surface | Location |
|---|---|
| README | [README.md](../README.md) |
| Man page | [man/f00.1.md](../man/f00.1.md) |
| Platform SOP (PDF) | [sop-f00-org-ops.pdf](sop-f00-org-ops.pdf) |
| Platform SOP (JSON) | [sop-f00-org-ops.json](sop-f00-org-ops.json) |
| Catalog | [site/catalog.json](../site/catalog.json) |
| Scene card | [file_id.diz](../file_id.diz) |

## Site

Static Pages source: `site/` → Cloudflare Pages project `f00` → https://f00.sh

After catalog edits:

```text
python3 scripts/sync-from-catalog.py
```

## License

[MIT](../LICENSE)
