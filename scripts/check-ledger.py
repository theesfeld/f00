#!/usr/bin/env python3
"""Validate site/ledger.json — open books, no payroll, balance print.

Usage:
  python3 scripts/check-ledger.py
  python3 scripts/check-ledger.py --check   # exit 1 on errors
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / "site" / "ledger.json"

KINDS = {"opening", "gift", "expense", "physical_sale", "transfer", "note"}
DIRS = {"in", "out", "none"}
FORBIDDEN = re.compile(
    r"\b(payroll|salary|salaries|wage|wages|stipend|owner.?draw|dividend)\b",
    re.I,
)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true", help="exit non-zero on error")
    args = ap.parse_args()

    data = json.loads(LEDGER.read_text(encoding="utf-8"))
    errors: list[str] = []
    entries = data.get("entries")
    if not isinstance(entries, list):
        errors.append("entries must be a list")
        entries = []

    pol = data.get("policy") or {}
    if pol.get("no_payroll") is not True:
        errors.append("policy.no_payroll must be true")
    if pol.get("open") is not True:
        errors.append("policy.open must be true")

    ids: set[str] = set()
    bal: dict[str, float] = {}

    for i, e in enumerate(entries):
        if not isinstance(e, dict):
            errors.append(f"entry[{i}] not an object")
            continue
        eid = e.get("id")
        if not eid or eid in ids:
            errors.append(f"entry[{i}]: missing or duplicate id")
        else:
            ids.add(str(eid))
        kind = e.get("kind")
        if kind not in KINDS:
            errors.append(f"{eid}: bad kind {kind!r}")
        direction = e.get("direction")
        if direction not in DIRS:
            errors.append(f"{eid}: bad direction {direction!r}")
        memo = str(e.get("memo") or "")
        if FORBIDDEN.search(memo):
            errors.append(f"{eid}: memo looks like payroll/private profit")
        if FORBIDDEN.search(str(kind)):
            errors.append(f"{eid}: kind forbidden")
        try:
            amount = float(e.get("amount", 0))
        except (TypeError, ValueError):
            errors.append(f"{eid}: amount not numeric")
            continue
        if amount < 0:
            errors.append(f"{eid}: amount must be non-negative (use direction)")
        cur = str(e.get("currency") or "USD").upper()
        bal.setdefault(cur, 0.0)
        if direction == "in":
            bal[cur] += amount
        elif direction == "out":
            bal[cur] -= amount

    print(f"ledger entries: {len(entries)}")
    print("balances:")
    for c, v in sorted(bal.items()):
        print(f"  {c}: {v:.6g}")
    if errors:
        print("errors:", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1 if args.check else 0
    print("ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
