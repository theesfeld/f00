/**
 * f00 zone Worker: force central theme + entropy + stars on every *.f00.sh HTML.
 *
 * SSOT live URLs (hub ships these; this Worker rewrites every site to match):
 *   https://f00.sh/theme/f00-theme.css
 *   https://f00.sh/theme/f00-entropy.js?v=…
 *
 * Project HTML may point at older f00-theme-N.css — we rewrite. Missing links
 * are injected. Project CSS stays layout-only.
 */
const THEME_HREF = "https://f00.sh/theme/f00-theme.css";
const ENTROPY_HREF = "https://f00.sh/theme/f00-entropy.js?v=24";

const THEME_LINK = `<link rel="stylesheet" href="${THEME_HREF}" data-f00-theme="1" />`;
const ENTROPY_SCRIPT =
  `<script src="${ENTROPY_HREF}" data-f00-entropy-script defer><` + `/script>`;

const STARS_JS =
  "/* f00 GitHub stars \u2014 hub + project footers (also SPA-safe) */\n(() => {\n  const fmt = (n) =>\n    typeof n === \"number\" && Number.isFinite(n) ? n.toLocaleString(\"en-US\") : \"\u2014\";\n  const cache = new Map();\n\n  const fetchRepo = async (repo) => {\n    if (cache.has(repo)) return cache.get(repo);\n    const p = fetch(`https://api.github.com/repos/${repo}`, {\n      headers: { Accept: \"application/vnd.github+json\" },\n    })\n      .then((r) => (r.ok ? r.json() : null))\n      .then((j) =>\n        j && typeof j.stargazers_count === \"number\" ? j.stargazers_count : 0\n      )\n      .catch(() => 0);\n    cache.set(repo, p);\n    return p;\n  };\n\n  const projectsFromCatalog = async () => {\n    for (const url of [\n      \"https://f00.sh/catalog.json\",\n      \"/catalog.json\",\n      \"catalog.json\",\n    ]) {\n      try {\n        const r = await fetch(url, { credentials: \"omit\" });\n        if (!r.ok) continue;\n        const j = await r.json();\n        const list = j.projects || j.products || [];\n        const repos = list\n          .filter((p) => p && p.status === \"released\" && p.repo_slug)\n          .map((p) => p.repo_slug);\n        if (repos.length) return repos;\n      } catch {\n        /* next */\n      }\n    }\n    return [\n      \"f00-sh/f00tils\",\n      \"f00-sh/clun\",\n      \"f00-sh/cel\",\n      \"f00-sh/trn\",\n      \"f00-sh/heartbox\",\n    ];\n  };\n\n  const paint = (el, n) => {\n    if (el.dataset.f00StarsPainted === String(n)) return;\n    el.textContent = `\u2605 ${fmt(n)}`;\n    el.setAttribute(\"aria-label\", `${fmt(n)} GitHub stars`);\n    el.dataset.f00StarsPainted = String(n);\n  };\n\n  const run = async () => {\n    const singles = document.querySelectorAll(\n      \"[data-f00-stars]:not([data-f00-stars-painted])\"\n    );\n    for (const el of singles) {\n      const repo = el.getAttribute(\"data-repo\");\n      if (!repo) continue;\n      paint(el, await fetchRepo(repo));\n    }\n    const totals = document.querySelectorAll(\"[data-f00-stars-total]\");\n    if (!totals.length) return;\n    const repos = await projectsFromCatalog();\n    let sum = 0;\n    for (const repo of repos) sum += await fetchRepo(repo);\n    totals.forEach((el) => paint(el, sum));\n  };\n  if (document.readyState === \"loading\") {\n    document.addEventListener(\"DOMContentLoaded\", run);\n  } else {\n    run();\n  }\n})();\n";

const STARS_SCRIPT = "<script data-f00-stars-script>" + STARS_JS + "</script>";

const SKIP_PREFIXES = [
  "/theme/",
  "/assets/",
  "/styles",
  "/catalog.json",
  "/favicon",
];

function pinTheme(html) {
  // any f00 theme URL → live SSOT
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/f00-theme(?:-\d+)?\.css(?:\?[^"'\s>]*)?/g,
    THEME_HREF
  );
  html = html.replace(
    /(?:https:\/\/f00\.sh)?\/theme\/f00-theme(?:-\d+)?\.css(?:\?[^"'\s>]*)?/g,
    THEME_HREF
  );
  // legacy heartbox / shell brand sheets → SSOT
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/(?:pack\/|textures\/)?hb-shell[^"'\s>]*\.css/g,
    THEME_HREF
  );
  html = html.replace(
    /href=["'][^"']*hb-shell[^"']*\.css["']/gi,
    `href="${THEME_HREF}" data-f00-theme="1"`
  );

  // ensure exactly one data-f00-theme link in head
  if (!/data-f00-theme\s*=\s*["']?1["']?/.test(html) && !html.includes(THEME_HREF)) {
    if (/<\/head>/i.test(html)) {
      html = html.replace(/<\/head>/i, `  ${THEME_LINK}\n</head>`);
    } else if (/<body/i.test(html)) {
      html = html.replace(/<body/i, `${THEME_LINK}\n<body`);
    } else {
      html = THEME_LINK + "\n" + html;
    }
  } else if (html.includes(THEME_HREF) && !/data-f00-theme/.test(html)) {
    html = html.replace(
      new RegExp(
        `(<link[^>]+href=["']${THEME_HREF.replace(/\./g, "\\.")}["'][^>]*)(/?>)`,
        "i"
      ),
      `$1 data-f00-theme="1"$2`
    );
  }
  return html;
}

function pinEntropy(html) {
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/f00-entropy\.js(?:\?v=[^"'\s>]*)?/g,
    ENTROPY_HREF
  );
  if (!html.includes("data-f00-entropy-script")) {
    if (/<\/body>/i.test(html)) {
      html = html.replace(/<\/body>/i, `${ENTROPY_SCRIPT}\n</body>`);
    } else if (/<\/head>/i.test(html)) {
      html = html.replace(/<\/head>/i, `  ${ENTROPY_SCRIPT}\n</head>`);
    } else {
      html += ENTROPY_SCRIPT;
    }
  }
  return html;
}

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

    html = pinTheme(html);
    html = pinEntropy(html);
    html = pinStars(html);

    return new Response(html, {
      status: response.status,
      statusText: response.statusText,
      headers,
    });
  },
};
