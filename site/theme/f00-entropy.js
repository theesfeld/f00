/* f00 — Projection Specimen Engine (org-wide)
 *
 * GOAL: every surface projected onto the display is a unique organic throw.
 * Header/footer rules, body type, cards, chrome — never CAD-perfect, never
 * identical twins. Species remains f00; specimen is always new.
 *
 * Zen band: readable, usable, hittable. Order in disorder.
 * Vocabulary: projection / throw / specimen — not “page load.”
 */
(() => {
  if (typeof document === "undefined") return;
  if (document.documentElement.dataset.f00Entropy === "1") return;
  document.documentElement.dataset.f00Entropy = "1";

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const root = document.documentElement;

  /* ── seed this throw (never persisted) ── */
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

  const chromeR = () => Math.round(range(200, 220));
  const chromeG = () => Math.round(range(208, 222));
  const chromeB = () => Math.round(range(214, 228));
  const chromeA = (lo, hi) => range(lo, hi);

  /* ── field specimen ── */
  const bgX = 50 + signed(4.5);
  const bgY = 50 + signed(3.8);
  set("--e-bg-x", `${bgX.toFixed(3)}%`);
  set("--e-bg-y", `${bgY.toFixed(3)}%`);
  set("--e-bg-scale", (1 + signed(0.02)).toFixed(4));
  set("--e-track", `${signed(0.014).toFixed(4)}em`);
  set("--e-word", `${signed(0.02).toFixed(4)}em`);
  set("--e-paper", range(0.965, 1).toFixed(4));
  set("--e-chrome-op", chromeA(0.48, 0.7).toFixed(3));
  set("--e-chrome-op-b", chromeA(0.44, 0.68).toFixed(3));
  set("--e-line-t", `${range(0.75, 1.3).toFixed(3)}px`);
  set("--e-line-b", `${range(0.75, 1.3).toFixed(3)}px`);
  set("--e-line", `var(--e-line-b)`);

  /* ── imperfect emulsion RULE (never a CAD 1px collinear border) ── */
  const rulePath = (segments) => {
    const n = segments || 7 + Math.floor(rnd() * 6);
    const mid = 1.2 + signed(0.15);
    let d = `M 0 ${(mid + signed(0.55)).toFixed(3)}`;
    for (let i = 1; i <= n; i++) {
      const x = ((i / n) * 100).toFixed(3);
      /* low-amp wander — species: a line; specimen: not straight */
      const y = (mid + signed(0.65)).toFixed(3);
      d += ` L ${x} ${y}`;
    }
    return d;
  };

  const makeRuleSvg = (opts) => {
    const stroke = opts.stroke;
    const sw = opts.width;
    const d = rulePath(opts.segments);
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-rule");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 2.5");
    svg.innerHTML = `<path d="${d}" fill="none" stroke="${stroke}" stroke-width="${sw}" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke"/>`;
    return svg;
  };

  const attachRule = (el, side) => {
    if (!el || el.dataset.f00Rule === "1") return;
    el.dataset.f00Rule = "1";
    el.classList.add("e-rule-host");
    if (side === "top") el.classList.add("e-rule-top");
    else el.classList.add("e-rule-bottom");
    /* kill perfect border on this edge — SVG is the projected line */
    if (side === "bottom") {
      el.style.borderBottomColor = "transparent";
      el.style.borderBottomWidth = "0";
    } else {
      el.style.borderTopColor = "transparent";
      el.style.borderTopWidth = "0";
    }
    const a = side === "bottom"
      ? chromeA(0.48, 0.72)
      : chromeA(0.45, 0.7);
    const r = chromeR();
    const g = chromeG();
    const b = chromeB();
    const stroke = `rgba(${r},${g},${b},${a.toFixed(3)})`;
    const width = range(0.9, 1.45).toFixed(2);
    const svg = makeRuleSvg({ stroke, width, segments: 8 + Math.floor(rnd() * 7) });
    el.appendChild(svg);
  };

  const ruleHosts = () => {
    document.querySelectorAll(".top-inner, header.top .top-inner").forEach((el) => {
      attachRule(el, "bottom");
    });
    document.querySelectorAll(".foot, footer, .site-footer").forEach((el) => {
      attachRule(el, "top");
    });
    document.querySelectorAll(".section-head").forEach((el) => {
      attachRule(el, "bottom");
    });
  };

  /* ── boxes: each organ under the skin ── */
  const boxSel = [
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
  ].join(",");

  const paintBoxes = (scope) => {
    scope.querySelectorAll(boxSel).forEach((el) => {
      if (el.dataset.f00EntropyBox === "1") return;
      el.dataset.f00EntropyBox = "1";
      el.style.setProperty("--e-rot", `${signed(0.32).toFixed(3)}deg`);
      el.style.setProperty("--e-x", `${signed(2.0).toFixed(2)}px`);
      el.style.setProperty("--e-y", `${signed(1.7).toFixed(2)}px`);
      el.style.setProperty("--e-bw-t", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-r", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-b", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-l", `${range(1.4, 2.6).toFixed(2)}px`);
      el.style.setProperty("--e-op", range(0.97, 1).toFixed(4));
    });
  };

  /* ── type: every text node-block is a slight specimen ── */
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

  const paintType = (scope) => {
    scope.querySelectorAll(typeSel).forEach((el) => {
      if (el.dataset.f00EntropyType === "1") return;
      /* skip WebGL-hidden splash glyphs */
      if (el.closest && el.closest(".splash-wrap.is-film .splash")) return;
      if (el.classList && el.classList.contains("glyph")) return;
      el.dataset.f00EntropyType = "1";
      el.style.setProperty("--e-t-track", `${signed(0.016).toFixed(4)}em`);
      el.style.setProperty("--e-t-word", `${signed(0.028).toFixed(4)}em`);
      /* baseline: tiny optical seat, not layout reflow */
      el.style.setProperty("--e-t-y", `${signed(0.35).toFixed(2)}px`);
      el.style.setProperty("--e-t-op", range(0.94, 1).toFixed(4));
    });
  };

  /* ── chrome bits ── */
  const paintChrome = (scope) => {
    scope.querySelectorAll(".brand, .nav, .top-inner, .foot").forEach((el) => {
      if (el.dataset.f00EntropyChrome === "1") return;
      el.dataset.f00EntropyChrome = "1";
      el.style.setProperty("--e-ch-y", `${signed(0.4).toFixed(2)}px`);
      el.style.setProperty("--e-ch-track", `${signed(0.02).toFixed(4)}em`);
    });
  };

  const paintAll = (scope) => {
    const sc = scope || document;
    paintBoxes(sc);
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
        if (n.matches && (n.matches(".section-head") || n.matches(".foot") || n.matches(".top-inner"))) {
          ruleHosts();
        }
        if (n.querySelectorAll) {
          if (n.querySelector(".section-head, .foot, .top-inner")) ruleHosts();
        }
      });
    }
  });
  mo.observe(document.documentElement, { childList: true, subtree: true });

  /* continuous field air only */
  if (!reduced) {
    let bx = bgX;
    let by = bgY;
    let t0 = performance.now();
    let last = t0;
    const p1 = range(0.05, 0.12);
    const p2 = range(0.07, 0.15);
    const p3 = range(0.03, 0.09);
    const a1 = range(0.35, 0.95);
    const a2 = range(0.25, 0.75);

    const tick = (now) => {
      const dt = Math.min(0.05, (now - last) / 1000);
      last = now;
      const t = (now - t0) / 1000;
      const tx = bgX + Math.sin(t * p1) * a1 + Math.sin(t * p3 * 1.7) * a2 * 0.4;
      const ty = bgY + Math.cos(t * p2) * a2 + Math.sin(t * p1 * 0.6) * a1 * 0.35;
      bx += (tx - bx) * Math.min(1, dt * 0.35);
      by += (ty - by) * Math.min(1, dt * 0.35);
      set("--e-bg-x", `${bx.toFixed(3)}%`);
      set("--e-bg-y", `${by.toFixed(3)}%`);
      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }

  root.dataset.f00Projection = seed.toString(16);
})();
