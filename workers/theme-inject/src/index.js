/**
 * f00 zone Worker: force central theme + entropy + chrome + stars on every *.f00.sh HTML.
 *
 * SSOT live URLs (hub ships these; this Worker rewrites every site to match):
 *   https://f00.sh/theme/f00-theme.css
 *   https://f00.sh/theme/f00-entropy.js?v=…
 *   https://f00.sh/theme/f00-chrome.js?v=…
 *
 * Project HTML may point at older f00-theme-N.css — we rewrite. Missing links
 * are injected. Project CSS stays layout-only.
 */
import { pinTheme, pinEntropy, pinChrome, pinDomain } from "./pins.js";

const STARS_JS =
  "/* f00 GitHub stars \u2014 hub + project footers (also SPA-safe) */\n(() => {\n  const fmt = (n) =>\n    typeof n === \"number\" && Number.isFinite(n) ? n.toLocaleString(\"en-US\") : \"\u2014\";\n  const cache = new Map();\n\n  const fetchRepo = async (repo) => {\n    if (cache.has(repo)) return cache.get(repo);\n    const p = fetch(`https://api.github.com/repos/${repo}`, {\n      headers: { Accept: \"application/vnd.github+json\" },\n    })\n      .then((r) => (r.ok ? r.json() : null))\n      .then((j) =>\n        j && typeof j.stargazers_count === \"number\" ? j.stargazers_count : 0\n      )\n      .catch(() => 0);\n    cache.set(repo, p);\n    return p;\n  };\n\n  const projectsFromCatalog = async () => {\n    for (const url of [\n      \"https://f00.sh/catalog.json\",\n      \"/catalog.json\",\n      \"catalog.json\",\n    ]) {\n      try {\n        const r = await fetch(url, { credentials: \"omit\" });\n        if (!r.ok) continue;\n        const j = await r.json();\n        const list = j.projects || j.products || [];\n        const repos = list\n          .filter((p) => p && p.status === \"released\" && p.repo_slug)\n          .map((p) => p.repo_slug);\n        if (repos.length) return repos;\n      } catch {\n        /* next */\n      }\n    }\n    return [\n      \"f00-sh/f00tils\",\n      \"f00-sh/clun\",\n      \"f00-sh/cel\",\n      \"f00-sh/trn\",\n      \"f00-sh/somata\",\n    ];\n  };\n\n  const paint = (el, n) => {\n    if (el.dataset.f00StarsPainted === String(n)) return;\n    el.textContent = `\u2605 ${fmt(n)}`;\n    el.setAttribute(\"aria-label\", `${fmt(n)} GitHub stars`);\n    el.dataset.f00StarsPainted = String(n);\n  };\n\n  const run = async () => {\n    const singles = document.querySelectorAll(\n      \"[data-f00-stars]:not([data-f00-stars-painted])\"\n    );\n    for (const el of singles) {\n      const repo = el.getAttribute(\"data-repo\");\n      if (!repo) continue;\n      paint(el, await fetchRepo(repo));\n    }\n    const totals = document.querySelectorAll(\n      \"[data-f00-stars-total]:not([data-f00-stars-painted])\"\n    );\n    if (!totals.length) return;\n    const repos = await projectsFromCatalog();\n    let sum = 0;\n    for (const r of repos) sum += await fetchRepo(r);\n    for (const el of totals) paint(el, sum);\n  };\n\n  const boot = () => {\n    run();\n    // SPA: re-run when React mounts footers\n    const mo = new MutationObserver(() => run());\n    mo.observe(document.documentElement, { childList: true, subtree: true });\n  };\n  if (document.readyState === \"loading\") {\n    document.addEventListener(\"DOMContentLoaded\", boot, { once: true });\n  } else {\n    boot();\n  }\n})();";

const STARS_SCRIPT = "<script data-f00-stars-script>" + STARS_JS + "</script>";

const SKIP_PREFIXES = [
  "/theme/",
  "/assets/",
  "/styles",
  "/catalog.json",
  "/favicon",
];

function pinStars(html) {
  if (!html.includes("data-f00-stars-script")) {
    if (/<\/body>/i.test(html)) {
      html = html.replace(/<\/body>/i, `${STARS_SCRIPT}\n</body>`);
    } else {
      html += STARS_SCRIPT;
    }
  }
  return html;
}

/** One collective donate link on every *.f00.sh site (never per-project). */
const DONATE_HREF = "https://f00.sh/donate";
const DONATE_JS =
  "/* f00 collective donate — header nav, one pool */\n(() => {\n" +
  "  if (document.querySelector(\"[data-f00-donate]\")) return;\n" +
  "  const nav = document.querySelector(\"nav.nav, header .nav, .top-inner .nav, .site-header nav, header nav\");\n" +
  "  if (!nav) return;\n" +
  "  const a = document.createElement(\"a\");\n" +
  "  a.href = " +
  JSON.stringify(DONATE_HREF) +
  ";\n" +
  "  a.className = \"nav-donate\";\n" +
  "  a.dataset.f00Donate = \"1\";\n" +
  "  a.textContent = \"donate\";\n" +
  "  a.rel = \"noopener\";\n" +
  "  const gh = [...nav.querySelectorAll(\"a\")].find((el) => /github/i.test(el.href || \"\") || /github/i.test(el.textContent || \"\"));\n" +
  "  if (gh) nav.insertBefore(a, gh);\n" +
  "  else nav.appendChild(a);\n" +
  "})();";
const DONATE_SCRIPT =
  "<script data-f00-donate-script>" + DONATE_JS + "</script>";

function pinDonate(html) {
  /* normalize any existing collective donate anchors */
  html = html.replace(
    /(<a[^>]*data-f00-donate[^>]*href=["'])[^"']*(["'])/gi,
    `$1${DONATE_HREF}$2`
  );
  if (!html.includes("data-f00-donate-script") && !html.includes("data-f00-donate")) {
    if (/<\/body>/i.test(html)) {
      html = html.replace(/<\/body>/i, `${DONATE_SCRIPT}\n</body>`);
    } else {
      html += DONATE_SCRIPT;
    }
  }
  return html;
}

export { pinTheme, pinEntropy, pinChrome, pinDomain, pinStars, pinDonate };

export default {
  async fetch(request) {
    const url = new URL(request.url);
    const path = url.pathname;
    if (SKIP_PREFIXES.some((p) => path.startsWith(p))) return fetch(request);

    const response = await fetch(request);
    const ct = response.headers.get("content-type") || "";
    if (!ct.includes("text/html")) return response;

    const headers = new Headers(response.headers);
    headers.delete("content-length");
    let html = await response.text();

    html = pinDomain(html, url.hostname);
    html = pinTheme(html);
    html = pinEntropy(html);
    html = pinChrome(html);
    html = pinStars(html);
    html = pinDonate(html);

    return new Response(html, {
      status: response.status,
      statusText: response.statusText,
      headers,
    });
  },
};
