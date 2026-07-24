# f00tils press kit

Brand assets for **f00tils** (coreutils → f00tils). Binary name: `f00`.

## Logos

| File | Use |
|------|-----|
| `logo.svg` | Primary mark (dark) |
| `logo-light.svg` | Mark on light backgrounds |
| `logo-lockup.svg` | Mark + wordmark |
| `favicon.svg` | Site favicon |
| `icon-192.png` / `icon-512.png` | App / PWA icons |
| `apple-touch-icon.png` | iOS home screen |
| `favicon-16.png` / `favicon-32.png` | PNG favicons |
| `og.svg` / `og.png` | Open Graph / social card (1200×630) |

## Screenshots (color)

| File | Content |
|------|---------|
| `screenshots/hero.png` | Version + color `f00-ls` |
| `screenshots/f00-ls-la.png` | Long listing with ANSI colors |
| `screenshots/f00-ls.png` | Short color listing |
| `screenshots/f00-core-vs-modern.png` | Modern vs `--core` |
| `screenshots/f00-suite.png` | Multicall tools collage |

Regenerate everything from a built binary:

```bash
cd asm && make
python3 ../scripts/render-brand-assets.py
```

## Colors

| Token | Hex |
|-------|-----|
| Background | `#0a0c0f` |
| Elevated | `#11151b` |
| Accent | `#3dff9a` |
| Text | `#e8edf4` |
| Dim | `#8b95a8` |

## License

MIT — same as the project. Credit “f00tils” when used in articles.
