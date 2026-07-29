/* f00.sh — catalog-driven cards + splash fit + particle field */
(() => {
  const CATALOG_URLS = [
    "/catalog.json",
    "catalog.json",
    "https://f00.sh/catalog.json",
  ];

  const escapeHtml = (s) =>
    String(s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");

  const cardHtml = (p) => {
    const domain = escapeHtml(p.domain || "");
    const name = escapeHtml(p.name || p.id || "");
    const blurb = p.blurb || p.one_liner || "";
    const facts = Array.isArray(p.facts) ? p.facts : [];
    const site = escapeHtml(p.site || "#");
    const repo = escapeHtml(p.repo || "#");
    const factsBlock = facts.length
      ? `<ul class="facts mono">${facts
          .map((f) => `<li>${escapeHtml(f)}</li>`)
          .join("")}</ul>`
      : "";
    return `<article class="card" data-product="${escapeHtml(p.id || "")}">
      <div class="card-meta mono">${domain}</div>
      <h3>${name}</h3>
      <p>${blurb}</p>
      ${factsBlock}
      <div class="card-actions">
        <a class="btn primary sm" href="${site}">site</a>
        <a class="btn ghost sm" href="${repo}">repo</a>
      </div>
    </article>`;
  };

  const renderProducts = (catalog) => {
    const grid = document.getElementById("product-grid")
      || document.querySelector(".products .grid");
    if (!grid || !catalog || !Array.isArray(catalog.products)) return;
    const released = catalog.products.filter((p) => p.status === "released");
    if (!released.length) return;
    grid.innerHTML = released.map(cardHtml).join("");
    grid.dataset.fromCatalog = "live";
  };

  const loadCatalog = async () => {
    for (const url of CATALOG_URLS) {
      try {
        const res = await fetch(url, { credentials: "omit" });
        if (!res.ok) continue;
        return await res.json();
      } catch {
        /* try next */
      }
    }
    return null;
  };

  // Splash fit — re-run after catalog paint and on resize.
  const splash = document.querySelector(".splash");
  const frame = document.querySelector(".splash-frame");
  let grid = document.querySelector(".products .grid");

  const fitSplash = () => {
    grid = document.getElementById("product-grid")
      || document.querySelector(".products .grid");
    if (!splash || !frame) return;
    const target = Math.floor((grid || frame).getBoundingClientRect().width);
    if (target <= 0) return;

    splash.style.width = "max-content";
    splash.style.display = "inline-block";
    splash.style.transform = "none";

    let lo = 32;
    let hi = Math.min(900, Math.ceil(target * 1.2));
    while (lo < hi) {
      const mid = Math.ceil((lo + hi) / 2);
      splash.style.fontSize = `${mid}px`;
      const w = splash.getBoundingClientRect().width;
      if (w <= target) lo = mid;
      else hi = mid - 1;
    }
    splash.style.fontSize = `${lo}px`;

    let w = splash.getBoundingClientRect().width;
    if (w > 0 && Math.abs(target - w) > 1) {
      const scaled = Math.max(32, Math.floor(lo * (target / w)));
      splash.style.fontSize = `${scaled}px`;
      w = splash.getBoundingClientRect().width;
      if (w > target) {
        splash.style.fontSize = `${Math.max(32, scaled - 1)}px`;
      }
    }
  };

  const bindSplash = () => {
    if (!splash || !frame) return;
    fitSplash();
    requestAnimationFrame(fitSplash);
    if (typeof ResizeObserver !== "undefined") {
      const ro = new ResizeObserver(fitSplash);
      ro.observe(frame);
      grid = document.getElementById("product-grid")
        || document.querySelector(".products .grid");
      if (grid) ro.observe(grid);
    } else {
      window.addEventListener("resize", fitSplash, { passive: true });
    }
    if (document.fonts && document.fonts.ready) {
      document.fonts.ready.then(fitSplash).catch(() => {});
    }
  };

  // Catalog first (source of truth), then splash.
  loadCatalog()
    .then((catalog) => {
      if (catalog) {
        renderProducts(catalog);
        // Expose for debugging / other scripts
        window.F00_CATALOG = catalog;
      }
    })
    .finally(bindSplash);

  // Particle field (monochrome)
  const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const canvas = document.getElementById("field");
  if (!canvas || prefersReduced) return;
  const ctx = canvas.getContext("2d", { alpha: true });
  if (!ctx) return;

  let w = 0;
  let h = 0;
  let dpr = 1;
  let dots = [];
  let raf = 0;

  const resize = () => {
    dpr = Math.min(window.devicePixelRatio || 1, 2);
    w = window.innerWidth;
    h = window.innerHeight;
    canvas.width = Math.floor(w * dpr);
    canvas.height = Math.floor(h * dpr);
    canvas.style.width = `${w}px`;
    canvas.style.height = `${h}px`;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    const count = Math.floor((w * h) / 14000);
    dots = Array.from({ length: count }, () => ({
      x: Math.random() * w,
      y: Math.random() * h,
      v: 0.15 + Math.random() * 0.55,
      s: Math.random() < 0.15 ? 2 : 1,
      a: 0.15 + Math.random() * 0.55,
    }));
  };

  const tick = (t) => {
    ctx.clearRect(0, 0, w, h);
    ctx.strokeStyle = "rgba(255, 255, 255, 0.04)";
    ctx.lineWidth = 1;
    const step = 48;
    const ox = (t * 0.01) % step;
    for (let x = -step + ox; x < w + step; x += step) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, h);
      ctx.stroke();
    }
    for (let y = 0; y < h; y += step) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(w, y);
      ctx.stroke();
    }

    for (const d of dots) {
      d.y += d.v;
      if (d.y > h + 4) {
        d.y = -4;
        d.x = Math.random() * w;
      }
      ctx.fillStyle = `rgba(255, 255, 255, ${d.a})`;
      ctx.fillRect(Math.floor(d.x), Math.floor(d.y), d.s, d.s);
    }

    if (Math.random() < 0.02) {
      const y = h * (0.18 + Math.random() * 0.2);
      ctx.fillStyle = "rgba(255, 255, 255, 0.03)";
      ctx.fillRect(0, y, w, 2 + Math.random() * 3);
    }

    raf = requestAnimationFrame(tick);
  };

  window.addEventListener("resize", resize, { passive: true });
  resize();
  raf = requestAnimationFrame(tick);

  document.addEventListener("visibilitychange", () => {
    if (document.hidden) cancelAnimationFrame(raf);
    else raf = requestAnimationFrame(tick);
  });
})();
