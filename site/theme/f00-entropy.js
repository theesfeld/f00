/* f00 — Projection Specimen Engine (org-wide, all objects equal)
 *
 * Tray of petals: every projection lays out a unique set of petals
 * (specimen). Wind moves them — same petals, view changes slightly.
 * Nothing underlying is rebuilt on each frame. No object is primary.
 *
 * Species = f00. Specimen = this throw. Wind = continuous air.
 * Zen band: always usable.
 */
(() => {
  if (typeof document === "undefined") return;
  if (document.documentElement.dataset.f00Entropy === "1") return;
  document.documentElement.dataset.f00Entropy = "1";

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const root = document.documentElement;

  /* ── one seed for THIS projection onto the display ── */
  const seed = (() => {
    try {
      const a = new Uint32Array(2);
      crypto.getRandomValues(a);
      return (a[0] ^ a[1] ^ ((performance.now() * 1000) | 0)) >>> 0;
    } catch {
      return ((Math.random() * 0xffffffff) ^ Date.now()) >>> 0;
    }
  })();

  let s = seed || 1;
  const rnd = () => {
    s |= 0;
    s = (s + 0x6d2b79f5) | 0;
    let t = Math.imul(s ^ (s >>> 15), 1 | s);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
  const range = (a, b) => a + (b - a) * rnd();
  const signed = (m) => range(-m, m);
  const set = (k, v) => root.style.setProperty(k, v);

  /* public: logo plate and others share this throw */
  window.F00Projection = {
    seed,
    reduced,
    rnd: () => rnd(),
    range,
    signed,
    /* 0..1 wind phase readouts (updated every frame when wind runs) */
    wind: { x: 0, y: 0, rot: 0, t: 0 },
  };

  const chromeA = (lo, hi) => range(lo, hi);
  const chromeRGB = () => [
    Math.round(range(200, 220)),
    Math.round(range(208, 222)),
    Math.round(range(214, 228)),
  ];

  /* ── field petal seat ── */
  const bgX = 50 + signed(4.5);
  const bgY = 50 + signed(3.8);
  set("--e-bg-x", `${bgX.toFixed(3)}%`);
  set("--e-bg-y", `${bgY.toFixed(3)}%`);
  set("--e-track", `${signed(0.014).toFixed(4)}em`);
  set("--e-word", `${signed(0.02).toFixed(4)}em`);
  set("--e-paper", range(0.965, 1).toFixed(4));
  set("--e-chrome-op", chromeA(0.48, 0.7).toFixed(3));
  set("--e-chrome-op-b", chromeA(0.44, 0.68).toFixed(3));
  set("--e-line-t", `${range(0.75, 1.3).toFixed(3)}px`);
  set("--e-line-b", `${range(0.75, 1.3).toFixed(3)}px`);
  set("--w-x", "0px");
  set("--w-y", "0px");
  set("--w-rot", "0deg");

  /* ── emulsion rule (path is the petal’s edge; wind can nudge later) ── */
  const rulePath = () => {
    const n = 7 + Math.floor(rnd() * 7);
    const mid = 1.2 + signed(0.15);
    let d = `M 0 ${(mid + signed(0.55)).toFixed(3)}`;
    for (let i = 1; i <= n; i++) {
      d += ` L ${((i / n) * 100).toFixed(3)} ${(mid + signed(0.65)).toFixed(3)}`;
    }
    return d;
  };

  const attachRule = (el, side) => {
    if (!el || el.dataset.f00Rule === "1") return;
    el.dataset.f00Rule = "1";
    el.classList.add("e-rule-host", side === "top" ? "e-rule-top" : "e-rule-bottom");
    if (side === "bottom") {
      el.style.borderBottomColor = "transparent";
      el.style.borderBottomWidth = "0";
    } else {
      el.style.borderTopColor = "transparent";
      el.style.borderTopWidth = "0";
    }
    const [r, g, b] = chromeRGB();
    const a = chromeA(0.45, 0.72);
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-rule");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 2.5");
    svg.innerHTML = `<path d="${rulePath()}" fill="none" stroke="rgba(${r},${g},${b},${a.toFixed(3)})" stroke-width="${range(0.9, 1.45).toFixed(2)}" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke"/>`;
    el.appendChild(svg);
  };

  const ruleHosts = () => {
    document.querySelectorAll(".top-inner").forEach((el) => attachRule(el, "bottom"));
    document.querySelectorAll(".foot, footer, .site-footer").forEach((el) => attachRule(el, "top"));
    document.querySelectorAll(".section-head").forEach((el) => attachRule(el, "bottom"));
  };

  /* ── every solid object is a petal on the tray ── */
  const solidSel = [
    ".card",
    ".panel",
    ".box",
    ".f00-box",
    "article.card",
    ".feature-card",
    ".doc-card",
    ".install-card",
    ".benchmark-card",
    ".tool-card",
    ".release-card",
    ".announcement",
    ".splash-wrap",
    ".btn",
  ].join(",");

  const typeSel = [
    "p",
    "li",
    "h1",
    "h2",
    "h3",
    "h4",
    "label",
    "td",
    "th",
    ".lede",
    ".howto",
    ".blurb",
    ".brand-mark",
    ".brand-sub",
    ".nav a",
    ".section-head h2",
    ".card-meta",
    ".facts",
    ".btn",
    ".foot",
    ".foot a",
    ".foot-stars",
    "code",
    "pre",
  ].join(",");

  const paintSolid = (scope) => {
    scope.querySelectorAll(solidSel).forEach((el) => {
      if (el.dataset.f00Petal === "1") return;
      el.dataset.f00Petal = "1";
      /* fixed petal geometry for this throw — wind adds --w-* later */
      el.style.setProperty("--e-rot", `${signed(0.32).toFixed(3)}deg`);
      el.style.setProperty("--e-x", `${signed(2.0).toFixed(2)}px`);
      el.style.setProperty("--e-y", `${signed(1.7).toFixed(2)}px`);
      el.style.setProperty("--e-bw-t", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-r", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-b", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-l", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-op", range(0.97, 1).toFixed(4));
      /* per-petal phase so wind moves them differently (same wind field) */
      el.style.setProperty("--e-phase", range(0, Math.PI * 2).toFixed(4));
      el.style.setProperty("--e-gain", range(0.55, 1.15).toFixed(3));
    });
  };

  const paintType = (scope) => {
    scope.querySelectorAll(typeSel).forEach((el) => {
      if (el.dataset.f00PetalType === "1") return;
      if (el.closest && el.closest(".splash-wrap.is-film .splash")) return;
      if (el.classList && el.classList.contains("glyph")) return;
      el.dataset.f00PetalType = "1";
      el.style.setProperty("--e-t-track", `${signed(0.016).toFixed(4)}em`);
      el.style.setProperty("--e-t-word", `${signed(0.028).toFixed(4)}em`);
      el.style.setProperty("--e-t-y", `${signed(0.35).toFixed(2)}px`);
      el.style.setProperty("--e-t-op", range(0.94, 1).toFixed(4));
      el.style.setProperty("--e-phase", range(0, Math.PI * 2).toFixed(4));
      el.style.setProperty("--e-gain", range(0.4, 1.0).toFixed(3));
    });
  };

  const paintChrome = (scope) => {
    scope.querySelectorAll(".brand, .nav, .top-inner, .foot").forEach((el) => {
      if (el.dataset.f00PetalChrome === "1") return;
      el.dataset.f00PetalChrome = "1";
      el.style.setProperty("--e-ch-y", `${signed(0.4).toFixed(2)}px`);
      el.style.setProperty("--e-ch-track", `${signed(0.02).toFixed(4)}em`);
      el.style.setProperty("--e-phase", range(0, Math.PI * 2).toFixed(4));
      el.style.setProperty("--e-gain", range(0.3, 0.9).toFixed(3));
    });
  };

  const paintAll = (scope) => {
    const sc = scope || document;
    paintSolid(sc);
    paintType(sc);
    paintChrome(sc);
  };

  ruleHosts();
  paintAll(document);

  const mo = new MutationObserver((muts) => {
    for (const m of muts) {
      m.addedNodes.forEach((n) => {
        if (n.nodeType !== 1) return;
        paintAll(n);
        if (
          n.matches?.(".section-head, .foot, .top-inner") ||
          n.querySelector?.(".section-head, .foot, .top-inner")
        ) {
          ruleHosts();
        }
      });
    }
  });
  mo.observe(document.documentElement, { childList: true, subtree: true });

  /*
   * Wind on the tray: continuous shared field.
   * Petals keep fixed specimen vars; wind offsets move the whole tray slightly.
   * Same petals — view changes. Usability: tiny amplitudes.
   */
  if (!reduced) {
    let t0 = performance.now();
    let last = t0;
    const p1 = range(0.04, 0.1);
    const p2 = range(0.06, 0.13);
    const p3 = range(0.03, 0.08);
    const a1 = range(0.4, 1.0);
    const a2 = range(0.3, 0.8);
    let bx = bgX;
    let by = bgY;

    const tick = (now) => {
      const dt = Math.min(0.05, (now - last) / 1000);
      last = now;
      const t = (now - t0) / 1000;
      window.F00Projection.wind.t = t;

      /* field air */
      const tx = bgX + Math.sin(t * p1) * a1 + Math.sin(t * p3 * 1.7) * a2 * 0.4;
      const ty = bgY + Math.cos(t * p2) * a2 + Math.sin(t * p1 * 0.6) * a1 * 0.35;
      bx += (tx - bx) * Math.min(1, dt * 0.35);
      by += (ty - by) * Math.min(1, dt * 0.35);
      set("--e-bg-x", `${bx.toFixed(3)}%`);
      set("--e-bg-y", `${by.toFixed(3)}%`);

      /* shared wind readouts (CSS + logo) */
      const wx = Math.sin(t * p1 * 0.9) * 0.55 + Math.sin(t * p3) * 0.25;
      const wy = Math.cos(t * p2 * 0.85) * 0.45 + Math.sin(t * p1 * 0.5) * 0.2;
      const wr = Math.sin(t * p2 * 0.4 + p3) * 0.06;
      window.F00Projection.wind.x = wx;
      window.F00Projection.wind.y = wy;
      window.F00Projection.wind.rot = wr;
      set("--w-x", `${wx.toFixed(3)}px`);
      set("--w-y", `${wy.toFixed(3)}px`);
      set("--w-rot", `${wr.toFixed(4)}deg`);

      /* per-petal phase offset: same wind, different response (tray of petals) */
      document.querySelectorAll("[data-f00-petal='1']").forEach((el) => {
        const ph = parseFloat(el.style.getPropertyValue("--e-phase")) || 0;
        const g = parseFloat(el.style.getPropertyValue("--e-gain")) || 1;
        const lx = wx * g * Math.cos(ph * 0.3) + wy * g * 0.15 * Math.sin(ph);
        const ly = wy * g * Math.sin(ph * 0.25) + wx * g * 0.12 * Math.cos(ph);
        const lr = wr * g * (0.7 + 0.3 * Math.sin(ph));
        el.style.setProperty("--w-x", `${lx.toFixed(3)}px`);
        el.style.setProperty("--w-y", `${ly.toFixed(3)}px`);
        el.style.setProperty("--w-rot", `${lr.toFixed(4)}deg`);
      });

      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }

  root.dataset.f00Projection = seed.toString(16);
})();
