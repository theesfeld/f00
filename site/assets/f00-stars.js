/* f00 GitHub stars — hub + project footers (also SPA-safe) */
(() => {
  const fmt = (n) =>
    typeof n === "number" && Number.isFinite(n) ? n.toLocaleString("en-US") : "—";
  const cache = new Map();

  const fetchRepo = async (repo) => {
    if (cache.has(repo)) return cache.get(repo);
    const p = fetch(`https://api.github.com/repos/${repo}`, {
      headers: { Accept: "application/vnd.github+json" },
    })
      .then((r) => (r.ok ? r.json() : null))
      .then((j) =>
        j && typeof j.stargazers_count === "number" ? j.stargazers_count : 0
      )
      .catch(() => 0);
    cache.set(repo, p);
    return p;
  };

  const projectsFromCatalog = async () => {
    for (const url of [
      "https://f00.sh/catalog.json",
      "/catalog.json",
      "catalog.json",
    ]) {
      try {
        const r = await fetch(url, { credentials: "omit" });
        if (!r.ok) continue;
        const j = await r.json();
        const list = j.projects || j.products || [];
        const repos = list
          .filter((p) => p && p.status === "released" && p.repo_slug)
          .map((p) => p.repo_slug);
        if (repos.length) return repos;
      } catch {
        /* next */
      }
    }
    return [
      "f00-sh/f00tils",
      "f00-sh/clun",
      "f00-sh/cel",
      "f00-sh/trn",
      "f00-sh/heartbox",
    ];
  };

  const paint = (el, n) => {
    if (el.dataset.f00StarsPainted === String(n)) return;
    el.textContent = `★ ${fmt(n)}`;
    el.setAttribute("aria-label", `${fmt(n)} GitHub stars`);
    el.dataset.f00StarsPainted = String(n);
  };

  let totalCache = null;
  const run = async () => {
    const singles = document.querySelectorAll(
      "[data-f00-stars]:not([data-f00-stars-painted])"
    );
    for (const el of singles) {
      const repo = el.getAttribute("data-repo");
      if (!repo) continue;
      paint(el, await fetchRepo(repo));
    }

    const totals = document.querySelectorAll(
      "[data-f00-stars-total]:not([data-f00-stars-painted])"
    );
    if (totals.length) {
      if (totalCache === null) {
        const repos = await projectsFromCatalog();
        const counts = await Promise.all(repos.map(fetchRepo));
        totalCache = counts.reduce((a, b) => a + b, 0);
      }
      for (const el of totals) paint(el, totalCache);
    }
  };

  const boot = () => {
    run();
    // SPA: re-run when React mounts footers
    const mo = new MutationObserver(() => {
      run();
    });
    mo.observe(document.documentElement, { childList: true, subtree: true });
    // also a few delayed passes
    [100, 500, 1500, 3000].forEach((ms) => setTimeout(run, ms));
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot);
  } else {
    boot();
  }
})();
