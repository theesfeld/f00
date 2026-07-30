/* f00 — Projection Specimen Engine
 *
 * Each object is its own projection: own seed, own entropic rules, own air.
 * Not one global wind with phase offsets — independent throws that coexist.
 * Species = f00 form. Specimen = this object’s throw. Zen band: usable.
 */
(() => {
  if (typeof document === "undefined") return;
  if (document.documentElement.dataset.f00Entropy === "1") return;
  document.documentElement.dataset.f00Entropy = "1";

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const root = document.documentElement;

  /* throw-level salt only — objects derive private seeds from identity + salt */
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
    const s = String(str);
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
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

  const makeBag = (rnd) => {
    const range = (a, b) => a + (b - a) * rnd();
    const signed = (m) => range(-m, m);
    return { rnd, range, signed };
  };

  /* public API: per-object projection registry */
  const objects = new Map(); /* el -> state */
  window.F00Projection = {
    throwSalt,
    seed: throwSalt, /* back-compat for logo */
    reduced,
    objects,
    /** stable object seed for any el */
    seedFor(el, role) {
      const id =
        el.getAttribute?.("data-project") ||
        el.id ||
        el.className ||
        el.tagName ||
        "node";
      const path = role + "|" + id + "|" + (el.textContent || "").slice(0, 48);
      return hashStr(path);
    },
  };

  const setRoot = (k, v) => root.style.setProperty(k, v);

  /* ── field is one projection among many ── */
  {
    const R = makeBag(mulberry(throwSalt ^ 0x91eb63f));
    const bgX = 50 + R.signed(4.5);
    const bgY = 50 + R.signed(3.8);
    setRoot("--e-bg-x", `${bgX.toFixed(3)}%`);
    setRoot("--e-bg-y", `${bgY.toFixed(3)}%`);
    setRoot("--e-track", `${R.signed(0.014).toFixed(4)}em`);
    setRoot("--e-word", `${R.signed(0.02).toFixed(4)}em`);
    setRoot("--e-paper", R.range(0.965, 1).toFixed(4));
    setRoot("--e-chrome-op", R.range(0.48, 0.7).toFixed(3));
    setRoot("--e-chrome-op-b", R.range(0.44, 0.68).toFixed(3));
    objects.set(root, {
      role: "field",
      seed: throwSalt ^ 0x91eb63f,
      bgX,
      bgY,
      p1: R.range(0.04, 0.11),
      p2: R.range(0.05, 0.14),
      p3: R.range(0.03, 0.09),
      a1: R.range(0.35, 1.0),
      a2: R.range(0.25, 0.8),
      bx: bgX,
      by: bgY,
    });
  }

  /* ── each object: private seed → private rules + private dynamics ── */
  const createObjectState = (el, role) => {
    const seed = window.F00Projection.seedFor(el, role);
    const R = makeBag(mulberry(seed));
    const st = {
      el,
      role,
      seed,
      /* specimen (fixed for this throw) */
      rot: R.signed(0.34),
      x: R.signed(2.1),
      y: R.signed(1.8),
      bwT: R.range(1.35, 2.65),
      bwR: R.range(1.35, 2.65),
      bwB: R.range(1.35, 2.65),
      bwL: R.range(1.35, 2.65),
      op: R.range(0.97, 1),
      track: R.signed(0.018),
      word: R.signed(0.03),
      tY: R.signed(0.4),
      tOp: R.range(0.93, 1),
      /* private continuous dynamics (own air — not shared wind) */
      phi: R.range(0, Math.PI * 2),
      omega: R.range(0.35, 1.25),
      ampX: R.range(0.25, 1.1),
      ampY: R.range(0.2, 0.95),
      ampR: R.range(0.02, 0.09),
      driftX: R.signed(0.15),
      driftY: R.signed(0.15),
      /* live wind readouts for this object only */
      wx: 0,
      wy: 0,
      wr: 0,
    };
    return st;
  };

  const applySolidSpecimen = (st) => {
    const el = st.el;
    el.style.setProperty("--e-rot", `${st.rot.toFixed(3)}deg`);
    el.style.setProperty("--e-x", `${st.x.toFixed(2)}px`);
    el.style.setProperty("--e-y", `${st.y.toFixed(2)}px`);
    el.style.setProperty("--e-bw-t", `${st.bwT.toFixed(2)}px`);
    el.style.setProperty("--e-bw-r", `${st.bwR.toFixed(2)}px`);
    el.style.setProperty("--e-bw-b", `${st.bwB.toFixed(2)}px`);
    el.style.setProperty("--e-bw-l", `${st.bwL.toFixed(2)}px`);
    el.style.setProperty("--e-op", st.op.toFixed(4));
    el.style.setProperty("--w-x", "0px");
    el.style.setProperty("--w-y", "0px");
    el.style.setProperty("--w-rot", "0deg");
  };

  const applyTypeSpecimen = (st) => {
    const el = st.el;
    el.style.setProperty("--e-t-track", `${st.track.toFixed(4)}em`);
    el.style.setProperty("--e-t-word", `${st.word.toFixed(4)}em`);
    el.style.setProperty("--e-t-y", `${st.tY.toFixed(2)}px`);
    el.style.setProperty("--e-t-op", st.tOp.toFixed(4));
  };

  const rulePath = (R) => {
    const n = 7 + Math.floor(R.rnd() * 7);
    const mid = 1.2 + R.signed(0.15);
    let d = `M 0 ${(mid + R.signed(0.55)).toFixed(3)}`;
    for (let i = 1; i <= n; i++) {
      d += ` L ${((i / n) * 100).toFixed(3)} ${(mid + R.signed(0.65)).toFixed(3)}`;
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
    const seed = window.F00Projection.seedFor(el, "rule:" + side);
    const R = makeBag(mulberry(seed));
    const r = Math.round(R.range(200, 220));
    const g = Math.round(R.range(208, 222));
    const b = Math.round(R.range(214, 228));
    const a = R.range(0.45, 0.72);
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-rule");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 2.5");
    svg.innerHTML = `<path d="${rulePath(R)}" fill="none" stroke="rgba(${r},${g},${b},${a.toFixed(3)})" stroke-width="${R.range(0.85, 1.5).toFixed(2)}" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke"/>`;
    el.appendChild(svg);
    /* rule is its own projection — store mild dynamics for path nudge optional */
    objects.set(el, createObjectState(el, "rule"));
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

  const registerSolid = (el) => {
    if (objects.has(el)) return;
    const st = createObjectState(el, "solid");
    objects.set(el, st);
    el.dataset.f00Petal = "1";
    applySolidSpecimen(st);
  };

  const registerType = (el) => {
    if (objects.has(el)) return;
    if (el.closest?.(".splash-wrap.is-film .splash")) return;
    if (el.classList?.contains("glyph")) return;
    const st = createObjectState(el, "type");
    objects.set(el, st);
    el.dataset.f00PetalType = "1";
    applyTypeSpecimen(st);
  };

  const paintAll = (scope) => {
    const sc = scope || document;
    sc.querySelectorAll(solidSel).forEach(registerSolid);
    sc.querySelectorAll(typeSel).forEach(registerType);
  };

  const ruleHosts = () => {
    document.querySelectorAll(".top-inner").forEach((el) => attachRule(el, "bottom"));
    document.querySelectorAll(".foot, footer, .site-footer").forEach((el) =>
      attachRule(el, "top")
    );
    document.querySelectorAll(".section-head").forEach((el) => attachRule(el, "bottom"));
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

  /* ── each object integrates its own air ── */
  if (!reduced) {
    let t0 = performance.now();
    let last = t0;

    const tick = (now) => {
      const dt = Math.min(0.05, (now - last) / 1000);
      last = now;
      const t = (now - t0) / 1000;

      /* field projection (independent) */
      const field = objects.get(root);
      if (field && field.role === "field") {
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
        setRoot("--e-bg-x", `${field.bx.toFixed(3)}%`);
        setRoot("--e-bg-y", `${field.by.toFixed(3)}%`);
      }

      objects.forEach((st) => {
        if (st.role === "field") return;
        /* private oscillator + drift — this object’s own entropy rules */
        st.phi += dt * st.omega;
        st.driftX += dt * (-0.08 * st.driftX) + (Math.random() - 0.5) * 0.02 * Math.sqrt(dt);
        st.driftY += dt * (-0.08 * st.driftY) + (Math.random() - 0.5) * 0.02 * Math.sqrt(dt);
        st.wx = Math.sin(st.phi) * st.ampX + st.driftX;
        st.wy = Math.cos(st.phi * 0.91) * st.ampY + st.driftY;
        st.wr = Math.sin(st.phi * 0.37) * st.ampR;

        if (st.role === "solid" || st.role === "rule") {
          st.el.style.setProperty("--w-x", `${st.wx.toFixed(3)}px`);
          st.el.style.setProperty("--w-y", `${st.wy.toFixed(3)}px`);
          st.el.style.setProperty("--w-rot", `${st.wr.toFixed(4)}deg`);
        }
        if (st.role === "type") {
          /* type: only micro baseline breathe — keep copy readable */
          st.el.style.setProperty(
            "--e-t-y",
            `${(st.tY + st.wy * 0.15).toFixed(2)}px`
          );
        }
      });

      /* expose logo’s sibling plate state if present */
      const wrap = document.querySelector(".splash-wrap");
      if (wrap && objects.has(wrap)) {
        const st = objects.get(wrap);
        window.F00Projection.wind = {
          x: st.wx,
          y: st.wy,
          rot: st.wr,
          t,
          seed: st.seed,
        };
      }

      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }

  root.dataset.f00Projection = throwSalt.toString(16);
})();
