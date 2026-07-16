# f00.sh DNS + HTTPS checklist

GitHub Pages is configured with **CNAME `f00.sh`** and deploys from `site/` on `main`.

## DNS records (at your registrar)

For an **apex** domain `f00.sh` pointed at GitHub Pages:

| Type | Name | Value |
|------|------|--------|
| A | `@` | `185.199.108.153` |
| A | `@` | `185.199.109.153` |
| A | `@` | `185.199.110.153` |
| A | `@` | `185.199.111.153` |
| AAAA | `@` | `2606:50c0:8000::153` |
| AAAA | `@` | `2606:50c0:8001::153` |
| AAAA | `@` | `2606:50c0:8002::153` |
| AAAA | `@` | `2606:50c0:8003::153` |

Optional www:

| Type | Name | Value |
|------|------|--------|
| CNAME | `www` | `theesfeld.github.io` |

## GitHub

1. Repo **Settings → Pages**: Source = GitHub Actions (already).
2. Custom domain = `f00.sh`.
3. Enable **Enforce HTTPS** after DNS verifies (can take minutes–hours).

## Verify

```bash
dig +short f00.sh A
curl -fsSI https://f00.sh/
curl -fsSL https://f00.sh/install.sh | head
curl -fsSL https://f00.sh/install.sh | bash
```

Fallback while DNS propagates:

```bash
curl -fsSL https://raw.githubusercontent.com/theesfeld/f00/main/install.sh | bash
# site preview:
# https://theesfeld.github.io/f00/
```

## Status note

As of v0.3 prep: Pages deploy succeeds; **apex DNS may still need records at the registrar** if `f00.sh` does not resolve. Tracked in issue #20.
