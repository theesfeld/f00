/* f00 — Projection Specimen Engine
 *
 * Each element is its OWN PROJECTION onto the display:
 *   light → object-as-film → lens → screen
 * Logo, cards, header, footer, type, rules, field — each a complete throw
 * with private seed + private optical stack. Not “positions of petals.”
 * An entropic *view* of a natural thing. Species f00; never clones.
 * Zen band: readable / usable.
 */
(() => {
  if (typeof document === "undefined") return;
  if (document.documentElement.dataset.f00Entropy === "1") return;
  document.documentElement.dataset.f00Entropy = "1";

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const root = document.documentElement;

  const throwSalt = (() => {
    try {
      const a = new Uint32Array(2);
      crypto.getRandomValues(a);
      return (a[0] ^ a[1] ^ ((performance.now() * 1000) | 0)) >>> 0;
    } catch {
      return ((Math.random() * 0xffffffff) ^ Date.now()) >>> 0;
    }
  })();

  const hashStr = (str) => {
    let h = 2166136261 >>> 0;
    for (let i = 0; i < str.length; i++) {
      h ^= str.charCodeAt(i);
      h = Math.imul(h, 16777619);
    }
    return (h ^ throwSalt) >>> 0;
  };

  const mulberry = (seed) => {
    let s = seed >>> 0 || 1;
    return () => {
      s |= 0;
      s = (s + 0x6d2b79f5) | 0;
      let t = Math.imul(s ^ (s >>> 15), 1 | s);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  };

  const makeBag = (rnd) => ({
    rnd,
    range: (a, b) => a + (b - a) * rnd(),
    signed: (m) => aSigned(rnd, m),
  });
  const aSigned = (rnd, m) => -m + 2 * m * rnd();

  const seedFor = (el, role) => {
    const id =
      el.getAttribute?.("data-project") ||
      el.id ||
      (typeof el.className === "string" ? el.className : el.tagName) ||
      "node";
    const path =
      role +
      "|" +
      id +
      "|" +
      (el.getAttribute?.("href") || "") +
      "|" +
      (el.textContent || "").slice(0, 64);
    return hashStr(path);
  };

  /** Full optical stack for ONE object-projection */
  const createProjection = (el, role, gains) => {
    const seed = seedFor(el, role);
    const R = makeBag(mulberry(seed));
    const g = gains || {};
    /* zen clamps: max amplitudes by role */
    const pose = g.pose ?? 1;
    const blur = g.blur ?? 1;
    const persp = g.persp ?? 1;

    return {
      el,
      role,
      seed,
      R,
      /* static specimen of this throw (never rebuilt mid-session) */
      gateX: R.signed(0.9 * pose),
      gateY: R.signed(0.75 * pose),
      tiltX: R.signed(0.55 * persp), /* deg keystone-ish */
      tiltY: R.signed(0.45 * persp),
      buckle: R.signed(0.012 * pose),
      rotZ: R.signed(0.28 * pose),
      edgeT: R.range(1.35, 2.65),
      edgeR: R.range(1.35, 2.65),
      edgeB: R.range(1.35, 2.65),
      edgeL: R.range(1.35, 2.65),
      lamp: R.range(0.92, 1),
      track: R.signed(0.016),
      word: R.signed(0.028),
      baseY: R.signed(0.35),
      defocus0: R.range(0.04, 0.22) * blur, /* px — non-zero floor */
      emul: R.range(0.7, 1.35), /* local emulsion thickness proxy */
      /* continuous private dynamics */
      phi: R.range(0, Math.PI * 2),
      omega: R.range(0.28, 1.15),
      ampGate: R.range(0.15, 0.7) * pose,
      ampTilt: R.range(0.05, 0.22) * persp,
      ampDef: R.range(0.02, 0.12) * blur,
      ampRot: R.range(0.015, 0.07) * pose,
      /* live */
      liveGateX: 0,
      liveGateY: 0,
      liveTiltX: 0,
      liveTiltY: 0,
      liveDef: 0,
      liveRot: 0,
    };
  };

  const projections = new Map();

  window.F00Projection = {
    throwSalt,
    seed: throwSalt,
    reduced,
    projections,
    seedFor,
    /** optical readout for WebGL plates sharing scene time */
    getPlateOptics(el) {
      const p = projections.get(el);
      if (!p) return null;
      return {
        seed: p.seed,
        gateX: p.gateX + p.liveGateX,
        gateY: p.gateY + p.liveGateY,
        tiltX: p.tiltX + p.liveTiltX,
        tiltY: p.tiltY + p.liveTiltY,
        defocus: p.defocus0 + p.liveDef,
        buckle: p.buckle,
        emul: p.emul,
      };
    },
  };

  const applyProjectionCSS = (p) => {
    const el = p.el;
    const gx = p.gateX + p.liveGateX;
    const gy = p.gateY + p.liveGateY;
    const tx = p.tiltX + p.liveTiltX;
    const ty = p.tiltY + p.liveTiltY;
    const rz = p.rotZ + p.liveRot;
    const def = Math.max(0.03, p.defocus0 + p.liveDef);

    /* full stack as transform: perspective keystone + gate + roll */
    el.style.setProperty("--p-persp", "900px");
    el.style.setProperty("--p-rx", `${tx.toFixed(3)}deg`);
    el.style.setProperty("--p-ry", `${ty.toFixed(3)}deg`);
    el.style.setProperty("--p-rz", `${rz.toFixed(3)}deg`);
    el.style.setProperty("--p-x", `${gx.toFixed(2)}px`);
    el.style.setProperty("--p-y", `${gy.toFixed(2)}px`);
    el.style.setProperty("--p-defocus", `${def.toFixed(3)}px`);
    el.style.setProperty("--p-lamp", p.lamp.toFixed(4));
    el.style.setProperty("--p-emul", p.emul.toFixed(3));

    if (p.role === "solid" || p.role === "plate" || p.role === "chrome") {
      el.style.setProperty("--e-bw-t", `${p.edgeT.toFixed(2)}px`);
      el.style.setProperty("--e-bw-r", `${p.edgeR.toFixed(2)}px`);
      el.style.setProperty("--e-bw-b", `${p.edgeB.toFixed(2)}px`);
      el.style.setProperty("--e-bw-l", `${p.edgeL.toFixed(2)}px`);
      el.style.setProperty("--e-op", p.lamp.toFixed(4));
    }
    if (p.role === "type" || p.role === "chrome") {
      el.style.setProperty("--e-t-track", `${p.track.toFixed(4)}em`);
      el.style.setProperty("--e-t-word", `${p.word.toFixed(4)}em`);
      el.style.setProperty(
        "--e-t-y",
        `${(p.baseY + p.liveGateY * 0.2).toFixed(2)}px`
      );
      el.style.setProperty("--e-t-op", Math.min(1, p.lamp + 0.02).toFixed(4));
    }
    /* alias legacy vars used by older CSS */
    el.style.setProperty("--e-rot", `${rz.toFixed(3)}deg`);
    el.style.setProperty("--e-x", `${gx.toFixed(2)}px`);
    el.style.setProperty("--e-y", `${gy.toFixed(2)}px`);
    el.style.setProperty("--w-x", "0px");
    el.style.setProperty("--w-y", "0px");
    el.style.setProperty("--w-rot", "0deg");
  };

  const register = (el, role, gains) => {
    if (!el || projections.has(el)) return;
    const p = createProjection(el, role, gains);
    projections.set(el, p);
    el.dataset.f00Projection = p.seed.toString(16);
    el.classList.add("f00-proj");
    applyProjectionCSS(p);
  };

  /* ── emulsion RULE: own projection (path = this throw’s line) ── */
  const rulePath = (R) => {
    const n = 7 + Math.floor(R.rnd() * 8);
    const mid = 1.2 + R.signed(0.2);
    let d = `M 0 ${(mid + R.signed(0.6)).toFixed(3)}`;
    for (let i = 1; i <= n; i++) {
      d += ` L ${((i / n) * 100).toFixed(3)} ${(mid + R.signed(0.7)).toFixed(3)}`;
    }
    return d;
  };

  const attachRule = (el, side) => {
    if (!el || el.dataset.f00Rule === "1") return;
    el.dataset.f00Rule = "1";
    el.classList.add("e-rule-host", side === "top" ? "e-rule-top" : "e-rule-bottom");
    if (side === "bottom") {
      el.style.borderBottom = "0";
    } else {
      el.style.borderTop = "0";
    }
    const seed = seedFor(el, "rule:" + side);
    const R = makeBag(mulberry(seed));
    const r = Math.round(R.range(198, 222));
    const g = Math.round(R.range(206, 224));
    const b = Math.round(R.range(212, 230));
    const a = R.range(0.42, 0.74);
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-rule");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 2.5");
    svg.innerHTML = `<path d="${rulePath(R)}" fill="none" stroke="rgba(${r},${g},${b},${a.toFixed(3)})" stroke-width="${R.range(0.8, 1.55).toFixed(2)}" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke"/>`;
    el.appendChild(svg);
    register(el, "rule", { pose: 0.35, blur: 0.5, persp: 0.25 });
  };

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

  const gainsFor = (role) => {
    if (role === "plate") return { pose: 0.85, blur: 0.9, persp: 0.7 };
    if (role === "solid") return { pose: 1, blur: 0.45, persp: 0.85 };
    if (role === "type") return { pose: 0.25, blur: 0.35, persp: 0.15 };
    if (role === "chrome") return { pose: 0.4, blur: 0.3, persp: 0.3 };
    return { pose: 0.6, blur: 0.4, persp: 0.5 };
  };

  const paint = (scope) => {
    const sc = scope || document;
    sc.querySelectorAll(solidSel).forEach((el) => {
      const role = el.classList.contains("splash-wrap") ? "plate" : "solid";
      register(el, role, gainsFor(role));
    });
    sc.querySelectorAll(typeSel).forEach((el) => {
      if (el.closest?.(".splash-wrap.is-film .splash")) return;
      if (el.classList?.contains("glyph")) return;
      register(el, "type", gainsFor("type"));
    });
    sc.querySelectorAll(".brand, .nav").forEach((el) =>
      register(el, "chrome", gainsFor("chrome"))
    );
  };

  const ruleHosts = () => {
    document.querySelectorAll(".top-inner").forEach((el) => attachRule(el, "bottom"));
    document
      .querySelectorAll(".foot, footer, .site-footer")
      .forEach((el) => attachRule(el, "top"));
    document.querySelectorAll(".section-head").forEach((el) => attachRule(el, "bottom"));
  };

  /* field = own projection */
  {
    const R = makeBag(mulberry(throwSalt ^ 0x51ed));
    root.style.setProperty("--e-bg-x", `${(50 + R.signed(4.5)).toFixed(3)}%`);
    root.style.setProperty("--e-bg-y", `${(50 + R.signed(3.8)).toFixed(3)}%`);
    root.style.setProperty("--e-track", `${R.signed(0.012).toFixed(4)}em`);
    root.style.setProperty("--e-paper", R.range(0.965, 1).toFixed(4));
    projections.set(root, {
      role: "field",
      seed: throwSalt ^ 0x51ed,
      bgX: 50 + R.signed(4.5),
      bgY: 50 + R.signed(3.8),
      p1: R.range(0.04, 0.11),
      p2: R.range(0.05, 0.14),
      p3: R.range(0.03, 0.09),
      a1: R.range(0.35, 1.0),
      a2: R.range(0.25, 0.8),
      bx: 50,
      by: 50,
    });
    const f = projections.get(root);
    f.bx = f.bgX;
    f.by = f.bgY;
  }

  ruleHosts();
  paint(document);

  new MutationObserver((muts) => {
    for (const m of muts) {
      m.addedNodes.forEach((n) => {
        if (n.nodeType !== 1) return;
        paint(n);
        if (
          n.matches?.(".section-head, .foot, .top-inner") ||
          n.querySelector?.(".section-head, .foot, .top-inner")
        ) {
          ruleHosts();
        }
      });
    }
  }).observe(document.documentElement, { childList: true, subtree: true });

  if (!reduced) {
    let t0 = performance.now();
    let last = t0;
    const tick = (now) => {
      const dt = Math.min(0.05, (now - last) / 1000);
      last = now;
      const t = (now - t0) / 1000;

      const field = projections.get(root);
      if (field?.role === "field") {
        const tx =
          field.bgX +
          Math.sin(t * field.p1) * field.a1 +
          Math.sin(t * field.p3 * 1.7) * field.a2 * 0.4;
        const ty =
          field.bgY +
          Math.cos(t * field.p2) * field.a2 +
          Math.sin(t * field.p1 * 0.6) * field.a1 * 0.35;
        field.bx += (tx - field.bx) * Math.min(1, dt * 0.35);
        field.by += (ty - field.by) * Math.min(1, dt * 0.35);
        root.style.setProperty("--e-bg-x", `${field.bx.toFixed(3)}%`);
        root.style.setProperty("--e-bg-y", `${field.by.toFixed(3)}%`);
      }

      projections.forEach((p) => {
        if (p.role === "field") return;
        /* private continuous optics for THIS projection only */
        p.phi += dt * p.omega;
        p.liveGateX = Math.sin(p.phi) * p.ampGate;
        p.liveGateY = Math.cos(p.phi * 0.93) * p.ampGate * 0.85;
        p.liveTiltX = Math.sin(p.phi * 0.41) * p.ampTilt;
        p.liveTiltY = Math.cos(p.phi * 0.37) * p.ampTilt;
        p.liveDef = (0.5 + 0.5 * Math.sin(p.phi * 0.55)) * p.ampDef;
        p.liveRot = Math.sin(p.phi * 0.29) * p.ampRot;
        applyProjectionCSS(p);
      });

      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }

  root.dataset.f00Projection = throwSalt.toString(16);
})();
