# f00 v0.15.0-beta.1

**Pure assembly GNU coreutils monorepo — first public beta of the multicall freestanding suite.**

## Install

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.15.0-beta.1 bash
# or from source
git checkout v0.15.0-beta.1 && cd asm && make && make install
```

## Highlights

- Full multicall **coreutils surface** (~107 tools) as one static Linux x86-64 binary
- **`--core`** for script-safe coreutils presentation
- **Modern default**: color, better layout, **`--json` / `--csv`** (f00/v1 metadata)
- **Speed-gate**: must beat coreutils on the core path (`make speed`)
- Man pages: `man f00`, `man f00-ls`, …

## Verify

```bash
cd asm && make smoke && make speed && bash benches/parity.sh
```

## Known beta limits

- Not every rare GNU long-option is complete on every util — see `docs/GNU-COMPLIANCE.md`
- Linux x86-64 first
- Distro packages migrating from the old Rust ls product

## Links

- https://f00.sh
- https://github.com/theesfeld/f00
