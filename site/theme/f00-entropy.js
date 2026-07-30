/* f00 — Projection Specimen Engine
 *
 * Each element is its OWN PROJECTION onto the display:
 *   light → object-as-film → lens → screen
 * Logo, cards, header, footer, type, rules, field — each a complete throw
 * with private seed + private optical stack. Not “positions of petals.”
 * Zen band: readable / usable. Organic, never CAD-uniform.
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
    signed: (m) => -m + 2 * m * rnd(),
  });

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
    const pose = g.pose ?? 1;
    const blur = g.blur ?? 1;
    const persp = g.persp ?? 1;

    /* organic but zen — enough to break factory uniformity */
    return {
      el,
      role,
      seed,
      R,
      gateX: R.signed(1.65 * pose),
      gateY: R.signed(1.35 * pose),
      tiltX: R.signed(0.75 * persp),
      tiltY: R.signed(0.6 * persp),
      buckle: R.signed(0.018 * pose),
      rotZ: R.signed(0.55 * pose),
      /* border weights — never equal on all four sides */
      edgeT: R.range(1.15, 3.1),
      edgeR: R.range(1.15, 3.1),
      edgeB: R.range(1.15, 3.1),
      edgeL: R.range(1.15, 3.1),
      lamp: R.range(0.93, 1),
      track: R.signed(0.022),
      word: R.signed(0.035),
      baseY: R.signed(0.55),
      defocus0: R.range(0.04, 0.22) * blur,
      emul: R.range(0.7, 1.35),
      /* slight pad / content drift — cards stop looking cloned */
      padT: R.range(0.95, 1.12),
      padR: R.range(0.94, 1.1),
      padB: R.range(0.96, 1.14),
      padL: R.range(0.94, 1.1),
      phi: R.range(0, Math.PI * 2),
      /* slow rates — flow, not twitch */
      omega: R.range(0.12, 0.42),
      ampGate: R.range(0.15, 0.55) * pose,
      ampTilt: R.range(0.03, 0.12) * persp,
      ampDef: R.range(0.02, 0.08) * blur,
      ampRot: R.range(0.015, 0.06) * pose,
      liveGateX: 0,
      liveGateY: 0,
      liveTiltX: 0,
      liveTiltY: 0,
      liveDef: 0,
      liveRot: 0,
      /* displayed (lerped) — what actually hits the DOM */
      dispX: 0,
      dispY: 0,
      dispRot: 0,
    };
  };

  const projections = new Map();

  window.F00Projection = {
    throwSalt,
    seed: throwSalt,
    reduced,
    projections,
    seedFor,
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

  /** static specimen props once; live motion is transform only */
  const applyStaticCSS = (p) => {
    const el = p.el;
    el.style.setProperty("--p-lamp", p.lamp.toFixed(4));
    el.style.setProperty("--p-emul", p.emul.toFixed(3));
    el.style.setProperty("--p-defocus", "0px");
    if (p.role === "solid" || p.role === "plate" || p.role === "chrome") {
      el.style.setProperty("--e-bw-t", `${p.edgeT.toFixed(2)}px`);
      el.style.setProperty("--e-bw-r", `${p.edgeR.toFixed(2)}px`);
      el.style.setProperty("--e-bw-b", `${p.edgeB.toFixed(2)}px`);
      el.style.setProperty("--e-bw-l", `${p.edgeL.toFixed(2)}px`);
      el.style.setProperty("--e-op", p.lamp.toFixed(4));
      if (p.padT != null) {
        el.style.setProperty("--e-pad-t", p.padT.toFixed(3));
        el.style.setProperty("--e-pad-r", p.padR.toFixed(3));
        el.style.setProperty("--e-pad-b", p.padB.toFixed(3));
        el.style.setProperty("--e-pad-l", p.padL.toFixed(3));
      }
    }
    if (p.role === "type" || p.role === "chrome") {
      el.style.setProperty("--e-t-track", `${p.track.toFixed(4)}em`);
      el.style.setProperty("--e-t-word", `${p.word.toFixed(4)}em`);
      el.style.setProperty("--e-t-op", Math.min(1, p.lamp + 0.02).toFixed(4));
    }
  };

  /**
   * Hot path: one composite transform string — continuous, no stepped CSS vars.
   * (CSS-var transforms updated every N frames read as jerky idle motion.)
   */
  const applyLiveMotion = (p) => {
    const el = p.el;
    if (!el || p.role === "plate") return;
    if (el.classList?.contains("splash-wrap")) return;

    const gx = p.gateX + p.dispX;
    const gy = p.gateY + p.dispY;
    const rz = p.rotZ + p.dispRot;

    /* direct transform = compositor-friendly continuous flow */
    el.style.transform = `translate3d(${gx.toFixed(3)}px, ${gy.toFixed(3)}px, 0) rotate(${rz.toFixed(4)}deg)`;

    if (p.role === "type" || p.role === "chrome") {
      el.style.setProperty(
        "--e-t-y",
        `${(p.baseY + p.dispY * 0.2).toFixed(3)}px`
      );
    }
  };

  const applyProjectionCSS = (p) => {
    applyStaticCSS(p);
    /* seed displayed pose so first paint isn't zero */
    p.dispX = p.liveGateX;
    p.dispY = p.liveGateY;
    p.dispRot = p.liveRot;
    applyLiveMotion(p);
  };

  const register = (el, role, gains) => {
    if (!el || projections.has(el)) return;
    const p = createProjection(el, role, gains);
    projections.set(el, p);
    el.dataset.f00Projection = p.seed.toString(16);
    el.classList.add("f00-proj");
    applyProjectionCSS(p);
  };

  /* ── emulsion RULE: never a CAD hairline ── */
  const rulePath = (R) => {
    /* continuous organic path — enough Y wander to never read as CAD */
    const n = 16 + Math.floor(R.rnd() * 12);
    const mid = 4;
    let y = mid + R.signed(1.4);
    let d = `M 0 ${y.toFixed(3)}`;
    for (let i = 1; i <= n; i++) {
      const x = (i / n) * 100;
      /* correlated wander — line, not white noise scribble */
      y += R.signed(1.15);
      y = mid + (y - mid) * 0.68 + R.signed(0.5);
      y = Math.max(0.5, Math.min(7.5, y));
      const cx = x - 50 / n + R.signed(0.65);
      const cy = y + R.signed(0.85);
      d += ` Q ${cx.toFixed(3)} ${cy.toFixed(3)} ${x.toFixed(3)} ${y.toFixed(3)}`;
    }
    return d;
  };

  /**
   * Nearly-straight frame — pretty straight, never CAD-perfect.
   * viewBox 0 0 100 100. Micro wander only (~0.1–0.25 units).
   */
  const framePath = (R) => {
    const inset = 0.55 + R.range(0, 0.25);
    /* max deviation from a true straight edge */
    const amp = 0.08 + R.range(0, 0.12);
    const j = (m) => R.signed(m);
    const n = 3; /* few samples — smooth almost-lines, not scribbles */
    const pts = [];

    const edge = (count, at) => {
      for (let i = 0; i <= count; i++) {
        const t = i / count;
        const end = i === 0 || i === count;
        pts.push(at(t, end));
      }
    };

    /* top L→R */
    edge(n, (t, end) => [
      inset + t * (100 - 2 * inset) + j(end ? 0.04 : 0.06),
      inset + (end ? j(0.04) : j(amp)),
    ]);
    /* right T→B (skip first = shared corner) */
    for (let i = 1; i <= n; i++) {
      const t = i / n;
      const end = i === n;
      pts.push([
        100 - inset + (end ? j(0.04) : j(amp)),
        inset + t * (100 - 2 * inset) + j(end ? 0.04 : 0.06),
      ]);
    }
    /* bottom R→L */
    for (let i = 1; i <= n; i++) {
      const t = i / n;
      const end = i === n;
      pts.push([
        100 - inset - t * (100 - 2 * inset) + j(end ? 0.04 : 0.06),
        100 - inset + (end ? j(0.04) : j(amp)),
      ]);
    }
    /* left B→T (skip last corner — close to start) */
    for (let i = 1; i < n; i++) {
      const t = i / n;
      pts.push([
        inset + j(amp),
        100 - inset - t * (100 - 2 * inset) + j(0.06),
      ]);
    }

    const fmt = (p) => `${p[0].toFixed(3)} ${p[1].toFixed(3)}`;
    let d = `M ${fmt(pts[0])}`;
    for (let i = 1; i < pts.length; i++) {
      /* light control points — almost collinear, tiny bow */
      const a = pts[i - 1];
      const b = pts[i];
      const mx = (a[0] + b[0]) / 2 + j(amp * 0.35);
      const my = (a[1] + b[1]) / 2 + j(amp * 0.35);
      d += ` Q ${mx.toFixed(3)} ${my.toFixed(3)} ${fmt(b)}`;
    }
    d += " Z";

    /* objectBoundingBox clip path (0..1) — no transform tricks */
    const d01 = d.replace(/(-?\d+\.\d+|-?\d+)/g, (num) => {
      const v = parseFloat(num) / 100;
      return v.toFixed(5);
    });

    return { d, d01, inset, amp };
  };

  const attachRule = (el, side) => {
    if (!el || el.dataset.f00Rule === "1") return;
    el.dataset.f00Rule = "1";
    el.classList.add(
      "e-rule-host",
      side === "top" ? "e-rule-top" : "e-rule-bottom"
    );
    if (side === "bottom") el.style.borderBottom = "0";
    else el.style.borderTop = "0";

    const seed = seedFor(el, "rule:" + side);
    const R = makeBag(mulberry(seed));
    const r = Math.round(R.range(188, 218));
    const g = Math.round(R.range(196, 222));
    const b = Math.round(R.range(204, 228));
    const a = R.range(0.48, 0.82);
    const sw = R.range(1.05, 1.85);
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-rule");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 8");
    svg.innerHTML = `<path d="${rulePath(R)}" fill="none" stroke="rgba(${r},${g},${b},${a.toFixed(3)})" stroke-width="${sw.toFixed(2)}" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke"/>`;
    el.appendChild(svg);
    register(el, "rule", { pose: 0.3, blur: 0.4, persp: 0.2 });
  };

  const attachFrame = (el) => {
    if (!el || el.dataset.f00Frame === "1") return;
    el.dataset.f00Frame = "1";
    el.classList.add("e-frame-host");
    const seed = seedFor(el, "frame");
    const R = makeBag(mulberry(seed));
    const r = Math.round(R.range(160, 192));
    const g = Math.round(R.range(166, 198));
    const b = Math.round(R.range(174, 204));
    const a = R.range(0.7, 0.92);
    /* stroke in viewBox units — thin metal edge */
    const sw = 0.38 + R.range(0, 0.18);
    const { d, d01 } = framePath(R);
    const clipId = `e-clip-${seed.toString(16)}`;

    /*
     * Architecture:
     *  - Card CSS bg/border OFF (transparent)
     *  - SVG path FILL is the cream plate (cannot exist outside the path)
     *  - Stroke is the metal edge on that same path
     *  - clip-path on the host clips content + ::before watercolor to the path
     * No card-level SVG filter — filters paint outside and break the frame.
     */
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-frame");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 100");
    svg.innerHTML = `
      <defs>
        <clipPath id="${clipId}" clipPathUnits="objectBoundingBox">
          <path d="${d01}"/>
        </clipPath>
      </defs>
      <path class="e-frame-fill" d="${d}" fill="#E8DFD4"/>
      <path class="e-frame-stroke" d="${d}" fill="none"
        stroke="rgba(${r},${g},${b},${a.toFixed(3)})"
        stroke-width="${sw.toFixed(3)}"
        stroke-linejoin="round" stroke-linecap="round"/>
    `.trim();
    /* paint plate behind content */
    el.insertBefore(svg, el.firstChild);

    el.style.setProperty("background", "transparent", "important");
    el.style.setProperty("background-color", "transparent", "important");
    el.style.setProperty("border", "0", "important");
    el.style.setProperty("border-width", "0", "important");
    el.style.setProperty("box-shadow", "none", "important");
    el.style.filter = "none";
    /* content + watercolor ::before stay inside the plate */
    el.style.clipPath = `url(#${clipId})`;
    el.style.webkitClipPath = `url(#${clipId})`;
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
    if (role === "plate") return { pose: 0.9, blur: 0.9, persp: 0.7 };
    /* solid cards: readable organic — not wild, not factory grid */
    if (role === "solid") return { pose: 1.25, blur: 0.4, persp: 0.95 };
    if (role === "type") return { pose: 0.35, blur: 0.3, persp: 0.18 };
    if (role === "chrome") return { pose: 0.45, blur: 0.28, persp: 0.32 };
    return { pose: 0.7, blur: 0.4, persp: 0.5 };
  };

  const cardFrameSel =
    ".card, article.card, .panel, .box, .f00-box, .feature-card, .doc-card, .install-card, .benchmark-card, .tool-card, .release-card, .announcement";

  const eachMatch = (sc, sel, fn) => {
    if (sc && sc.nodeType === 1 && sc.matches?.(sel)) fn(sc);
    sc?.querySelectorAll?.(sel)?.forEach(fn);
  };

  const paint = (scope) => {
    const sc = scope || document;
    eachMatch(sc, solidSel, (el) => {
      const role = el.classList.contains("splash-wrap") ? "plate" : "solid";
      register(el, role, gainsFor(role));
      if (role === "solid" && el.matches(cardFrameSel)) attachFrame(el);
    });
    eachMatch(sc, typeSel, (el) => {
      if (el.closest?.(".splash-wrap.is-film .splash")) return;
      if (el.classList?.contains("glyph")) return;
      register(el, "type", gainsFor("type"));
    });
    eachMatch(sc, ".brand, .nav", (el) =>
      register(el, "chrome", gainsFor("chrome"))
    );
  };

  const ruleHosts = () => {
    document
      .querySelectorAll(".top-inner")
      .forEach((el) => attachRule(el, "bottom"));
    document
      .querySelectorAll(".foot, footer, .site-footer")
      .forEach((el) => attachRule(el, "top"));
    document
      .querySelectorAll(".section-head")
      .forEach((el) => attachRule(el, "bottom"));
    document
      .querySelectorAll(".card h3, .card .card-actions, .panel h3")
      .forEach((el) => attachRule(el, "bottom"));
  };

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
          n.matches?.(
            ".section-head, .foot, .top-inner, .card, article.card, .card h3, .card-actions"
          ) ||
          n.querySelector?.(
            ".section-head, .foot, .top-inner, .card, article.card, .card h3, .card-actions"
          )
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
      const dt = Math.min(0.048, (now - last) / 1000);
      last = now;
      const t = (now - t0) / 1000;
      const scrolling = root.dataset.f00Scrolling === "1";

      /* exp lerp factor — continuous flow, never stepped frames */
      const follow = 1 - Math.exp(-dt * (scrolling ? 4 : 7));

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
        field.bx += (tx - field.bx) * follow * 0.55;
        field.by += (ty - field.by) * follow * 0.55;
        root.style.setProperty("--e-bg-x", `${field.bx.toFixed(4)}%`);
        root.style.setProperty("--e-bg-y", `${field.by.toFixed(4)}%`);
      }

      /*
       * Continuous phase every frame. Lerp display toward target so motion
       * reads as liquid, not a 20fps sample-and-hold.
       * While scrolling: still advance phase lightly so settle is already flowing.
       */
      const ampScale = scrolling ? 0.35 : 1;
      projections.forEach((p) => {
        if (p.role === "field" || p.role === "plate") return;
        if (p.el?.classList?.contains("splash-wrap")) return;

        /* slow multi-rate drift — incommensurate periods = organic */
        p.phi += dt * p.omega * (scrolling ? 0.45 : 1);
        const ph = p.phi;
        const targetX =
          (Math.sin(ph) * 0.72 + Math.sin(ph * 0.37) * 0.28) *
          p.ampGate *
          ampScale;
        const targetY =
          (Math.cos(ph * 0.93) * 0.7 + Math.sin(ph * 0.51) * 0.3) *
          p.ampGate *
          0.85 *
          ampScale;
        const targetRot =
          (Math.sin(ph * 0.29) * 0.75 + Math.sin(ph * 0.11) * 0.25) *
          p.ampRot *
          ampScale;

        p.liveGateX = targetX;
        p.liveGateY = targetY;
        p.liveRot = targetRot;

        p.dispX += (targetX - p.dispX) * follow;
        p.dispY += (targetY - p.dispY) * follow;
        p.dispRot += (targetRot - p.dispRot) * follow;

        applyLiveMotion(p);
      });

      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }

  root.dataset.f00Projection = throwSalt.toString(16);
})();
