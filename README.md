# f00

**f00** is a **complete GNU coreutils replacement monorepo**, implemented as a **pure x86-64 Linux freestanding assembly** multicall suite. No libc. One static binary. MIT.

| | |
|---|---|
| **Product** | Drop-in `f00-*` tools + optional short names (`ls`, `cat`, …) |
| **Default** | Modern (color, richer layout, `--json` / `--csv`) |
| **Scripts** | `--core` — strict coreutils-compatible presentation |
| **Engine** | Pure ASM multicall · ~600K static |
| **License** | MIT |
| **Status** | **Beta** `v0.15.0-beta.1` |
| **Site** | [https://f00.sh](https://f00.sh) |
| **Repo** | [github.com/theesfeld/f00](https://github.com/theesfeld/f00) |

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

---

## Product laws

1. **Clone first.** Every GNU coreutils tool has a `f00-*` counterpart. Under **`--core`**, flags, inputs, outputs, and exit codes target 1:1 coreutils behavior for scripts.
2. **Modern on top.** Default mode is never a subset of GNU: color on TTY, better layout, `--json` / `--csv` with rich metadata, interactivity where it fits.
3. **Faster always.** Freestanding ASM must beat coreutils on the core path. Slow and correct is not done.
4. **One binary.** Multicall dispatch by `argv0` (`f00-ls`, `ls`, `f00-cat`, …).

---

## Feature parity (ecosystem)

| Area | GNU coreutils | **f00 (ASM)** | uutils (Rust) | busybox | toybox |
|------|---------------|---------------|---------------|---------|--------|
| All coreutils *names* | Yes | **Scoreboard below** | Growing | Subset | Subset |
| Script drop-in | Yes | **`--core`** | Flags vary | Reduced | Reduced |
| Modern default UX | No | **Yes** | Partial | Minimal | Minimal |
| Suite-wide `--json`/`--csv` | No | **Yes (f00/v1)** | Limited | No | No |
| Pure freestanding ASM | No | **Yes** | No | C | C |
| Multicall single binary | No* | **Yes** | Optional | Yes | Yes |

\*coreutils ships many separate binaries.

### Suite modern surface (every util)

| Capability | Default | `--core` |
|------------|---------|----------|
| Color (TTY) | **On** (respects `NO_COLOR`) | Off |
| `--json` | Rich `f00/v1` metadata | Available |
| `--csv` | Same facts, tabular | Available |
| Help | Coreutils flags + Modern flags | Same structure |
| Speed | Optimized | **Must beat coreutils** |

---

## Coreutils replacement progress

**Goal: replace every GNU coreutils program.** This table is the scoreboard.

<!-- progress: total=106 shipped=106 core_full=22 core_partial=84 core_missing=0 -->
**Progress (goal = replace every coreutil):** **106/106** tools shipped · **`--core` depth:** 22 full · 84 partial · 0 missing

| Status | Count | Meaning |
|--------|------:|---------|
| shipped | 106/106 | Multicall name exists as `f00-*` |
| `--core` **full** | 22 | Tracked flags match for common cases |
| `--core` partial | 84 | Tool works; some GNU flags still deepening |
| `--core` **missing** | 0 | Not yet in multicall |

Legend — **speed:** `win` = measured faster under `--core`; `win*` = hash-family; `TBD` = not on formal speed-gate yet; `—` = not shipped.

| # | coreutils | f00 | shipped | `--core` depth | modern | speed vs GNU |
|--:|:----------|:----|:--------|:---------------|:-------|:-------------|
| 1 | `arch` | `f00-arch` | yes | **full** | yes | TBD |
| 2 | `b2sum` | `f00-b2sum` | yes | partial | yes | win |
| 3 | `base32` | `f00-base32` | yes | partial | yes | TBD |
| 4 | `base64` | `f00-base64` | yes | partial | yes | TBD |
| 5 | `basename` | `f00-basename` | yes | **full** | yes | win |
| 6 | `basenc` | `f00-basenc` | yes | partial | yes | TBD |
| 7 | `cat` | `f00-cat` | yes | **full** | deep | win |
| 8 | `chcon` | `f00-chcon` | yes | partial | yes | TBD |
| 9 | `chgrp` | `f00-chgrp` | yes | partial | yes | TBD |
| 10 | `chmod` | `f00-chmod` | yes | partial | yes | TBD |
| 11 | `chown` | `f00-chown` | yes | partial | yes | TBD |
| 12 | `chroot` | `f00-chroot` | yes | partial | yes | TBD |
| 13 | `cksum` | `f00-cksum` | yes | partial | yes | TBD |
| 14 | `comm` | `f00-comm` | yes | partial | yes | TBD |
| 15 | `cp` | `f00-cp` | yes | partial | yes | TBD |
| 16 | `csplit` | `f00-csplit` | yes | partial | yes | TBD |
| 17 | `cut` | `f00-cut` | yes | partial | yes | TBD |
| 18 | `date` | `f00-date` | yes | partial | yes | TBD |
| 19 | `dd` | `f00-dd` | yes | partial | yes | TBD |
| 20 | `df` | `f00-df` | yes | partial | yes | TBD |
| 21 | `dir` | `f00-dir` | yes | partial | yes | TBD |
| 22 | `dircolors` | `f00-dircolors` | yes | partial | yes | TBD |
| 23 | `dirname` | `f00-dirname` | yes | **full** | yes | TBD |
| 24 | `du` | `f00-du` | yes | partial | yes | TBD |
| 25 | `echo` | `f00-echo` | yes | **full** | yes | TBD |
| 26 | `env` | `f00-env` | yes | partial | yes | TBD |
| 27 | `expand` | `f00-expand` | yes | partial | yes | TBD |
| 28 | `expr` | `f00-expr` | yes | **full** | yes | TBD |
| 29 | `factor` | `f00-factor` | yes | partial | yes | TBD |
| 30 | `false` | `f00-false` | yes | **full** | yes | TBD |
| 31 | `fmt` | `f00-fmt` | yes | partial | yes | TBD |
| 32 | `fold` | `f00-fold` | yes | partial | yes | TBD |
| 33 | `groups` | `f00-groups` | yes | **full** | yes | TBD |
| 34 | `head` | `f00-head` | yes | partial | yes | win |
| 35 | `hostid` | `f00-hostid` | yes | **full** | yes | TBD |
| 36 | `id` | `f00-id` | yes | partial | yes | win |
| 37 | `install` | `f00-install` | yes | **full** | yes | TBD |
| 38 | `join` | `f00-join` | yes | partial | yes | TBD |
| 39 | `link` | `f00-link` | yes | partial | yes | TBD |
| 40 | `ln` | `f00-ln` | yes | partial | yes | TBD |
| 41 | `logname` | `f00-logname` | yes | **full** | yes | TBD |
| 42 | `ls` | `f00-ls` | yes | partial | deep | win |
| 43 | `md5sum` | `f00-md5sum` | yes | partial | yes | win |
| 44 | `mkdir` | `f00-mkdir` | yes | partial | yes | TBD |
| 45 | `mkfifo` | `f00-mkfifo` | yes | partial | yes | TBD |
| 46 | `mknod` | `f00-mknod` | yes | partial | yes | TBD |
| 47 | `mktemp` | `f00-mktemp` | yes | partial | yes | TBD |
| 48 | `mv` | `f00-mv` | yes | partial | yes | TBD |
| 49 | `nice` | `f00-nice` | yes | partial | yes | TBD |
| 50 | `nl` | `f00-nl` | yes | partial | yes | TBD |
| 51 | `nohup` | `f00-nohup` | yes | partial | yes | TBD |
| 52 | `nproc` | `f00-nproc` | yes | **full** | yes | win |
| 53 | `numfmt` | `f00-numfmt` | yes | partial | yes | TBD |
| 54 | `od` | `f00-od` | yes | partial | yes | TBD |
| 55 | `paste` | `f00-paste` | yes | partial | yes | TBD |
| 56 | `pathchk` | `f00-pathchk` | yes | partial | yes | TBD |
| 57 | `pinky` | `f00-pinky` | yes | partial | yes | TBD |
| 58 | `pr` | `f00-pr` | yes | partial | yes | TBD |
| 59 | `printenv` | `f00-printenv` | yes | **full** | yes | TBD |
| 60 | `printf` | `f00-printf` | yes | **full** | yes | TBD |
| 61 | `ptx` | `f00-ptx` | yes | partial | yes | TBD |
| 62 | `pwd` | `f00-pwd` | yes | **full** | yes | TBD |
| 63 | `readlink` | `f00-readlink` | yes | partial | yes | TBD |
| 64 | `realpath` | `f00-realpath` | yes | partial | yes | win |
| 65 | `rm` | `f00-rm` | yes | partial | yes | TBD |
| 66 | `rmdir` | `f00-rmdir` | yes | **full** | yes | TBD |
| 67 | `runcon` | `f00-runcon` | yes | partial | yes | TBD |
| 68 | `seq` | `f00-seq` | yes | partial | yes | win |
| 69 | `sha1sum` | `f00-sha1sum` | yes | partial | yes | win* |
| 70 | `sha224sum` | `f00-sha224sum` | yes | partial | yes | win* |
| 71 | `sha256sum` | `f00-sha256sum` | yes | partial | yes | win |
| 72 | `sha384sum` | `f00-sha384sum` | yes | partial | yes | win* |
| 73 | `sha512sum` | `f00-sha512sum` | yes | partial | yes | win* |
| 74 | `shred` | `f00-shred` | yes | partial | yes | TBD |
| 75 | `shuf` | `f00-shuf` | yes | partial | yes | TBD |
| 76 | `sleep` | `f00-sleep` | yes | **full** | yes | TBD |
| 77 | `sort` | `f00-sort` | yes | partial | yes | win |
| 78 | `split` | `f00-split` | yes | partial | yes | TBD |
| 79 | `stat` | `f00-stat` | yes | partial | yes | TBD |
| 80 | `stdbuf` | `f00-stdbuf` | yes | partial | yes | TBD |
| 81 | `stty` | `f00-stty` | yes | partial | yes | TBD |
| 82 | `sum` | `f00-sum` | yes | partial | yes | TBD |
| 83 | `sync` | `f00-sync` | yes | partial | yes | TBD |
| 84 | `tac` | `f00-tac` | yes | partial | yes | TBD |
| 85 | `tail` | `f00-tail` | yes | partial | yes | win |
| 86 | `tee` | `f00-tee` | yes | partial | yes | TBD |
| 87 | `test` | `f00-test` | yes | **full** | yes | TBD |
| 88 | `timeout` | `f00-timeout` | yes | partial | yes | TBD |
| 89 | `touch` | `f00-touch` | yes | partial | yes | TBD |
| 90 | `tr` | `f00-tr` | yes | partial | yes | TBD |
| 91 | `true` | `f00-true` | yes | **full** | yes | win |
| 92 | `truncate` | `f00-truncate` | yes | partial | yes | TBD |
| 93 | `tsort` | `f00-tsort` | yes | partial | yes | TBD |
| 94 | `tty` | `f00-tty` | yes | **full** | yes | TBD |
| 95 | `uname` | `f00-uname` | yes | partial | yes | win |
| 96 | `unexpand` | `f00-unexpand` | yes | partial | yes | TBD |
| 97 | `uniq` | `f00-uniq` | yes | partial | yes | TBD |
| 98 | `unlink` | `f00-unlink` | yes | partial | yes | TBD |
| 99 | `uptime` | `f00-uptime` | yes | partial | yes | TBD |
| 100 | `users` | `f00-users` | yes | partial | yes | TBD |
| 101 | `vdir` | `f00-vdir` | yes | partial | yes | TBD |
| 102 | `wc` | `f00-wc` | yes | partial | yes | win |
| 103 | `who` | `f00-who` | yes | partial | yes | TBD |
| 104 | `whoami` | `f00-whoami` | yes | **full** | yes | TBD |
| 105 | `yes` | `f00-yes` | yes | **full** | yes | TBD |
| 106 | `[` | `f00-[ / test` | yes | partial | yes | TBD |

Also shipped (useful multicall extras; not always in the coreutils package): `f00-hostname`, `f00-kill`, `f00-rev`.

Detailed per-flag matrix: [docs/GNU-COMPLIANCE.md](docs/GNU-COMPLIANCE.md) · scoreboard source: [docs/COREUTILS-PROGRESS.md](docs/COREUTILS-PROGRESS.md)


## Speed parity

Warm cache, **spawn-inclusive**, median of 40 runs, `f00-* --core` vs `/usr/bin/*` on Linux x86-64 (representative host; re-run `make speed`).

| Workload | coreutils | **f00 `--core`** | vs coreutils | Notes |
|----------|-----------|------------------|--------------|--------|
| `true` | 0.22 ms | **0.07 ms** | **~3.1×** | Multicall entry |
| `basename` | 0.24 ms | **0.07 ms** | **~3.2×** | |
| `wc -l` | 0.51 ms | **0.24 ms** | **~2.1×** | |
| `cat` (small file) | 0.27 ms | **0.19 ms** | **~1.4×** | Bulk path |
| `ls -1` | 0.29 ms | **0.21 ms** | **~1.4×** | |
| `ls -la` | 0.99 ms | **0.23 ms** | **~4.3×** | Large win |
| `md5sum` | 0.91 ms | **0.39 ms** | **~2.3×** | Pure ASM MD5 |
| `seq 1…` | 0.24 ms | **0.13 ms** | **~1.8×** | |
| `nproc` | 0.32 ms | **0.08 ms** | **~3.9×** | |
| `id` | 1.33 ms | **0.13 ms** | **~10×** | |

| Competitor class | Typical profile | f00 stance |
|------------------|-----------------|------------|
| **GNU coreutils** | libc, portable C | Beat on freestanding hot paths |
| **uutils/coreutils** | Rust, safe/portable | f00 targets lower latency, not portability breadth |
| **busybox / toybox** | Small embedded applets | f00 targets full desktop/server flag surface + modern UX |
| **Single-tool rewrites** (eza, bat, …) | Deep one-tool UX | f00 ships **full suite** + deep `f00-ls` / `f00-cat` |

Reproduce:

```bash
cd asm && make && make speed      # speed-gate vs coreutils
bash benches/parity.sh            # functional --core diffs
```

---

## Install

### One-liner (recommended)

```bash
curl -fsSL https://f00.sh/install.sh | bash
```

Installs multicall `f00` + all `f00-*` links into `~/.local/bin` (override with `INSTALL_DIR`).

| Env | Effect |
|-----|--------|
| `INSTALL_DIR` | Target bin dir (default `~/.local/bin`) |
| `F00_VERSION` | Release tag (default: latest; beta: `v0.15.0-beta.1`) |
| `F00_LOCAL` | Path to local `asm/` build containing `./f00` (skip download) |
| `F00_TOOLS` | `all` or comma list |
| `F00_SUPERSEDE=1` | Also install short names (`ls`, `cat`, …) in `INSTALL_DIR` |
| `F00_ALIAS=1` | Append shell aliases |
| `F00_MAN=1` | Install man pages (default on) |

```bash
# beta pin
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.15.0-beta.1 bash

# from a local build
curl -fsSL https://f00.sh/install.sh | F00_LOCAL=$PWD/asm bash

# side-by-side + short names
curl -fsSL https://f00.sh/install.sh | F00_SUPERSEDE=1 bash
```

### From source

```bash
git clone https://github.com/theesfeld/f00.git
cd f00/asm
make            # needs nasm + ld
make smoke
make install    # ~/.local/bin + man pages
```

Requires: `nasm`, `ld` (binutils). Target: **Linux x86-64**.

---

## Package managers

> **Beta note:** package recipes are migrating from the historical Rust `ls` product to the **ASM multicall suite**. Prefer the install script or source build for `v0.15.0-beta.1`. Distro packages will track stable tags.

| Channel | Status | Command / notes |
|---------|--------|-----------------|
| **Install script** | **Primary** | `curl -fsSL https://f00.sh/install.sh \| bash` |
| **From source** | Supported | `cd asm && make install` |
| **AUR** | Updating | `packaging/aur/PKGBUILD` — rebuild for ASM static binary |
| **Homebrew** | Updating | `Formula/f00.rb` — tap recipe (Linux bottle TBD) |
| **nfpm (deb/rpm)** | Updating | `packaging/nfpm/f00.yaml` |
| **Scoop / Winget** | Windows later | Manifests present; ASM product is Linux-first |
| **Nix** | Experimental | `flake.nix` |

Arch (after PKGBUILD points at ASM release assets):

```bash
# AUR helper example (when published)
yay -S f00
```

Debian/Fedora (when release artifacts ship):

```bash
# illustrative — use published .deb / .rpm from GitHub Releases
sudo dpkg -i f00_*_amd64.deb
# or
sudo rpm -Uvh f00-*.x86_64.rpm
```

---

## Beta `v0.15.0-beta.1`

| | |
|---|---|
| **Tag** | `v0.15.0-beta.1` |
| **What** | Full multicall coreutils *surface* in pure ASM; modern UX; speed-gate; parity harness |
| **Not yet** | Every obscure GNU long-option on every util (tracked in [docs/GNU-COMPLIANCE.md](docs/GNU-COMPLIANCE.md)); non-x86_64; macOS Darwin layer |
| **Feedback** | [GitHub Issues](https://github.com/theesfeld/f00/issues) |

```bash
curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.15.0-beta.1 bash
# or
git fetch --tags && git checkout v0.15.0-beta.1 && cd asm && make
```

---

## Quick start

```bash
f00-ls -la
f00-ls --core -la          # script-safe
f00-cat -n README.md       # modern line numbers on TTY
f00-wc --json Makefile
f00-sha256sum --core file
f00-df -h                  # modern table
f00-id --core              # match GNU id
f00 --list-utils           # when argv0 is f00
```

---

## Layout

```
asm/                 pure assembly product (canonical)
  src/ls/            multicall sources + suite_*.asm modules
  man/man1/          f00(1) + f00-*(1)
  benches/           speed-gate, parity, smoke
site/                f00.sh (GitHub Pages) + install.sh
docs/                compliance, UX, modern features
packaging/           AUR, nfpm, scoop, winget
crates/              historical Rust f00-ls (reference only)
```

---

## Documentation

| Doc | |
|-----|--|
| [docs/COREUTILS-PROGRESS.md](docs/COREUTILS-PROGRESS.md) | **Scoreboard — every coreutil** (shipped / `--core` depth / modern / speed) |
| [docs/GNU-COMPLIANCE.md](docs/GNU-COMPLIANCE.md) | Per-flag full / partial / missing |
| [docs/TERMINAL-UX.md](docs/TERMINAL-UX.md) | Color tokens, help structure, JSON envelope |
| [docs/MODERN-FEATURES.md](docs/MODERN-FEATURES.md) | Modern extras survey |
| [CHANGELOG.md](CHANGELOG.md) | Releases |
| Man | `man f00` · `man f00-ls` · `man f00-cat` · … |

---

## Build & quality gates

```bash
cd asm
make              # f00 + f00-* links
make smoke        # functional smoke
make speed        # must beat coreutils (+5% gate)
make ux-check     # speed + parity
```

---

## License

MIT — see [LICENSE](LICENSE).
