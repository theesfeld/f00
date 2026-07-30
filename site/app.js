/* f00.sh — catalog project cards + silver sparkles + logo color glitches */
(() => {
  const CATALOG_URLS = [
    "/catalog.json",
    "catalog.json",
    "https://f00.sh/catalog.json",
  ];

  const THEME_COLORS = [
    "#C50A1B", "#2096EE", "#EDE6DE", "#B8BEC2", "#C47A72",
    "#D4A83A", "#5A8A3A", "#454B93", "#C45A20",
    "#F5F1EA", "#E81420", "#4AADF5",
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

  /* stable per-project watercolor seed so cards don't all match */
  const hashStr = (s) => {
    let h = 2166136261;
    const str = String(s || "");
    for (let i = 0; i < str.length; i++) {
      h ^= str.charCodeAt(i);
      h = Math.imul(h, 16777619);
    }
    return h >>> 0;
  };
  const wcStyle = (id) => {
    const h = hashStr(id);
    const x = 10 + (h % 80);
    const y = 10 + ((h >>> 7) % 80);
    const x2 = 10 + ((h >>> 14) % 80);
    const y2 = 10 + ((h >>> 21) % 80);
    const px = (h >>> 3) % 90;
    const py = (h >>> 11) % 90;
    const sx = 130 + (h % 60);
    const sy = 130 + ((h >>> 8) % 60);
    const paper = h & 1 ? "var(--tex-wc-b)" : "var(--tex-wc-a)";
    const wash = (0.44 + ((h >>> 5) % 16) / 100).toFixed(2);
    const fiber = (0.28 + ((h >>> 9) % 14) / 100).toFixed(2);
    const aR = 245 + (h % 10);
    const aG = 190 + ((h >>> 4) % 45);
    const aB = 170 + ((h >>> 10) % 50);
    const aA = (0.22 + ((h >>> 2) % 14) / 100).toFixed(2);
    return [
      `--wc-x:${x}%`,
      `--wc-y:${y}%`,
      `--wc-x2:${x2}%`,
      `--wc-y2:${y2}%`,
      `--wc-pos:${px}% ${py}%`,
      `--wc-size:${sx}% ${sy}%`,
      `--wc-paper:${paper}`,
      `--wc-wash-op:${wash}`,
      `--wc-fiber-op:${fiber}`,
      `--wc-a:rgba(${aR},${aG},${aB},${aA})`,
      `--wc-b:rgba(${40 + (h % 70)},${2 + (h % 8)},${8 + (h % 14)},${(0.22 + ((h >>> 6) % 12) / 100).toFixed(2)})`,
      `--wc-c:rgba(${190 + (h % 40)},${10 + (h % 50)},${25 + (h % 40)},${(0.14 + ((h >>> 12) % 12) / 100).toFixed(2)})`,
    ].join(";");
  };

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
    const pid = p.id || name || domain;
    const style = wcStyle(pid);
    return `<article class="card" data-project="${escapeHtml(p.id || "")}" style="${style}">
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

  const paintStaticCards = () => {
    document.querySelectorAll("article.card[data-project]").forEach((el) => {
      if (el.getAttribute("style") && el.getAttribute("style").includes("--wc-x")) return;
      const id = el.getAttribute("data-project") || el.querySelector("h3")?.textContent || "";
      el.style.cssText = (el.style.cssText ? el.style.cssText + ";" : "") + wcStyle(id);
    });
  };

  const projectsList = (catalog) => {
    if (!catalog) return [];
    return catalog.projects || catalog.products || [];
  };

  const renderProjects = (catalog) => {
    const grid =
      document.getElementById("project-grid") ||
      document.getElementById("product-grid") ||
      document.querySelector(".projects .grid") ||
      document.querySelector(".products .grid");
    if (!grid) return;
    const list = projectsList(catalog);
    if (!Array.isArray(list)) return;
    const released = list.filter((p) => p.status === "released");
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

  paintStaticCards();
  loadCatalog().then((catalog) => {
    if (catalog) {
      renderProjects(catalog);
      window.F00_CATALOG = catalog;
    }
    paintStaticCards();
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
