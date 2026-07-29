#!/usr/bin/env python3
"""Regenerate hub docs (and optional HTML fallback) from site/catalog.json.

SSOT: site/catalog.json → https://f00.sh/catalog.json
Theme: site/theme/f00-theme.css → https://f00.sh/theme/f00-theme.css

Usage:
  python3 scripts/sync-from-catalog.py
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG_PATH = ROOT / "site" / "catalog.json"
README_PATH = ROOT / "README.md"
AGENTS_PATH = ROOT / "AGENTS.md"
INDEX_PATH = ROOT / "site" / "index.html"

BEGIN = "<!-- f00-catalog:products:begin -->"
END = "<!-- f00-catalog:products:end -->"


def load_catalog() -> dict:
    data = json.loads(CATALOG_PATH.read_text(encoding="utf-8"))
    if not isinstance(data.get("products"), list):
        raise SystemExit("catalog.json: missing products[]")
    if not data.get("theme", {}).get("css"):
        raise SystemExit("catalog.json: missing theme.css")
    return data


def released(products: list[dict]) -> list[dict]:
    return [p for p in products if p.get("status") == "released"]


def escape_attr(s: str) -> str:
    return (
        s.replace("&", "&amp;")
        .replace('"', "&quot;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
    )


def card_html(p: dict) -> str:
    domain = p.get("domain") or ""
    name = p.get("name") or p.get("id") or ""
    blurb = p.get("blurb") or p.get("one_liner") or ""
    facts = p.get("facts") or []
    site = p.get("site") or "#"
    repo = p.get("repo") or "#"
    fact_items = "\n".join(f"            <li>{f}</li>" for f in facts)
    facts_block = (
        f"""
          <ul class="facts mono">
{fact_items}
          </ul>"""
        if facts
        else ""
    )
    return f"""        <article class="card" data-product="{escape_attr(p.get('id', ''))}">
          <div class="card-meta mono">{escape_attr(domain)}</div>
          <h3>{escape_attr(name)}</h3>
          <p>
            {blurb}
          </p>{facts_block}
          <div class="card-actions">
            <a class="btn primary sm" href="{escape_attr(site)}">site</a>
            <a class="btn ghost sm" href="{escape_attr(repo)}">repo</a>
          </div>
        </article>"""


def products_grid_html(products: list[dict]) -> str:
    cards = "\n\n".join(card_html(p) for p in released(products))
    return f"""      <div class="grid" id="product-grid" data-from-catalog="1">
{BEGIN}
{cards}
{END}
      </div>"""


def patch_index(catalog: dict) -> None:
    html = INDEX_PATH.read_text(encoding="utf-8")
    grid = products_grid_html(catalog["products"])

    # Prefer stable marker replace when present.
    if BEGIN in html and END in html:
        # Replace whole grid div (from opening div through close after END).
        pattern = re.compile(
            r'[ \t]*<div class="grid"[^>]*>\s*'
            + re.escape(BEGIN)
            + r".*?"
            + re.escape(END)
            + r"\s*</div>",
            re.S,
        )
        if not pattern.search(html):
            raise SystemExit("index.html: catalog markers found but grid pattern failed")
        html = pattern.sub(grid, html, count=1)
    else:
        pattern = re.compile(
            r'(<section class="products" id="products">.*?</header>\s*)'
            r'<div class="grid".*?</div>\s*'
            r'(</section>)',
            re.S,
        )
        if not pattern.search(html):
            raise SystemExit("index.html: could not find products .grid to replace")
        html = pattern.sub(rf"\1{grid}\n    \2", html, count=1)

    # Meta description from released names
    names = ", ".join(p["name"] for p in released(catalog["products"]))
    html = re.sub(
        r'(<meta name="description" content=")[^"]*(" />)',
        rf'\1f00: freestanding tools that feel inevitable. {names}.\2',
        html,
        count=1,
    )
    INDEX_PATH.write_text(html, encoding="utf-8")
    print(f"updated {INDEX_PATH.relative_to(ROOT)}")


def patch_readme(catalog: dict) -> None:
    rows = []
    for p in released(catalog["products"]):
        name = p["name"]
        site = p.get("site") or ""
        domain = p.get("domain") or site
        repo = p.get("repo") or ""
        slug = p.get("repo_slug") or ""
        one = p.get("one_liner") or ""
        site_cell = f"[{domain.rstrip('/')}]({site})" if site else "—"
        repo_cell = f"[{slug}]({repo})" if repo else "—"
        rows.append(f"| **{name}** | {site_cell} | {repo_cell} | {one} |")

    theme = catalog["theme"]["css"]
    table = "\n".join(
        [
            "| Product | Site | Repo | One-liner |",
            "|---------|------|------|-----------|",
            *rows,
        ]
    )
    block = f"""## Products

> **Source of truth:** [`site/catalog.json`](site/catalog.json) → https://f00.sh/catalog.json
> **Theme:** [{theme}]({theme}) (Onyx, white on black). Do not fork colors in product repos.

{table}

