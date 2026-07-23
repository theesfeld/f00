# f00 v0.15.0

**Full-use release: pure assembly GNU coreutils monorepo.**

## Install

```bash
curl -fsSL https://f00.sh/install.sh | bash
# pin
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.15.0 bash
# source
git checkout v0.15.0 && cd asm && make && make install
```

## Highlights

- **106/106** GNU coreutils names — multicall freestanding Linux x86-64 ASM (~650K)
- **`--core`** script-safe coreutils presentation; modern default color + layout
- **`--json` / `--csv`** rich `f00/v1` metadata
- Progress scoreboard: all tools **full** core depth · modern · speed **win**
- Man pages: `man f00`, `man f00-ls`, …

## Verify

```bash
cd asm && make smoke && make speed && bash benches/parity.sh
```

## Links

- https://f00.sh
- https://github.com/theesfeld/f00
- https://github.com/theesfeld/f00/blob/main/docs/COREUTILS-PROGRESS.md

**SHA256:** `4e9420e48412e9765d30132e8dcb6829087343fcddd94bd436b6d6dfcb685b99`

