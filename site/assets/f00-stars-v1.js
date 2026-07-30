/* f00 GitHub stars — shared by hub + all project sites
   Usage:
     <a data-f00-stars data-repo="f00-sh/clun" href="https://github.com/f00-sh/clun">★ …</a>
     <a data-f00-stars-total href="https://github.com/f00-sh">★ …</a>
*/
(() => {
  const fmt = (n) => {
    if (typeof n !== "number" || !Number.isFinite(n)) return "—";
    return n.toLocaleString("en-US");
  };

  const cache = new Map();

  const fetchRepo = async (repo) => {
    if (cache.has(repo)) return cache.get(repo);
    const p = fetch(`https://api.github.com/repos/${repo}`, {
      headers: { Accept: "application/vnd.github+json" },
    })
      .then((r) => (r.ok ? r.json() : null))
      .then((j) => (j && typeof j.stargazers_count === "number" ? j.stargazers_count : 0))
      .catch(() => 0);
    cache.set(repo, p);
    return p;
  };

  const projectsFromCatalog = async () => {
    const urls = [
      "https://f00.sh/catalog.json",
      "/catalog.json",
      "catalog.json",
    ];
    for (const url of urls) {
      try {
        const r = await fetch(url, { credentials: "omit" });
        if (!r.ok) continue;
        const j = await r.json();
        const list = j.projects || j.products || [];
        return list
          .filter((p) => p && p.status === "released" && p.repo_slug)
          .map((p) => p.repo_slug);
      } catch {
        /* try next */
      }
    }
    // fallback known set
    return [
      "f00-sh/f00tils",
      "f00-sh/clun",
      "f00-sh/cel",
      "f00-sh/trn",
      "f00-sh/heartbox",
    ];
  };

  const paint = (el, n, href) => {
    el.textContent = `★ ${fmt(n)}`;
    el.setAttribute("aria-label", `${fmt(n)} GitHub stars`);
    if (href && !el.getAttribute("href")) el.setAttribute("href", href);
  };

  const run = async () => {
    const singles = document.querySelectorAll("[data-f00-stars]");
    for (const el of singles) {
      const repo = el.getAttribute("data-repo") || el.dataset.repo;
      if (!repo) continue;
      const n = await fetchRepo(repo);
      paint(el, n, el.getAttribute("href") || `https://github.com/${repo}`);
    }

    const totals = document.querySelectorAll("[data-f00-stars-total]");
    if (totals.length) {
      const repos = await projectsFromCatalog();
      // include hub repo itself? user said "all f00 projects" — catalog released projects
      const counts = await Promise.all(repos.map(fetchRepo));
      const sum = counts.reduce((a, b) => a + b, 0);
      for (const el of totals) {
        paint(el, sum, el.getAttribute("href") || "https://github.com/f00-sh");
      }
    }
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", run);
  } else {
    run();
  }
})();
