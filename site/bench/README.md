# f00 public benchmarks

Reproduced with [hyperfine](https://github.com/sharkdp/hyperfine) on a warm directory of **20 000** small files.

## Two tracks (do not mix)

| Track | Opponent | f00 command |
|-------|----------|-------------|
| **A · drop-in** | coreutils `ls` | **`f00 --gnu`** only |
| **B · modern** | eza / lsd | default **`f00`** (no `--gnu`) |

Colors forced off (`--color=never`) for all tools.

## Reproduce

```bash
cargo build -p f00 --release
# real coreutils binary (not an f00 symlink named ls)
# eza, lsd, hyperfine on PATH
./scripts/bench-compare.sh 20000
```

Numbers on the site: Linux x86_64 · coreutils 9.11 · eza 0.23 · lsd 1.2 · f00 0.12.
