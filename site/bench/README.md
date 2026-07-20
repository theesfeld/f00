# f00 public benchmarks

Reproduced with [hyperfine](https://github.com/sharkdp/hyperfine) on a warm directory of **20 000** small files.

## Fairness

| Opponent | f00 flags |
|----------|-----------|
| **coreutils `ls`** | **`f00 --gnu`** (or non-TTY auto) — no icons/git chrome |
| eza / lsd | product mode as noted (`--icons`, etc.) |

Colors forced off (`--color=never`) for all tools.

## Reproduce

```bash
cargo build -p f00 --release
# install eza, lsd, hyperfine; use real coreutils (not an f00 symlink named ls)
./scripts/bench-compare.sh 20000
```

Numbers on the site were measured on Linux x86_64 with coreutils 9.11, eza 0.23, lsd 1.2, f00 0.11.
