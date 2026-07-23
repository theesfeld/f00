# f00 v0.15.0

**Pure assembly GNU coreutils monorepo — first public beta of the multicall freestanding suite.**

## Install

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.15.0 bash
# or from source
git checkout v0.15.0 && cd asm && make && make install
# or local archive
tar xzf f00-0.15.0-x86_64-linux.tar.gz
cd f00-0.15.0-x86_64-linux && ./install-local.sh
```

**Asset:** `f00-0.15.0-x86_64-linux.tar.gz`  
**SHA256:** `3d8c375a388f9a6c68d3d74abaa7b97bdb4da3935bfa621e267d0b2dfabb6606`

## Highlights

- Full multicall **coreutils surface** (106/106 GNU names) as one static Linux x86-64 binary (~600K)
- **`--core`** for script-safe coreutils presentation
- **Modern default**: color, better layout, **`--json` / `--csv`** (f00/v1 metadata)
- **Speed-gate**: must beat coreutils on the core path (`make speed`)
- Man pages: `man f00`, `man f00-ls`, …
- Progress scoreboard: [COREUTILS-PROGRESS.md](https://github.com/theesfeld/f00/blob/main/docs/COREUTILS-PROGRESS.md) · site [f00.sh](https://f00.sh)

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
