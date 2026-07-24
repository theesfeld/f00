# f00tils suite benchmarks

Machine-readable and markdown tables for the website and README.

| File | Role |
|------|------|
| [suite.json](suite.json) | Per-tool: command, sample output, GNU ms, f00 ms, ratio |
| [suite.md](suite.md) | Human table |

## Regenerate

```bash
cd asm
make
N=25 python3 ../scripts/gen-suite-bench.py
```

Method: warm cache, spawn-inclusive, median of `N` runs.
f00 is timed as `f00-TOOL --core …` against `/usr/bin/TOOL`.
