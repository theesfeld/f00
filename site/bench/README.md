# f00tils suite benchmarks

Machine-readable and markdown tables for the website and README.

| File | Role |
|------|------|
| [suite.json](suite.json) | Per-tool times + **overall summary**, showcase, cold-startup series |
| [suite.md](suite.md) | Human table + overall headline |

## Overall headline

`summary.headline` / `summary.headline_x` / `summary.pct_faster_geo` are the single source of truth:

- **×** = geometric mean of per-tool speedups (`f00-* --core` vs GNU)
- **% faster** = `(geo − 1) × 100`
- Stamped into **README**, **file_id.diz**, and the website hero / race charts

## Regenerate

```bash
cd asm
make
N=25 python3 ../scripts/gen-suite-bench.py
```

Method: warm cache, spawn-inclusive, median of `N` runs.
f00 is timed as `f00-TOOL --core …` against `/usr/bin/TOOL`.
Cold-start panel stores raw sample series for entry tools.
