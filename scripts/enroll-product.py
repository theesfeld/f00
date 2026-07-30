#!/usr/bin/env python3
"""Enroll a new project into the f00 catalog + optional project-site theme wiring.

Usage:
  python3 scripts/enroll-project.py \\
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
COC_CANONICAL = "https://f00.sh/CODE_OF_CONDUCT.md"
COC_LOCAL = ROOT / "site" / "CODE_OF_CONDUCT.md"
ORG = "f00-sh"
DEFAULT_THEME = "https://f00.sh/theme/f00-theme.css"
DEFAULT_ENTROPY = "https://f00.sh/theme/f00-entropy.js?v=21"


def theme_urls() -> tuple[str, str]:
    data = json.loads(CATALOG.read_text(encoding="utf-8"))
    t = data.get("theme") or {}
    css = t.get("css_live") or t.get("css") or t.get("css_canonical") or DEFAULT_THEME
    # prefer unversioned live file so hub updates hit all sites
    if re.search(r"f00-theme-\d+\.css", css):
        css = DEFAULT_THEME
    entropy = t.get("entropy") or DEFAULT_ENTROPY
    return css, entropy


def load() -> dict:
    return json.loads(CATALOG.read_text(encoding="utf-8"))


def save(data: dict) -> None:
    CATALOG.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def ensure_project(data: dict, args: argparse.Namespace) -> dict:
    projects = data.setdefault("projects", [])
    existing = None
    for p in projects:
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
            "docs": args.docs,
            "blurb": args.blurb,
            "facts": entry.get("facts") or [],
        }
    )
    if existing is None:
        projects.append(entry)
        print(f"added project {args.id} status={args.status}")
    else:
        print(f"updated project {args.id} status={args.status}")
    return entry


def inject_theme_link(repo: Path) -> list[str]:
    """Pin project HTML to central theme + entropy (catalog SSOT)."""
    theme_href, entropy_href = theme_urls()
    changed: list[str] = []
    candidates = [
        repo / "site" / "index.html",
        repo / "index.html",
        repo / "dist" / "index.html",
    ]
    for html_path in candidates:
        if not html_path.is_file():
            continue
        text = html_path.read_text(encoding="utf-8")
        orig = text
        text = re.sub(
            r"https://f00\.sh/theme/f00-theme[^\"'\s>]*",
            theme_href,
            text,
        )
        text = re.sub(
            r"https://f00\.sh/theme/f00-entropy\.js(?:\?v=[^\"'\s>]*)?",
            entropy_href,
            text,
        )
        if theme_href not in text:
            link = f'  <link rel="stylesheet" href="{theme_href}" data-f00-theme="1" />\n'
            if re.search(r'<link[^>]+rel=["\']stylesheet["\']', text, re.I):
                text = re.sub(
                    r'(<link[^>]+rel=["\']stylesheet["\'][^>]*>)',
                    link + r"\1",
                    text,
                    count=1,
                    flags=re.I,
                )
            elif re.search(r"</head>", text, re.I):
                text = re.sub(r"(</head>)", link + r"\1", text, count=1, flags=re.I)
        if "data-f00-entropy-script" not in text:
            tag = (
                f'  <script src="{entropy_href}" data-f00-entropy-script defer>'
                f"</script>\n"
            )
            if re.search(r"</body>", text, re.I):
                text = re.sub(r"(</body>)", tag + r"\1", text, count=1, flags=re.I)
            elif re.search(r"</head>", text, re.I):
                text = re.sub(r"(</head>)", tag + r"\1", text, count=1, flags=re.I)
        if text != orig:
            html_path.write_text(text, encoding="utf-8")
            changed.append(str(html_path))
            print(f"pinned theme+entropy: {html_path}")
        else:
            print(f"theme already pinned: {html_path}")
    return changed


def install_code_of_conduct(repo: Path) -> None:
    """Copy org CoC into the project root (SSOT: site/CODE_OF_CONDUCT.md)."""
    if not COC_LOCAL.is_file():
        print(f"skip CoC install (missing {COC_LOCAL})", file=sys.stderr)
        return
    dest = repo / "CODE_OF_CONDUCT.md"
    dest.write_text(COC_LOCAL.read_text(encoding="utf-8"), encoding="utf-8")
    print(f"wrote {dest} (from {COC_CANONICAL})")


def patch_agents_note(repo: Path) -> None:
    agents = repo / "AGENTS.md"
    if not agents.is_file():
        return
    text = agents.read_text(encoding="utf-8")
    if "f00.sh/theme/f00-theme.css" in text and "catalog.json" in text:
        return
    theme_href, _entropy = theme_urls()
    block = (
        "\n## f00 membership\n\n"
        "- Org: `f00-sh`\n"
        "- Catalog SSOT: https://f00.sh/catalog.json (`f00` repo `site/catalog.json`)\n"
        f"- Theme (live SSOT): {theme_href} — do not redefine brand colors/fonts; "
        "project CSS is layout-only. Edge Worker pins this on all `*.f00.sh`.\n"
        f"- Code of Conduct: {COC_CANONICAL} (pull via f00 `scripts/pull-code-of-conduct.sh`)\n"
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
    ap.add_argument("--docs", default=None, help="Docs URL (project site #docs or methodology)")
    ap.add_argument("--repo-path", default=None, help="Local project repo to theme-link")
    ap.add_argument("--no-sync", action="store_true")
    ap.add_argument("--no-theme-link", action="store_true")
    args = ap.parse_args()

    if args.status == "infra":
        print("use catalog infra[] manually for pure infra repos", file=sys.stderr)
        return 2

    data = load()
    ensure_project(data, args)
    save(data)

    if not args.no_sync:
        sync = ROOT / "scripts" / "sync-from-catalog.py"
        subprocess.check_call([sys.executable, str(sync)], cwd=str(ROOT))

    if args.repo_path:
        repo = Path(args.repo_path).expanduser().resolve()
        install_code_of_conduct(repo)
        if not args.no_theme_link:
            inject_theme_link(repo)
        patch_agents_note(repo)

    print("enroll complete — commit f00 hub (catalog + derived) when ready")
    return 0


if __name__ == "__main__":
    sys.exit(main())
