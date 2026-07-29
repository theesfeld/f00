#!/usr/bin/env python3
"""Enroll a new product into the f00 catalog + optional product-site theme wiring.

Usage:
  python3 scripts/enroll-product.py \\
    --id mytool \\
    --name "My Tool" \\
    --one-liner "Short description" \\
    [--status wip|released] \\
    [--domain mytool.f00.sh] \\
    [--repo-path /path/to/local/repo] \\
    [--no-sync] \\
    [--no-theme-link]

Adds/updates site/catalog.json, runs sync-from-catalog.py, and if --repo-path
points at a site with index.html, injects the global theme <link>.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "site" / "catalog.json"
THEME_HREF = "https://f00.sh/theme/f00-theme-8.css"
ORG = "f00-sh"


def load() -> dict:
    return json.loads(CATALOG.read_text(encoding="utf-8"))


def save(data: dict) -> None:
    CATALOG.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def ensure_product(data: dict, args: argparse.Namespace) -> dict:
    products = data.setdefault("products", [])
    existing = None
    for p in products:
        if p.get("id") == args.id:
            existing = p
            break

    site = None
    if args.domain:
        site = f"https://{args.domain.rstrip('/')}/"
    elif args.site:
        site = args.site

    entry = existing or {}
    entry.update(
        {
            "id": args.id,
            "name": args.name or args.id,
            "status": args.status,
            "domain": args.domain,
            "site": site,
            "repo": args.repo or f"https://github.com/{ORG}/{args.id}",
            "repo_slug": args.repo_slug or f"{ORG}/{args.id}",
            "pages_project": args.pages_project,
            "packages": args.packages,
            "local": args.local or args.id,
            "license": args.license or "MIT",
            "one_liner": args.one_liner or args.name or args.id,
            "blurb": args.blurb,
            "facts": entry.get("facts") or [],
        }
    )
    if existing is None:
        products.append(entry)
        print(f"added product {args.id} status={args.status}")
    else:
        print(f"updated product {args.id} status={args.status}")
    return entry


def inject_theme_link(repo: Path) -> list[str]:
    changed: list[str] = []
    candidates = [
        repo / "site" / "index.html",
        repo / "index.html",
    ]
    for html_path in candidates:
        if not html_path.is_file():
            continue
        text = html_path.read_text(encoding="utf-8")
        # normalize any prior theme URL to current standard
        import re as _re
        text2 = _re.sub(
            r"https://f00\.sh/theme/f00-theme[^\"']*",
            THEME_HREF,
            text,
        )
        if text2 != text:
            text = text2
            html_path.write_text(text, encoding="utf-8")
            changed.append(str(html_path))
            print(f"normalized theme URL in {html_path}")
        if "f00-theme" in text and THEME_HREF in text:
            print(f"theme already linked: {html_path}")
            continue
        # Prefer insert before first local stylesheet or before </head>
        link = f'  <link rel="stylesheet" href="{THEME_HREF}" />\n'
        if re.search(r'<link[^>]+rel=["\']stylesheet["\']', text, re.I):
            text2 = re.sub(
                r'(<link[^>]+rel=["\']stylesheet["\'][^>]*>)',
                link + r"\1",
                text,
                count=1,
                flags=re.I,
            )
        elif "</head>" in text.lower():
            text2 = re.sub(r"(</head>)", link + r"\1", text, count=1, flags=re.I)
        else:
            print(f"skip theme inject (no head/stylesheet): {html_path}", file=sys.stderr)
            continue
        if text2 != text:
            html_path.write_text(text2, encoding="utf-8")
            changed.append(str(html_path))
            print(f"linked theme in {html_path}")
    return changed


def patch_agents_note(repo: Path) -> None:
    agents = repo / "AGENTS.md"
    if not agents.is_file():
        return
    text = agents.read_text(encoding="utf-8")
    if "f00.sh/theme/f00-theme.css" in text and "catalog.json" in text:
        return
    block = (
        "\n## f00 membership\n\n"
        "- Org: `f00-sh`\n"
        "- Catalog SSOT: https://f00.sh/catalog.json (`f00` repo `site/catalog.json`)\n"
        f"- Theme: {THEME_HREF} (do not redefine brand colors/fonts)\n"
        "- Card on hub only when catalog `status=released` after a real release\n"
    )
    if "## f00 membership" not in text:
        agents.write_text(text.rstrip() + "\n" + block + "\n", encoding="utf-8")
        print(f"appended f00 membership note to {agents}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--id", required=True, help="Product id / repo name (slug)")
    ap.add_argument("--name", default=None, help="Display name")
    ap.add_argument("--one-liner", default=None)
    ap.add_argument("--blurb", default=None)
    ap.add_argument("--status", default="wip", choices=("wip", "released", "infra"))
    ap.add_argument("--domain", default=None, help="e.g. mytool.f00.sh")
    ap.add_argument("--site", default=None)
    ap.add_argument("--repo", default=None)
    ap.add_argument("--repo-slug", default=None)
    ap.add_argument("--pages-project", default=None)
    ap.add_argument("--packages", default=None)
    ap.add_argument("--local", default=None, help="Local folder name under $PROJECTS")
    ap.add_argument("--license", default="MIT")
    ap.add_argument("--repo-path", default=None, help="Local product repo to theme-link")
    ap.add_argument("--no-sync", action="store_true")
    ap.add_argument("--no-theme-link", action="store_true")
    args = ap.parse_args()

    if args.status == "infra":
        print("use catalog infra[] manually for pure infra repos", file=sys.stderr)
        return 2

    data = load()
    ensure_product(data, args)
    save(data)

    if not args.no_sync:
        sync = ROOT / "scripts" / "sync-from-catalog.py"
        subprocess.check_call([sys.executable, str(sync)], cwd=str(ROOT))

    if args.repo_path and not args.no_theme_link:
        repo = Path(args.repo_path).expanduser().resolve()
        inject_theme_link(repo)
        patch_agents_note(repo)

    print("enroll complete — commit f00 hub (catalog + derived) when ready")
    return 0


if __name__ == "__main__":
    sys.exit(main())
