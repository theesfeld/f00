# Contributing to f00

Thanks for helping improve f00 — a cross-platform `ls` rewrite in Rust.

## Getting started

1. Install a recent [Rust stable](https://rustup.rs/) toolchain.
2. Fork and clone the repo.
3. Build and test:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Binary crate: `crates/f00-cli` → binary name `f00`.

## Project layout

| Crate | Role |
|-------|------|
| `f00-cli` | CLI entrypoint, flags, UX |
| `f00-core` | Walk, metadata, sort |
| `f00-format` | Color, columns, tree, JSON, icons |
| `f00-git` | Git status integration |
| `f00-compat` | GNU mode / flag translation |

## Guidelines

- **Accuracy over hype** — label planned work clearly; don’t claim unfinished features in user-facing docs.
- **Small PRs** — easier to review; one concern per change when practical.
- **Tests** — add or update tests for behavior changes, especially flag parsing and format output.
- **Style** — run `cargo fmt` and `clippy` cleanly before opening a PR.
- **Platforms** — call out Windows / macOS / Linux behavior differences in the PR description when relevant.

## Pull requests

1. Open an issue for larger features when in doubt.
2. Branch from `main`.
3. Ensure CI checks pass (fmt, clippy, tests on Linux / macOS / Windows).
4. Fill in a short summary: what changed, why, and how you tested.

## License

By contributing, you agree that your contributions are dual-licensed under
**MIT OR Apache-2.0**, the same as the project, without additional terms.
