# f00

f00 — brand hub for f00.sh products (f00tils, clun, …)

This page is optional depth for the hub. Keep it in sync with
[README.md](../README.md) and the man page(s) under [man/](../man/).

## Why this project exists

Lead with purpose. State who it helps and what problem it solves.
Use clear public narrative (NASA Stylebook + AP Style habits).
Keep sentences short.

## Requirements

- other (document the supported toolchain and version)

## Install

Write install steps as procedures (Simplified Technical English).
Every install method installs man page(s).

### Curl (releases)

```text
curl -fsSL https://github.com/f00-sh/f00/releases/latest/download/install.sh | sh
```

### Package managers

List only channels this project offers (Arch/AUR, Homebrew, RPM, deb as chosen).
Do not list packages that do not exist.

### From source

```text
# Add from-source steps when useful for developers.
```

## Usage

```text
# Show the common commands a new user needs first.
```

See also the man page for full option reference.

## Configuration

Document flags, environment variables, and config files.
Provide a `.env.example` when environment variables are required.
Never commit real secrets.

## Documentation set

| Surface | Location |
|---|---|
| README | [README.md](../README.md) |
| Man page(s) | [man/](../man/) |
| Product hub | `site/` (Cloudflare Pages → f00.sh) |
| Changelog | [CHANGELOG.md](../CHANGELOG.md) |
| Scene card | [file_id.diz](../file_id.diz) |
| Security | [SECURITY.md](../SECURITY.md) |

## Scene card

Each SemVer release ships a crafted `file_id.diz` scene card (ACiD / 16colo.rs-style
block ASCII). Keep this preview identical to the repository root file and to the
GitHub Release asset named `file_id.diz`.

```text
╔══════════════════════════════════════════════════╗
║▓▓▓▓░░░░  f00  ░░░░▓▓▓▓              ║
║████████████████████████████████████████████████  ║
║  ▄█▀  SCENE CARD  ▀█▄   release identity         ║
║████████████████████████████████████████████████  ║
║  v0.0.0  ·  MIT  ·  2026                     ║
║  f00 — brand hub for f00.sh products (f00tils, clun, …)                         ║
║  github:f00-sh/f00          ║
╚══════════════════════════════════════════════════╝
```

See [file_id.diz](../file_id.diz) and [CHANGELOG.md](../CHANGELOG.md).

## Development

See [CONTRIBUTING.md](../CONTRIBUTING.md).

## Versioning

This project uses [Semantic Versioning](https://semver.org/).
See [CHANGELOG.md](../CHANGELOG.md).
Every published version refreshes `file_id.diz` and attaches it to the GitHub Release.

## License

[MIT](../LICENSE) © William Theesfeld
