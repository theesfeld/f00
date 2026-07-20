# Migration guide

## 0.10 → 0.11

**f00 0.11** tightens defaults around the daily-driver `ls` replacement: **fast and beautiful on a TTY**, **script-safe when piped**, with a leaner binary and a separate interactive browser.

### Behavior: non-TTY is script-safe by default

When **stdout is not a TTY** (pipes, capture, CI), f00 now auto-enables **script-safe / GNU-equivalent** mode — same effect as `--gnu` for decorations and listing behavior (no icons, no git column, classic sort, no archive-as-directory, etc.).

| Context | 0.10 | 0.11 |
|---------|------|------|
| Interactive TTY | Modern chrome (icons/git auto) | Unchanged |
| Pipe / redirect | Modern git/icons could leak into scripts | **Auto script-safe** |
| Force modern on a pipe | (default) | `--no-gnu` or `F00_NO_GNU=1` |
| Force GNU always | `--gnu` / `F00_GNU=1` | Same |

```bash
# scripts / pipelines — no flag needed in 0.11
f00 -la /tmp | grep foo

# want icons/git in a pipe or capture?
f00 --no-gnu -la /tmp | cat
```

### Cargo default features

| Feature | 0.10 default | 0.11 default |
|---------|--------------|--------------|
| `git` | yes | **yes** |
| `io-uring` | yes | **yes** |
| `archives` | yes | **no** (opt-in) |
| `tui` | yes | **no** (prefer `f00-tui` binary) |
| `plugins` | no | no |

```bash
# lean release binary (default)
cargo build -p f00 --release

# previous “everything” shape
cargo build -p f00 --release --features full
# or selectively:
cargo build -p f00 --release --features "archives,tui,plugins"
```

### Interactive browser → `f00-tui`

The dual-pane file browser is a **separate binary**:

```bash
f00-tui              # start in .
f00-tui ~/src        # start path
f00-tui --help
```

| 0.10 | 0.11 |
|------|------|
| `f00 --browse` / `f00 --tui` (default feature) | **`f00-tui`** binary (release archive + installer) |
| Embedded only | Optional embed: `cargo build -p f00 --features tui` then `f00 --browse` |

Release tarballs and `install.sh` install **both** `f00` and `f00-tui` when present. Skip the browser with `F00_INSTALL_TUI=0`.

### Installer / Nix

- **Recommended:** `curl -fsSL https://f00.sh/install.sh | bash` and **Nix** (`nix profile install github:theesfeld/f00`).
- Other package managers (Homebrew, AUR, Scoop, winget, deb/rpm) may lag; they are convenience channels, not the primary support surface.

### Config / env

| Variable / flag | Meaning |
|-----------------|---------|
| `--gnu` / `F00_GNU=1` | Force script-safe / GNU mode |
| `--no-gnu` / `F00_NO_GNU=1` | Force modern product mode (disables auto non-TTY GNU) |

### crates.io / rename notes

- CLI package remains **`f00`** on crates.io.
- Library crate **`f00-tui`** now also builds the **`f00-tui`** binary.

If anything breaks for you after upgrading, open an issue with `f00 --version`, OS, and the exact command (TTY vs pipe).
