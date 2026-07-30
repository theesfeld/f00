/* f00.sh — catalog cards + scroll-shrink film hero + technicolor glitches */
(() => {
  const CATALOG_URLS = [
    "/catalog.json",
    "catalog.json",
    "https://f00.sh/catalog.json",
  ];

  /* photo-locked Heartbox — no pure white / LED primaries */
  const THEME_COLORS = [
    "#D44A18", "#1E78C8", "#EDE6DE", "#B8BEC2", "#C47A72",
    "#C49A3C", "#3D8A48", "#454B93", "#C45A20",
    "#8B9AD0", "#E86022", "#3A94D8", "#E8E2D8",
  ];

  const root = document.documentElement;
  const hero = document.querySelector(".hero");
  const splash = document.querySelector(".splash");
  const header = document.querySelector(".top");
  const headerInner = document.querySelector(".top-inner");
  if (splash) {
    splash.style.fontSize = "";
    splash.style.width = "";
    splash.style.display = "";
    splash.style.transform = "";
  }

  /*
   * Fixed-logo dock + constant-slot:
   *  - logo is position:fixed under the header (rule never bisects it)
   *  - in-flow slot height = measured max logo height + --logo-gap (constant)
   *  - p = scrollY / (maxH − restH) so logo height drop ≈ scroll → projects
   *    stay a fixed visual gap under the mark while it shrinks UP
   *  - p≥1: logo docks at rest under header; further scroll sends body
   *    under the dock / frosted header (header line is the cutoff)
   */
  const prefersReduced = window.matchMedia(
    "(prefers-reduced-motion: reduce)"
  ).matches;

  let splashMaxPx = 0;
  let splashRestPx = 0;
  let splashMaxH = 0;
  let splashRestH = 0;
  let logoGapPx = 28;
  let shrinkRange = 1;

  const measureHeader = () => {
    const el = headerInner || header;
    if (!el) return;
    const h = Math.ceil(el.getBoundingClientRect().height);
    if (h > 0) root.style.setProperty("--header-h", `${h}px`);
  };

  const readLogoGap = () => {
    const raw = getComputedStyle(root).getPropertyValue("--logo-gap").trim();
    const n = parseFloat(raw);
    if (!Number.isFinite(n)) return 28;
    if (raw.endsWith("rem")) {
      const fs = parseFloat(getComputedStyle(document.body).fontSize) || 16;
      return n * fs;
    }
    return n;
  };

  /** Force font-size (!important CSS), measure glyph box. */
  const measureAtFont = (px) => {
    splash.style.setProperty("font-size", `${px}px`, "important");
    void splash.offsetHeight;
    const h = splash.getBoundingClientRect().height;
    splash.style.removeProperty("font-size");
    return h;
  };

  const measureSplashSizes = () => {
    if (!splash) return;
    const headerH =
      parseFloat(getComputedStyle(root).getPropertyValue("--header-h")) || 54;
    const availH = Math.max(160, window.innerHeight - headerH - 56);
    const availW = window.innerWidth * 0.9;
    /* rest: catalog-scale mark under header */
    splashRestPx = Math.min(112, Math.max(52, window.innerWidth * 0.085));
    /* max: fill device under header (Onyx ~0.55em wide × 3 ≈ 1.5em) */
    const byH = availH / 0.92;
    const byW = availW / 1.55;
    splashMaxPx = Math.min(byH, byW);
    splashMaxPx = Math.max(splashRestPx * 1.45, splashMaxPx);

    root.style.setProperty("--splash-max", `${splashMaxPx.toFixed(1)}px`);
    root.style.setProperty("--splash-rest", `${splashRestPx.toFixed(1)}px`);

    /* measure real painted heights at max/rest (Onyx metrics ≠ CSS math) */
    const pWas = root.style.getPropertyValue("--p");
    root.style.setProperty("--p", "0");
    splashMaxH = measureAtFont(splashMaxPx) || splashMaxPx * 0.86;
    root.style.setProperty("--p", "1");
    splashRestH = measureAtFont(splashRestPx) || splashRestPx * 0.86;
    root.style.setProperty("--p", pWas || "0");

    logoGapPx = readLogoGap();
    /*
     * Slot = max logo height + gap. splash-frame also has 0.35rem top pad —
     * include so projects sit --logo-gap under the painted mark, not the frame.
     */
    const framePad = 0.35 * (parseFloat(getComputedStyle(document.body).fontSize) || 16);
    const slotH = Math.ceil(framePad + splashMaxH + logoGapPx);
    root.style.setProperty("--splash-slot-h", `${slotH}px`);

    /*
     * Scroll distance for p 0→1 must equal height delta so:
     *   projects_vp ≈ header + framePad + maxH + gap − scrollY
     *   logo_bottom ≈ header + framePad + (maxH − scrollY)  [while p<1]
     *   gap_visual  ≈ logoGap  (constant)
     */
    shrinkRange = Math.max(64, splashMaxH - splashRestH);
  };

  const setProgress = (p) => {
    const v = Math.max(0, Math.min(1, p));
    root.style.setProperty("--p", v.toFixed(4));
    if (hero) {
      hero.classList.toggle("is-done", v > 0.98);
      hero.classList.add("is-live");
    }
  };

  const setDock = (yPx, op) => {
    root.style.setProperty("--dock-y", `${yPx.toFixed(1)}px`);
    root.style.setProperty("--dock-op", Math.max(0, Math.min(1, op)).toFixed(3));
  };

  const updateHeroScroll = () => {
    if (!splash || prefersReduced) {
      setProgress(prefersReduced ? 1 : 0);
      setDock(0, prefersReduced ? 0 : 1);
      return;
    }
    const y = window.scrollY;
    if (y <= shrinkRange) {
      /* phase 1: shrink UP under header; body gap constant */
      setProgress(y / shrinkRange);
      setDock(0, 1);
      return;
    }
    /*
     * phase 2: dock at rest under header, then dissolve in place.
     * Do NOT translate up through the header rule (that re-creates the
     * “line through the logo”). Header frost is the scroll cutoff —
     * body continues under it once the mark has faded.
     */
    setProgress(1);
    const over = y - shrinkRange;
    const fadeDist = Math.max(72, splashRestH * 0.95);
    const op = Math.max(0, 1 - over / fadeDist);
    setDock(0, op);
  };

  if (splash) {
    let ticking = false;
    const onScroll = () => {
      if (ticking) return;
      ticking = true;
      requestAnimationFrame(() => {
        updateHeroScroll();
        ticking = false;
      });
    };
    const onResize = () => {
      measureHeader();
      measureSplashSizes();
      updateHeroScroll();
    };
    window.addEventListener("scroll", onScroll, { passive: true });
    window.addEventListener("resize", onResize, { passive: true });
    measureHeader();
    measureSplashSizes();
    updateHeroScroll();
    if (document.fonts && document.fonts.ready) {
      document.fonts.ready.then(() => {
        measureSplashSizes();
        updateHeroScroll();
      });
    }
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
    const x = 8 + (h % 84);
    const y = 8 + ((h >>> 7) % 84);
    const x2 = 8 + ((h >>> 14) % 84);
    const y2 = 8 + ((h >>> 21) % 84);
    const px = (h >>> 3) % 92;
    const py = (h >>> 11) % 92;
    const sx = 140 + (h % 80);
    const sy = 140 + ((h >>> 8) % 80);
    const papers = [
      "var(--tex-wc-1)",
      "var(--tex-wc-2)",
      "var(--tex-wc-3)",
      "var(--tex-wc-4)",
      "var(--tex-wc-5)",
      "var(--tex-wc-a)",
      "var(--tex-wc-b)",
    ];
    const paper = papers[h % papers.length];
    const wash = (0.5 + ((h >>> 5) % 22) / 100).toFixed(2);
    const fiber = (0.3 + ((h >>> 9) % 18) / 100).toFixed(2);
    const aR = 220 + (h % 35);
    const aG = 180 + ((h >>> 4) % 55);
    const aB = 160 + ((h >>> 10) % 60);
    const aA = (0.2 + ((h >>> 2) % 16) / 100).toFixed(2);
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
      `--wc-b:rgba(${30 + (h % 80)},${2 + (h % 10)},${6 + (h % 16)},${(0.24 + ((h >>> 6) % 14) / 100).toFixed(2)})`,
      `--wc-c:rgba(${180 + (h % 50)},${8 + (h % 55)},${20 + (h % 45)},${(0.14 + ((h >>> 12) % 14) / 100).toFixed(2)})`,
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

// —— Film glitches: technicolor plate misregistration + emulsion flash ——
  const glyphs = document.querySelectorAll(".splash .glyph");

  if (glyphs.length && !prefersReduced) {
    const pickColor = () =>
      THEME_COLORS[Math.floor(Math.random() * THEME_COLORS.length)];

    const pNow = () => parseFloat(getComputedStyle(root).getPropertyValue("--p")) || 0;

    const fireGlitch = () => {
      const p = pNow();
      /* stronger / wider when logo is large (full hero) */
      const amp = 1.2 + (1 - p) * 4.5;
      const g = glyphs[Math.floor(Math.random() * glyphs.length)];
      const color = pickColor();
      const ox = (Math.random() < 0.5 ? -1 : 1) * (1 + Math.random() * amp);
      const oy = (Math.random() < 0.5 ? -1 : 1) * (Math.random() * amp * 0.55);
      g.style.setProperty("--glitch-color", color);
      g.style.setProperty("--glitch-x", `${ox.toFixed(2)}px`);
      g.style.setProperty("--glitch-y", `${oy.toFixed(2)}px`);
      g.style.setProperty(
        "--rgb-cx",
        `${((Math.random() < 0.5 ? -1 : 1) * (1 + Math.random() * amp * 0.7)).toFixed(2)}px`
      );
      g.style.setProperty(
        "--rgb-cy",
        `${((Math.random() < 0.5 ? -1 : 1) * Math.random() * amp * 0.4).toFixed(2)}px`
      );
      g.classList.add("is-glitching");

      /* occasional multi-plate flash (dye transfer tear) */
      if (Math.random() < 0.4 + (1 - p) * 0.25) {
        const g2 = glyphs[Math.floor(Math.random() * glyphs.length)];
        g2.style.setProperty("--glitch-color", pickColor());
        g2.style.setProperty(
          "--glitch-x",
          `${((Math.random() < 0.5 ? -1 : 1) * (2 + Math.random() * amp)).toFixed(2)}px`
        );
        g2.style.setProperty(
          "--glitch-y",
          `${((Math.random() < 0.5 ? -1 : 1) * Math.random() * amp * 0.5).toFixed(2)}px`
        );
        g2.classList.add("is-glitching");
        window.setTimeout(
          () => g2.classList.remove("is-glitching"),
          70 + Math.random() * 140
        );
      }

      /* rare full-word flash */
      if (Math.random() < 0.12 * (1.2 - p)) {
        glyphs.forEach((el) => {
          el.style.setProperty("--glitch-color", pickColor());
          el.classList.add("is-glitching");
        });
        window.setTimeout(() => {
          glyphs.forEach((el) => el.classList.remove("is-glitching"));
        }, 40 + Math.random() * 60);
      }

      window.setTimeout(
        () => g.classList.remove("is-glitching"),
        55 + Math.random() * (90 + (1 - p) * 80)
      );
    };

    const scheduleGlitch = () => {
      fireGlitch();
      const p = pNow();
      /* denser glitches while full-screen */
      const next = 220 + Math.random() * (900 + p * 1600);
      window.setTimeout(scheduleGlitch, next);
    };
    window.setTimeout(scheduleGlitch, 400);
  }
})();
