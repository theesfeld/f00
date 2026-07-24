# f00tils — assembly suite

**f00tils** is the freestanding assembly coreutils replacement (coreutils → f00tils).

Multicall pure-assembly tools (x86-64 Linux, freestanding, no libc). Binary name: `f00`.

## Build

```bash
make            # builds f00 + f00-* links
make smoke      # quick functional check
make parity     # GNU --core parity samples
make speed      # median wall-time gate vs coreutils
make ux-check   # speed-gate + benches/parity.sh
./f00-ls --version
./f00 --help
```

Produces `f00` and multicall links for every util in the Makefile `UTILS` list.

Requires `nasm` and `ld`.

### Modes

- **Modern (default on TTY):** colorized output, richer layout, maximal `--json`.
- **`--core`:** plain GNU coreutils-compatible presentation (no color).

### Date / timezone

Freestanding build always uses **UTC** (no libc TZ/locale). Under `--core`, match `LC_ALL=C date -u`.

## Install (local)

```bash
make install
# or
F00_LOCAL=$PWD bash ../install.sh
```

## Man pages

Pages live under `man/man1/` (`f00.1`, `f00-ls.1`, …).

```bash
./man/gen-manpages.sh
FORCE=1 ./man/gen-manpages.sh   # refresh stubs
```

## Benchmarks

```bash
make speed
N=25 python3 ../scripts/gen-suite-bench.py
```

Suite JSON for the website: `../site/bench/suite.json`.
