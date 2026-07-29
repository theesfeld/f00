/* f00.sh — catalog cards + silver sparkles + logo color glitches */
(() => {
  const CATALOG_URLS = [
    "/catalog.json",
    "catalog.json",
    "https://f00.sh/catalog.json",
  ];

  // Heartbox palette — glitch colors for logo
  const THEME_COLORS = [
    "#E02030", // poppy red
    "#5EC8E8", // verse sky
    "#F4EBE0", // cream
    "#B8C0C8", // silver
    "#E86A9A", // pink
    "#E8D45A", // yellow
    "#5FBF4A", // green
    "#7A5A9E", // purple
    "#E8924A", // orange
    "#FFF8F0", // bright white
    "#FF4A58", // bright red
  ];

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
    const grid =
      document.getElementById("product-grid") ||
      document.querySelector(".products .grid");
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

  // —— Logo glitches: random Heartbox theme color ——
  const glyphs = document.querySelectorAll(".splash .glyph");
  const prefersReduced = window.matchMedia(
    "(prefers-reduced-motion: reduce)"
  ).matches;

  if (glyphs.length && !prefersReduced) {
    const pickColor = () =>
      THEME_COLORS[Math.floor(Math.random() * THEME_COLORS.length)];

    const fireGlitch = () => {
      const g = glyphs[Math.floor(Math.random() * glyphs.length)];
      const color = pickColor();
      const ox = (Math.random() < 0.5 ? -1 : 1) * (1 + Math.random() * 3);
      const oy = (Math.random() < 0.5 ? -1 : 1) * (Math.random() * 2);
      g.style.setProperty("--glitch-color", color);
      g.style.setProperty("--glitch-x", `${ox}px`);
      g.style.setProperty("--glitch-y", `${oy}px`);
      g.classList.add("is-glitching");
      // occasional second ghost flash
      if (Math.random() < 0.35) {
        const g2 = glyphs[Math.floor(Math.random() * glyphs.length)];
        g2.style.setProperty("--glitch-color", pickColor());
        g2.style.setProperty(
          "--glitch-x",
          `${(Math.random() < 0.5 ? -1 : 1) * (2 + Math.random() * 4)}px`
        );
        g2.style.setProperty(
          "--glitch-y",
          `${(Math.random() < 0.5 ? -1 : 1) * Math.random() * 3}px`
        );
        g2.classList.add("is-glitching");
        window.setTimeout(() => g2.classList.remove("is-glitching"), 60 + Math.random() * 80);
      }
      window.setTimeout(() => g.classList.remove("is-glitching"), 50 + Math.random() * 120);
    };

    const scheduleGlitch = () => {
      fireGlitch();
      const next = 400 + Math.random() * 2200;
      window.setTimeout(scheduleGlitch, next);
    };
    window.setTimeout(scheduleGlitch, 600);
  }

  // —— Silver sparkles only (no background grid) ——
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

    const count = Math.floor((w * h) / 11000);
    dots = Array.from({ length: count }, () => ({
      x: Math.random() * w,
      y: Math.random() * h,
      v: 0.2 + Math.random() * 0.7,
      s: Math.random() < 0.2 ? 2 : 1,
      a: 0.25 + Math.random() * 0.65,
      tw: Math.random() * Math.PI * 2,
    }));
  };

  const tick = (t) => {
    ctx.clearRect(0, 0, w, h);

    // soft paint smudges (no grid) — drifting silver haze
    if (Math.random() < 0.08) {
      const gx = Math.random() * w;
      const gy = Math.random() * h * 0.7;
      const gr = 20 + Math.random() * 80;
      const g = ctx.createRadialGradient(gx, gy, 0, gx, gy, gr);
      g.addColorStop(0, "rgba(184, 192, 200, 0.06)");
      g.addColorStop(1, "rgba(184, 192, 200, 0)");
      ctx.fillStyle = g;
      ctx.beginPath();
      ctx.arc(gx, gy, gr, 0, Math.PI * 2);
      ctx.fill();
    }

    for (const d of dots) {
      d.y += d.v;
      d.tw += 0.04;
      if (d.y > h + 4) {
        d.y = -4;
        d.x = Math.random() * w;
      }
      const twinkle = 0.55 + 0.45 * Math.sin(d.tw + t * 0.002);
      const a = Math.min(1, d.a * twinkle);
      // silver only
      ctx.fillStyle = `rgba(184, 192, 200, ${a})`;
      ctx.fillRect(Math.floor(d.x), Math.floor(d.y), d.s, d.s);
      // brighter silver core on larger sparks
      if (d.s > 1) {
        ctx.fillStyle = `rgba(244, 248, 252, ${a * 0.7})`;
        ctx.fillRect(Math.floor(d.x), Math.floor(d.y), 1, 1);
      }
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
