#!/usr/bin/env python3
"""Pin every catalog project's site HTML to the central f00 theme + entropy.

SSOT: site/catalog.json → theme.css / theme.css_canonical
Live edge also rewrites via workers/theme-inject (*.f00.sh).

Usage (from f00 repo):
  python3 scripts/pin-theme-all-projects.py
  python3 scripts/pin-theme-all-projects.py --projects-root /home/glenda/Projects
  python3 scripts/pin-theme-all-projects.py --dry-run
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "site" / "catalog.json"

DEFAULT_THEME = "https://f00.sh/theme/f00-theme.css"
DEFAULT_ENTROPY = "https://f00.sh/theme/f00-entropy.js?v=24"

THEME_HREF_RE = re.compile(
    r"https?://f00\.sh/theme/f00-theme(?:-\d+)?\.css(?:\?[^\"'\s>]*)?"
    r"|/theme/f00-theme(?:-\d+)?\.css(?:\?[^\"'\s>]*)?",
    re.I,
)
ENTROPY_HREF_RE = re.compile(
    r"https?://f00\.sh/theme/f00-entropy\.js(?:\?v=[^\"'\s>]*)?",
    re.I,
)
HB_SHELL_RE = re.compile(
    r"https?://f00\.sh/theme/(?:pack/|textures/)?hb-shell[^\"'\s>]*\.css",
    re.I,
)


def load_theme_urls() -> tuple[str, str]:
    data = json.loads(CATALOG.read_text(encoding="utf-8"))
    theme = data.get("theme") or {}
    # Prefer unversioned live URL so hub file updates propagate without re-pinning
    css = (
        theme.get("css_live")
        or "https://f00.sh/theme/f00-theme.css"
        or theme.get("css")
        or DEFAULT_THEME
    )
    # Keep pin version for cache-bust when catalog still lists versioned snapshot
    if "f00-theme-" in str(theme.get("css") or "") and not theme.get("css_live"):
        # still ship unversioned live as SSOT; snapshot remains on hub for history
        css = "https://f00.sh/theme/f00-theme.css"
    entropy = theme.get("entropy") or DEFAULT_ENTROPY
    return css, entropy


def find_html(repo: Path) -> list[Path]:
    out: list[Path] = []
    for rel in ("site/index.html", "index.html", "dist/index.html"):
        p = repo / rel
        if p.is_file():
            out.append(p)
    return out


def pin_html(path: Path, theme: str, entropy: str, dry: bool) -> bool:
    text = path.read_text(encoding="utf-8")
    orig = text

    text = THEME_HREF_RE.sub(theme, text)
    text = HB_SHELL_RE.sub(theme, text)
    text = ENTROPY_HREF_RE.sub(entropy, text)

    theme_link = f'<link rel="stylesheet" href="{theme}" data-f00-theme="1" />'
    if theme not in text and "data-f00-theme" not in text:
        if re.search(r"</head>", text, re.I):
            text = re.sub(
                r"</head>",
                f"  {theme_link}\n</head>",
                text,
                count=1,
                flags=re.I,
            )
        else:
            text = theme_link + "\n" + text

    # ensure data-f00-theme on the theme link
    if theme in text and "data-f00-theme" not in text:
        text = text.replace(
            f'href="{theme}"',
            f'href="{theme}" data-f00-theme="1"',
            1,
        )

    entropy_tag = (
        f'<script src="{entropy}" data-f00-entropy-script defer></script>'
    )
    if "data-f00-entropy-script" not in text:
        if re.search(r"</body>", text, re.I):
            text = re.sub(
                r"</body>",
                f"  {entropy_tag}\n</body>",
                text,
                count=1,
                flags=re.I,
            )
        elif re.search(r"</head>", text, re.I):
            text = re.sub(
                r"</head>",
                f"  {entropy_tag}\n</head>",
                text,
                count=1,
                flags=re.I,
            )
        else:
            text += "\n" + entropy_tag + "\n"

    if text == orig:
        return False
    if not dry:
        path.write_text(text, encoding="utf-8")
    return True


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--projects-root",
        default=os.environ.get("PROJECTS", str(ROOT.parent)),
        help="Parent of project checkouts (default: $PROJECTS or sibling of f00)",
    )
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    theme, entropy = load_theme_urls()
    data = json.loads(CATALOG.read_text(encoding="utf-8"))
    projects = data.get("projects") or data.get("products") or []
    root = Path(args.projects_root).expanduser().resolve()

    print(f"theme SSOT:   {theme}")
    print(f"entropy SSOT: {entropy}")
    print(f"projects root: {root}")

    changed = 0
    missing = 0
    for p in projects:
        local = p.get("local") or p.get("id")
        if not local:
            continue
        repo = root / local
        if not repo.is_dir():
            print(f"  skip missing: {repo}")
            missing += 1
            continue
        htmls = find_html(repo)
        if not htmls:
            print(f"  skip no html: {repo}")
            continue
        for html in htmls:
            if pin_html(html, theme, entropy, args.dry_run):
                print(f"  {'would pin' if args.dry_run else 'pinned'}: {html}")
                changed += 1
            else:
                print(f"  ok: {html}")

    print(f"done — {changed} file(s) changed, {missing} missing checkouts")
    return 0


if __name__ == "__main__":
    sys.exit(main())
