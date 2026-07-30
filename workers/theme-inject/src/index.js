/**
 * f00 zone Worker:
 * 1) Inject shared theme CSS if missing
 * 2) Inject GitHub stars footer script (inline — avoids apex asset cache poison)
 */
const THEME_LINK =
  '<link rel="stylesheet" href="https://f00.sh/theme/f00-theme.css" data-f00-theme="1" />';

const STARS_SCRIPT = `<script data-f00-stars-script>
(() => {
  const fmt = (n) => (typeof n === "number" && Number.isFinite(n) ? n.toLocaleString("en-US") : "—");
  const cache = new Map();
  const fetchRepo = async (repo) => {
    if (cache.has(repo)) return cache.get(repo);
    const p = fetch("https://api.github.com/repos/" + repo, {
      headers: { Accept: "application/vnd.github+json" },
    })
      .then((r) => (r.ok ? r.json() : null))
      .then((j) => (j && typeof j.stargazers_count === "number" ? j.stargazers_count : 0))
      .catch(() => 0);
    cache.set(repo, p);
    return p;
  };
  const projectsFromCatalog = async () => {
    for (const url of ["https://f00.sh/catalog.json", "/catalog.json", "catalog.json"]) {
      try {
        const r = await fetch(url, { credentials: "omit" });
        if (!r.ok) continue;
        const j = await r.json();
        const list = j.projects || j.products || [];
        const repos = list.filter((p) => p && p.status === "released" && p.repo_slug).map((p) => p.repo_slug);
        if (repos.length) return repos;
      } catch (_) {}
    }
    return ["f00-sh/f00tils", "f00-sh/clun", "f00-sh/cel", "f00-sh/trn", "f00-sh/heartbox"];
  };
  const paint = (el, n) => {
    el.textContent = "★ " + fmt(n);
    el.setAttribute("aria-label", fmt(n) + " GitHub stars");
  };
  const run = async () => {
    for (const el of document.querySelectorAll("[data-f00-stars]")) {
      const repo = el.getAttribute("data-repo");
      if (!repo) continue;
      paint(el, await fetchRepo(repo));
    }
    const totals = document.querySelectorAll("[data-f00-stars-total]");
    if (totals.length) {
      const repos = await projectsFromCatalog();
      const counts = await Promise.all(repos.map(fetchRepo));
      const sum = counts.reduce((a, b) => a + b, 0);
      for (const el of totals) paint(el, sum);
    }
  };
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", run);
  else run();
})();
</script>`;

const SKIP_PREFIXES = ["/theme/", "/assets/", "/styles", "/catalog.json", "/favicon"];

export default {
  async fetch(request) {
    const url = new URL(request.url);
    const path = url.pathname;

    if (SKIP_PREFIXES.some((p) => path.startsWith(p))) {
      return fetch(request);
    }

    const response = await fetch(request);
    const ct = response.headers.get("content-type") || "";
    if (!ct.includes("text/html")) {
      return response;
    }

    const headers = new Headers(response.headers);
    headers.delete("content-length");

    let html = await response.text();

    // Inject theme CSS if missing
    if (
      !html.includes("f00-theme.css") &&
      !html.includes("data-f00-theme")
    ) {
      if (/<\/head>/i.test(html)) {
        html = html.replace(/<\/head>/i, `  ${THEME_LINK}\n</head>`);
      } else if (/<body/i.test(html)) {
        html = html.replace(/<body/i, `${THEME_LINK}\n<body`);
      }
    }

    // Inject stars script once
    if (!html.includes("data-f00-stars-script")) {
      if (/<\/body>/i.test(html)) {
        html = html.replace(/<\/body>/i, `${STARS_SCRIPT}\n</body>`);
      } else {
        html += STARS_SCRIPT;
      }
    }

    return new Response(html, {
      status: response.status,
      statusText: response.statusText,
      headers,
    });
  },
};
