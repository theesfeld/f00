/* f00.sh — catalog-driven cards + Heartbox-tinted particle field */
(() => {
  const CATALOG_URLS = [
    "/catalog.json",
    "catalog.json",
    "https://f00.sh/catalog.json",
  ];

  // Never let leftover fitSplash inline styles blow up the logo.
  const splash = document.querySelector(".splash");
  if (splash) {
    splash.style.fontSize = "";
    splash.style.width = "";
    splash.style.display = "";
    splash.style.transform = "";
  }

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
    const docs = p.docs ? escapeHtml(p.docs) : "";
    const factsBlock = facts.length
      ? `<ul class="facts">${facts
          .map((f) => `<li>${escapeHtml(f)}</li>`)
          .join("")}</ul>`
      : "";
    const docsBtn = docs
      ? `<a class="btn ghost sm" href="${docs}">docs</a>`
      : "";
    return `<article class="card" data-product="${escapeHtml(p.id || "")}">
      <div class="card-meta mono">${domain}</div>
      <h3>${name}</h3>
      <p>${blurb}</p>
      ${factsBlock}
      <div class="card-actions">
        <a class="btn primary sm" href="${site}">site</a>
        <a class="btn ghost sm" href="${repo}">repo</a>
        ${docsBtn}
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

  loadCatalog().then((catalog) => {
    if (catalog) {
      renderProducts(catalog);
      window.F00_CATALOG = catalog;
    }
  });

  // Particle field (Heartbox cream / silver / red dust)
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
    const pickKind = () => {
      const r = Math.random();
      if (r < 0.04) return "red";
      if (r < 0.08) return "sky";
      if (r < 0.22) return "metal";
      return "cream";
    };
    dots = Array.from({ length: count }, () => ({
      x: Math.random() * w,
      y: Math.random() * h,
      v: 0.15 + Math.random() * 0.55,
      s: Math.random() < 0.15 ? 2 : 1,
      a: 0.15 + Math.random() * 0.55,
      kind: pickKind(),
    }));
  };

  const fillFor = (d) => {
    if (d.kind === "red") return `rgba(224, 32, 48, ${Math.min(1, d.a + 0.15)})`;
    if (d.kind === "sky") return `rgba(94, 200, 232, ${Math.min(1, d.a + 0.1)})`;
    if (d.kind === "metal") return `rgba(184, 192, 200, ${d.a})`;
    return `rgba(244, 235, 224, ${d.a * 0.85})`;
  };

  const tick = (t) => {
    ctx.clearRect(0, 0, w, h);
    ctx.strokeStyle = "rgba(184, 192, 200, 0.05)";
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
      ctx.fillStyle = fillFor(d);
      ctx.fillRect(Math.floor(d.x), Math.floor(d.y), d.s, d.s);
    }

    if (Math.random() < 0.02) {
      const y = h * (0.18 + Math.random() * 0.2);
      ctx.fillStyle = "rgba(224, 32, 48, 0.04)";
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
