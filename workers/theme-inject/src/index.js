/**
 * f00 zone Worker: inject theme CSS + stars script if missing
 */
const THEME_LINK =
  '<link rel="stylesheet" href="https://f00.sh/theme/f00-theme-34.css" data-f00-theme="1" />';
const STARS_JS = "/* f00 GitHub stars \u2014 hub + project footers (also SPA-safe) */\n(() => {\n  const fmt = (n) =>\n    typeof n === \"number\" && Number.isFinite(n) ? n.toLocaleString(\"en-US\") : \"\u2014\";\n  const cache = new Map();\n\n  const fetchRepo = async (repo) => {\n    if (cache.has(repo)) return cache.get(repo);\n    const p = fetch(`https://api.github.com/repos/${repo}`, {\n      headers: { Accept: \"application/vnd.github+json\" },\n    })\n      .then((r) => (r.ok ? r.json() : null))\n      .then((j) =>\n        j && typeof j.stargazers_count === \"number\" ? j.stargazers_count : 0\n      )\n      .catch(() => 0);\n    cache.set(repo, p);\n    return p;\n  };\n\n  const projectsFromCatalog = async () => {\n    for (const url of [\n      \"https://f00.sh/catalog.json\",\n      \"/catalog.json\",\n      \"catalog.json\",\n    ]) {\n      try {\n        const r = await fetch(url, { credentials: \"omit\" });\n        if (!r.ok) continue;\n        const j = await r.json();\n        const list = j.projects || j.products || [];\n        const repos = list\n          .filter((p) => p && p.status === \"released\" && p.repo_slug)\n          .map((p) => p.repo_slug);\n        if (repos.length) return repos;\n      } catch {\n        /* next */\n      }\n    }\n    return [\n      \"f00-sh/f00tils\",\n      \"f00-sh/clun\",\n      \"f00-sh/cel\",\n      \"f00-sh/trn\",\n      \"f00-sh/heartbox\",\n    ];\n  };\n\n  const paint = (el, n) => {\n    if (el.dataset.f00StarsPainted === String(n)) return;\n    el.textContent = `\u2605 ${fmt(n)}`;\n    el.setAttribute(\"aria-label\", `${fmt(n)} GitHub stars`);\n    el.dataset.f00StarsPainted = String(n);\n  };\n\n  let totalCache = null;\n  const run = async () => {\n    const singles = document.querySelectorAll(\n      \"[data-f00-stars]:not([data-f00-stars-painted])\"\n    );\n    for (const el of singles) {\n      const repo = el.getAttribute(\"data-repo\");\n      if (!repo) continue;\n      paint(el, await fetchRepo(repo));\n    }\n\n    const totals = document.querySelectorAll(\n      \"[data-f00-stars-total]:not([data-f00-stars-painted])\"\n    );\n    if (totals.length) {\n      if (totalCache === null) {\n        const repos = await projectsFromCatalog();\n        const counts = await Promise.all(repos.map(fetchRepo));\n        totalCache = counts.reduce((a, b) => a + b, 0);\n      }\n      for (const el of totals) paint(el, totalCache);\n    }\n  };\n\n  const boot = () => {\n    run();\n    // SPA: re-run when React mounts footers\n    const mo = new MutationObserver(() => {\n      run();\n    });\n    mo.observe(document.documentElement, { childList: true, subtree: true });\n    // also a few delayed passes\n    [100, 500, 1500, 3000].forEach((ms) => setTimeout(run, ms));\n  };\n\n  if (document.readyState === \"loading\") {\n    document.addEventListener(\"DOMContentLoaded\", boot);\n  } else {\n    boot();\n  }\n})();\n";
const ENTROPY_SCRIPT = '<script src="https://f00.sh/theme/f00-entropy.js?v=21" data-f00-entropy-script defer><'+'/script>';
const STARS_SCRIPT = '<script data-f00-stars-script>' + STARS_JS + '</script>';
const SKIP_PREFIXES = ["/theme/", "/assets/", "/styles", "/catalog.json", "/favicon"];

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

    // pin all sites to current shared shell CSS
    html = html.replace(
      /https:\/\/f00\.sh\/theme\/f00-theme(?:-\d+)?\.css/g,
      "https://f00.sh/theme/f00-theme-34.css"
    );
    html = html.replace(
      /(?:https:\/\/f00\.sh)?\/theme\/f00-theme(?:-\d+)?\.css/g,
      "https://f00.sh/theme/f00-theme-34.css"
    );
    html = html.replace(
      /https:\/\/f00\.sh\/theme\/(?:pack\/|textures\/)?hb-shell[^"'\s>]*\.css/g,
      "https://f00.sh/theme/f00-theme-34.css"
    );
    if (!html.includes("f00-theme-34.css") && !html.includes("data-f00-theme")) {
      if (/<\/head>/i.test(html)) html = html.replace(/<\/head>/i, `  ${THEME_LINK}\n</head>`);
      else if (/<body/i.test(html)) html = html.replace(/<body/i, `${THEME_LINK}\n<body`);
    }
    /* pin entropy script version so all projects get organic rules/frames */
    html = html.replace(
      /https:\/\/f00\.sh\/theme\/f00-entropy\.js(?:\?v=[^"'\s>]*)?/g,
      "https://f00.sh/theme/f00-entropy.js?v=21"
    );
    if (!html.includes("data-f00-entropy-script")) {
      if (/<\/body>/i.test(html)) html = html.replace(/<\/body>/i, `${ENTROPY_SCRIPT}\n</body>`);
      else html += ENTROPY_SCRIPT;
    }
    if (!html.includes("data-f00-stars-script")) {
      if (/<\/body>/i.test(html)) html = html.replace(/<\/body>/i, `${STARS_SCRIPT}\n</body>`);
      else html += STARS_SCRIPT;
    }
    return new Response(html, { status: response.status, statusText: response.statusText, headers });
  },
};
