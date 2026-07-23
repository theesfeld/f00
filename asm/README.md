# f00 assembly suite

Multicall pure-assembly coreutils replacements (x86-64 Linux, freestanding, no libc).

## Build

```bash
make            # builds f00 + f00-* (and short-name) links
make smoke      # quick functional check (also: make test)
make parity     # GNU --core byte-parity for id/date/uname/md5sum/sha256sum/base64/nproc
make speed      # median wall-time gate vs coreutils (f00 --core must not be >5% slower)
make ux-check   # speed-gate + benches/parity.sh
./f00-ls --version
./f00 --help
```

Produces `f00` and multicall links for every util in the `UTILS` list in the Makefile
(`f00-ls`, `f00-cat`, `f00-wc`, …).

Requires `nasm` and `ld`.

### Modes

- **Modern (default on TTY):** colorized output (e.g. id numbers, hash digests), maximal `--json`.
- **`--core`:** plain GNU coreutils-compatible presentation (no color).

### Date / timezone

Freestanding build always uses **UTC** (no libc TZ/locale). Under `--core`, match `LC_ALL=C date -u`. Default human form: `Thu Jul 23 20:12:42 UTC 2026`.

## Install (local)

```bash
make install
# or from the site installer against this tree:
# F00_LOCAL=$PWD bash ../site/install.sh
```

## Man pages

Hand-written pages for key tools live under `man/man1/` (`f00.1`, `f00-ls.1`, …).
Generate stubs for the full `UTILS` list:

```bash
./man/gen-manpages.sh
```

## Benchmarks

```bash
make speed                 # N=40 median; exit 1 if any case >5% slower
./benches/bench.sh         # informal averages
# ITERS=50 DIR=/usr/bin ./benches/bench.sh
# N=80 THRESH_PCT=5 ./benches/speed-gate.sh
```

`speed-gate.sh` compares coreutils vs `f00-* --core` for: true, basename, wc -l, cat, ls -1, ls -la, md5sum, seq, nproc, id.

Terminal UX conventions: [`../docs/TERMINAL-UX.md`](../docs/TERMINAL-UX.md).

## f00-ls

See `man/man1/f00-ls.1` and `./f00-ls --help` (Coreutils flags / Modern flags).

## License

MIT
