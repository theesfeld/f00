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
  let dockOp = 1;
  let filmHandle = null;

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

  const notifyThrow = () => {
    const h = filmHandle || window.__f00ThrowHandle;
    if (h && typeof h.resize === "function") h.resize();
  };

  const measureSplashSizes = () => {
    if (!splash) return;
    const headerH =
      parseFloat(getComputedStyle(root).getPropertyValue("--header-h")) || 54;
    logoGapPx = readLogoGap();
    /*
     * Symmetric air when docked: header line → logo top  ==  logo bottom → projects.
     * At full hero size we also reserve --logo-hero-top so the mark isn't hard
     * against the header rule.
     */
    const heroTopExtra = (() => {
      const raw = getComputedStyle(root).getPropertyValue("--logo-hero-top").trim();
      const n = parseFloat(raw);
      if (!Number.isFinite(n)) return 18;
      if (raw.endsWith("rem")) {
        const fs = parseFloat(getComputedStyle(document.body).fontSize) || 16;
        return n * fs;
      }
      return n;
    })();
    const peek = 36; /* thin projects peek under the mark */
    const availH = Math.max(
      200,
      window.innerHeight - headerH - logoGapPx * 2 - heroTopExtra - peek
    );
    const availW = window.innerWidth * 0.94;
    /* rest: compact brand mark under header (stays visible, not card-scale) */
    splashRestPx = Math.min(56, Math.max(40, window.innerWidth * 0.038));
    /*
     * Onyx "f00": glyph box ≈ 0.84×font tall, ≈ 1.28×font wide.
     * Size by the tighter axis so the plate eats the hero band.
     */
    const byH = availH / 0.84;
    const byW = availW / 1.28;
    splashMaxPx = Math.min(byH, byW);
    splashMaxPx = Math.max(splashRestPx * 2.2, splashMaxPx);

    root.style.setProperty("--splash-max", `${splashMaxPx.toFixed(1)}px`);
    root.style.setProperty("--splash-rest", `${splashRestPx.toFixed(1)}px`);

    /* measure real painted heights at max/rest (Onyx metrics ≠ CSS math) */
    const pWas = root.style.getPropertyValue("--p");
    root.style.setProperty("--p", "0");
    splashMaxH = measureAtFont(splashMaxPx) || splashMaxPx * 0.84;
    root.style.setProperty("--p", "1");
    splashRestH = measureAtFont(splashRestPx) || splashRestPx * 0.84;
    root.style.setProperty("--p", pWas || "0");

    /* if measured box overshoots avail, scale font down to fit */
    if (splashMaxH > availH * 1.02) {
      splashMaxPx *= availH / splashMaxH;
      root.style.setProperty("--splash-max", `${splashMaxPx.toFixed(1)}px`);
      root.style.setProperty("--p", "0");
      splashMaxH = measureAtFont(splashMaxPx) || splashMaxPx * 0.84;
      root.style.setProperty("--p", pWas || "0");
    }

    /* slot = air + maxH + air → projects line sits logo-gap under mark */
    const slotH = Math.ceil(logoGapPx + splashMaxH + logoGapPx);
    root.style.setProperty("--splash-slot-h", `${slotH}px`);

    /*
     * p 0→1 over height delta so bottom air stays constant while mark shrinks:
     *   logo_top    = header + gap
     *   logo_bottom = header + gap + splashH(p)
     *   projects    = header + gap + maxH + gap − scrollY
     *   with splashH ≈ maxH − scrollY → bottom gap stays `gap`
     */
    shrinkRange = Math.max(64, splashMaxH - splashRestH);
    root.style.setProperty("--shrink-range", `${shrinkRange.toFixed(1)}px`);
    notifyThrow();
  };

  const setProgress = (p) => {
    const v = Math.max(0, Math.min(1, p));
    root.style.setProperty("--p", v.toFixed(5));
    if (hero) {
      hero.classList.toggle("is-done", v > 0.98);
      hero.classList.add("is-live");
    }
  };

  const setDock = (yPx, op) => {
    dockOp = Math.max(0, Math.min(1, op));
    root.style.setProperty("--dock-y", `${yPx.toFixed(2)}px`);
    root.style.setProperty("--dock-op", dockOp.toFixed(4));
  };

  const updateHeroScroll = () => {
    if (!splash || prefersReduced) {
      setProgress(prefersReduced ? 1 : 0);
      setDock(0, 1);
      return;
    }
    const y = window.scrollY;
    if (y <= shrinkRange) {
      /* shrink UP under header; body gap constant */
      setProgress(y / Math.max(shrinkRange, 1));
    } else {
      /* docked at rest size — stay visible, do not fade out */
      setProgress(1);
    }
    setDock(0, 1);
  };

  if (splash) {
    /* rAF scroll — one update per frame, no scroll-event storms */
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
      if (filmHandle && filmHandle.resize) filmHandle.resize();
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
        notifyThrow();
      });
    }
    /* throw module may boot after us — catch late handle */
    let tries = 0;
    const waitThrow = () => {
      if (window.__f00ThrowHandle) {
        filmHandle = window.__f00ThrowHandle;
        notifyThrow();
        return;
      }
      if (++tries < 40) requestAnimationFrame(waitThrow);
    };
    requestAnimationFrame(waitThrow);
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

  /*
   * Splash mark via throw engine (structure → develop → project → display).
   * Not CSS blur. Peer to every other projected object on the surface.
   */
  /* throw-plate.js self-mounts as module — only attach handle, never double-develop */
  const splashWrap = document.querySelector(".splash-wrap");
  if (splashWrap?.dataset.throwMounted === "1" && window.__f00ThrowHandle) {
    filmHandle = window.__f00ThrowHandle;
  }
})();