_After editing `catalog.json`, run `python3 scripts/sync-from-catalog.py`._
"""
    text = README_PATH.read_text(encoding="utf-8")
    text2, n = re.subn(
        r"## Products\n.*?(?=\n## )",
        block + "\n",
        text,
        count=1,
        flags=re.S,
    )
    if n != 1:
        raise SystemExit("README.md: could not find ## Products section")
    README_PATH.write_text(text2, encoding="utf-8")
    print(f"updated {README_PATH.relative_to(ROOT)}")


def patch_agents(catalog: dict) -> None:
    released_p = released(catalog["products"])
    domain_bits = ["`f00.sh` hub"]
    for p in released_p:
        if p.get("domain"):
            domain_bits.append(f"`{p['domain']}` {p['name']}")
    domain_bits.append("`dist.f00.sh` R2 packages")
    domains_line = " · ".join(domain_bits)

    theme_css = catalog["theme"]["css"]
    rows = []
    for p in catalog["products"]:
        local = p.get("local") or "—"
        site = p.get("site") or "—"
        packages = p.get("packages") or "n/a"
        if p.get("pages_project") and not p.get("packages"):
            packages = f"n/a (Pages `{p['pages_project']}`)"
        status = p.get("status") or "?"
        name = p.get("name") or p.get("id")
        path = f"`$PROJECTS/{local}`" if local != "—" else "—"
        rows.append(
            f"| {name} | `{status}` | {path} | {site} | {packages} |"
        )

    sister = f"""## Catalog (single source of truth)

**Edit only** [`site/catalog.json`](site/catalog.json) (live: https://f00.sh/catalog.json).

Then run:

```bash
python3 scripts/sync-from-catalog.py
```

That regenerates hub product cards, this table, and README products.

| Field | URL / path |
|-------|------------|
| Catalog | `site/catalog.json` → https://f00.sh/catalog.json |
| Theme CSS | `site/theme/f00-theme.css` → {theme_css} |
| Org | github.com/{catalog.get('org', 'f00-sh')} |

`$PROJECTS` is the developer machines' projects root (here: `/home/glenda/Projects`).

| Product | Status | Path (local) | Site | Packages |
|---------|--------|--------------|------|----------|
{chr(10).join(rows)}

**Card rule:** `status=released` → hub card. `wip` / other → listed here, not on f00.sh grid.
"""

    text = AGENTS_PATH.read_text(encoding="utf-8")

    # product law 2 — point at catalog
    text = re.sub(
        r"2\. \*\*Release → card\.\*\*.*",
        "2. **Release → card.** Set `status: \"released\"` on the product in `site/catalog.json` and run `scripts/sync-from-catalog.py`. No card until released. Update catalog blurb/facts/links when the product ships again.",
        text,
        count=1,
    )

    # domains line
    text = re.sub(
        r"5\. \*\*Domains \(ops, not site copy\):\*\*.*",
        f"5. **Domains (ops, not site copy):** {domains_line}.",
        text,
        count=1,
    )

    # aesthetic line keeps theme URL from catalog
    text = re.sub(
        r"6\. \*\*Aesthetic:\*\*.*",
        f"6. **Aesthetic:** white on black, Onyx type. Theme: {theme_css} (path `site/theme/f00-theme.css`). All product Pages must link the theme; do not redefine brand colors/fonts in product CSS. Catalog is SSOT for products + theme pointer.",
        text,
        count=1,
    )

    # replace Sister products section
    text2, n = re.subn(
        r"## Sister products\n.*?(?=\n## License\n)",
        sister + "\n",
        text,
        count=1,
        flags=re.S,
    )
    if n != 1:
        # allow already-renamed Catalog section
        text2, n = re.subn(
            r"## Catalog \(single source of truth\)\n.*?(?=\n## License\n)",
            sister + "\n",
            text,
            count=1,
            flags=re.S,
        )
    if n != 1:
        raise SystemExit("AGENTS.md: could not find Sister products / Catalog section")

    AGENTS_PATH.write_text(text2, encoding="utf-8")
    print(f"updated {AGENTS_PATH.relative_to(ROOT)}")


def main(argv: list[str] | None = None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    check = "--check" in args

    before = {
        INDEX_PATH: INDEX_PATH.read_text(encoding="utf-8"),
        README_PATH: README_PATH.read_text(encoding="utf-8"),
        AGENTS_PATH: AGENTS_PATH.read_text(encoding="utf-8"),
    }

    catalog = load_catalog()
    patch_index(catalog)
    patch_readme(catalog)
    patch_agents(catalog)

    n = len(released(catalog["products"]))
    print(f"ok — {n} released product(s), theme={catalog['theme']['css']}")

    if check:
        dirty = []
        for path, old in before.items():
            new = path.read_text(encoding="utf-8")
            if new != old:
                dirty.append(str(path.relative_to(ROOT)))
                path.write_text(old, encoding="utf-8")  # restore
        if dirty:
            print(
                "catalog out of sync; run: python3 scripts/sync-from-catalog.py",
                file=sys.stderr,
            )
            for d in dirty:
                print(f"  would change: {d}", file=sys.stderr)
            return 1
        print("check: derived files match catalog")
    return 0


if __name__ == "__main__":
    sys.exit(main())
