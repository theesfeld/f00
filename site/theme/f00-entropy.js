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
    const project =
      el.getAttribute?.("data-project") ||
      el.closest?.("[data-project]")?.getAttribute("data-project") ||
      "";
    const id =
      el.id ||
      (typeof el.className === "string" ? el.className : el.tagName) ||
      "node";
    const idx = el.parentElement
      ? Array.prototype.indexOf.call(el.parentElement.children, el)
      : 0;
    const path =
      role +
      "|" +
      project +
      "|" +
      id +
      "|" +
      idx +
      "|" +
      (el.getAttribute?.("href") || "") +
      "|" +
      (el.textContent || "").slice(0, 48);
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
      track: R.signed(0.035),
      word: R.signed(0.05),
      baseY: R.signed(0.65),
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
      /* beat theme !important letter-spacing on titles; body still uses vars */
      if (p.inCard) {
        const baseTrack = p.role === "type" && el.matches("h1,h2,h3,h4")
          ? 0.04
          : 0.02;
        el.style.setProperty(
          "letter-spacing",
          `${(baseTrack + p.track).toFixed(4)}em`,
          "important"
        );
        el.style.setProperty(
          "word-spacing",
          `${p.word.toFixed(4)}em`,
          "important"
        );
      }
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
      const ty = p.baseY + p.dispY * 0.35;
      el.style.setProperty("--e-t-y", `${ty.toFixed(3)}px`);
      /* include baseline drift in the same transform (no second layout) */
      if (p.inCard) {
        const gx = p.gateX + p.dispX;
        const gy = p.gateY + p.dispY + ty * 0.35;
        const rz = p.rotZ + p.dispRot;
        el.style.transform = `translate3d(${gx.toFixed(3)}px, ${gy.toFixed(3)}px, 0) rotate(${rz.toFixed(4)}deg)`;
      }
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
    p.inCard = !!el.closest?.(
      ".card, article.card, .panel, .box, .f00-box, .feature-card"
    );
    /* card body type: a bit more breath — readable, not drunk */
    if (p.inCard && role === "type") {
      p.ampGate *= 1.35;
      p.ampRot *= 1.25;
      p.track *= 1.4;
      p.word *= 1.3;
    }
    projections.set(el, p);
    el.dataset.f00Projection = p.seed.toString(16);
    el.classList.add("f00-proj");
    applyProjectionCSS(p);
  };

  /* ── emulsion RULE: universal — never a CAD hairline ── */
  const rulePath = (R) => {
    /* pretty-straight emulsion path — same language as header/footer */
    const n = 10 + Math.floor(R.rnd() * 8);
    const mid = 4;
    const amp = 0.45 + R.range(0, 0.55); /* visible, not wild */
    let y = mid + R.signed(amp * 0.6);
    let d = `M 0 ${y.toFixed(3)}`;
    for (let i = 1; i <= n; i++) {
      const x = (i / n) * 100;
      y += R.signed(amp * 0.55);
      y = mid + (y - mid) * 0.72 + R.signed(amp * 0.25);
      y = Math.max(1.2, Math.min(6.8, y));
      const cx = x - 50 / n + R.signed(0.35);
      const cy = y + R.signed(amp * 0.35);
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
    const amp = 0.05 + R.range(0, 0.08); /* ~pretty straight */
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
    /* beat theme !important CAD borders */
    if (side === "bottom") {
      el.style.setProperty("border-bottom", "0", "important");
      el.style.setProperty("border-bottom-width", "0", "important");
      el.style.setProperty("border-bottom-color", "transparent", "important");
    } else {
      el.style.setProperty("border-top", "0", "important");
      el.style.setProperty("border-top-width", "0", "important");
      el.style.setProperty("border-top-color", "transparent", "important");
    }

    const seed = seedFor(el, "rule:" + side);
    const R = makeBag(mulberry(seed));
    const inCard = !!el.closest?.(
      ".card, article.card, .panel, .box, .f00-box, .feature-card"
    );
    /* chrome rules = cool silver; in-card rules = warm poppy metal */
    let r, g, b, a, sw;
    if (inCard) {
      r = Math.round(R.range(180, 212));
      g = Math.round(R.range(70, 110));
      b = Math.round(R.range(40, 70));
      a = R.range(0.38, 0.62);
      sw = R.range(0.95, 1.45);
    } else {
      r = Math.round(R.range(188, 218));
      g = Math.round(R.range(196, 222));
      b = Math.round(R.range(204, 228));
      a = R.range(0.48, 0.82);
      sw = R.range(1.05, 1.85);
    }
    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("class", "e-rule");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("viewBox", "0 0 100 8");
    svg.innerHTML = `<path d="${rulePath(R)}" fill="none" stroke="rgba(${r},${g},${b},${a.toFixed(3)})" stroke-width="${sw.toFixed(2)}" stroke-linecap="round" stroke-linejoin="round" vector-effect="non-scaling-stroke"/>`;
    el.appendChild(svg);
    register(el, "rule", { pose: 0.25, blur: 0.3, persp: 0.15 });
  };

  const attachFrame = (el) => {
    if (!el || el.dataset.f00Frame === "1") return;
    el.dataset.f00Frame = "1";
    el.classList.add("e-frame-host");
    const seed = seedFor(el, "frame");
    const R = makeBag(mulberry(seed));
    /* metal edge */
    const er = Math.round(R.range(160, 192));
    const eg = Math.round(R.range(166, 198));
    const eb = Math.round(R.range(174, 204));
    const ea = R.range(0.7, 0.92);
    const sw = 0.38 + R.range(0, 0.18);
    const { d, d01 } = framePath(R);
    const id = seed.toString(16);
    const clipId = `e-clip-${id}`;
    const clipUserId = `e-clipu-${id}`;
    const toothId = `e-tooth-${id}`;
    const patId = `e-paper-${id}`;
    const g1 = `e-w1-${id}`;
    const g2 = `e-w2-${id}`;
    const g3 = `e-w3-${id}`;

    /*
     * Cream plate is never a flat hex:
     *  base cream (specimen-shifted) + soft wash ellipses + fiber tooth
     *  all clipped to the same organic path — nothing outside the frame.
     */
    /* wider specimen cream — clearly not one shared hex */
    const cr = Math.round(220 + R.range(0, 22));
    const cg = Math.round(210 + R.range(0, 20));
    const cb = Math.round(198 + R.range(0, 18));
    const cr2 = Math.round(Math.min(255, Math.max(190, cr + R.signed(14))));
    const cg2 = Math.round(Math.min(255, Math.max(185, cg + R.signed(12))));
    const cb2 = Math.round(Math.min(255, Math.max(175, cb + R.signed(12))));

    const wx = R.range(12, 48);
    const wy = R.range(10, 46);
    const wx2 = R.range(52, 90);
    const wy2 = R.range(48, 92);
    const wx3 = R.range(22, 78);
    const wy3 = R.range(18, 82);
    const toothF = 0.55 + R.range(0, 0.85); /* visible paper tooth */
    const toothOp = 0.16 + R.range(0, 0.14);
    const fiberOp = 0.28 + R.range(0, 0.22);
    const washA = 0.45 + R.range(0, 0.3);
    const washB = 0.18 + R.range(0, 0.2);
    const washC = 0.28 + R.range(0, 0.25);
    const papers = [
      "/theme/textures/hb-wc-a7.webp",
      "/theme/textures/hb-wc-b7.webp",
      "/theme/textures/hb-wc-c7.webp",
      "/theme/textures/hb-wc-d7.webp",
      "/theme/textures/hb-wc-e7.webp",
      "/theme/textures/hb-fiber-q9.webp",
      "/theme/textures/hb-wash-a-q9.webp",
      "/theme/textures/hb-wash-b-q9.webp",
    ];
    const paper = papers[Math.floor(R.rnd() * papers.length)];
    const paper2 = papers[Math.floor(R.rnd() * papers.length)];
    const pox = R.range(0, 50);
    const poy = R.range(0, 50);
    const psz = 40 + R.range(0, 70);
    const psz2 = 70 + R.range(0, 80);
    const pox2 = R.range(0, 60);
    const poy2 = R.range(0, 60);

    /* also seed CSS wash vars so ::before/::after reinforce if present */
    el.style.setProperty("--wc-x", `${wx.toFixed(1)}%`);
    el.style.setProperty("--wc-y", `${wy.toFixed(1)}%`);
    el.style.setProperty("--wc-x2", `${wx2.toFixed(1)}%`);
    el.style.setProperty("--wc-y2", `${wy2.toFixed(1)}%`);
    el.style.setProperty("--wc-pos", `${pox.toFixed(0)}% ${poy.toFixed(0)}%`);
    el.style.setProperty("--wc-size", `${(140 + R.range(0, 60)).toFixed(0)}% ${(140 + R.range(0, 60)).toFixed(0)}%`);
    el.style.setProperty("--wc-wash-op", (0.55 + R.range(0, 0.28)).toFixed(2));
    el.style.setProperty("--wc-fiber-op", (0.32 + R.range(0, 0.22)).toFixed(2));
    el.style.setProperty(
      "--wc-a",
      `rgba(${Math.min(255, cr + 18)},${Math.min(255, cg + 14)},${Math.min(255, cb + 12)},${(0.4 + R.range(0, 0.25)).toFixed(2)})`
    );
    el.style.setProperty(
      "--wc-b",
      `rgba(${50 + Math.floor(R.rnd() * 60)},${12 + Math.floor(R.rnd() * 24)},${8 + Math.floor(R.rnd() * 16)},${(0.1 + R.range(0, 0.12)).toFixed(2)})`
    );
    el.style.setProperty(
      "--wc-c",
      `rgba(${170 + Math.floor(R.rnd() * 50)},${40 + Math.floor(R.rnd() * 50)},${18 + Math.floor(R.rnd() * 35)},${(0.08 + R.range(0, 0.12)).toFixed(2)})`
    );
    el.style.setProperty("--wc-paper", `url("${paper}")`);

    const pat2Id = `e-paper2-${id}`;
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
        <clipPath id="${clipUserId}" clipPathUnits="userSpaceOnUse">
          <path d="${d}"/>
        </clipPath>
        <radialGradient id="${g1}" cx="${wx.toFixed(1)}%" cy="${wy.toFixed(1)}%" r="${(48 + R.range(0, 28)).toFixed(1)}%">
          <stop offset="0%" stop-color="rgb(${Math.min(255, cr + 20)},${Math.min(255, cg + 16)},${Math.min(255, cb + 14)})" stop-opacity="${washA.toFixed(2)}"/>
          <stop offset="100%" stop-color="rgb(${cr},${cg},${cb})" stop-opacity="0"/>
        </radialGradient>
        <radialGradient id="${g2}" cx="${wx2.toFixed(1)}%" cy="${wy2.toFixed(1)}%" r="${(44 + R.range(0, 28)).toFixed(1)}%">
          <stop offset="0%" stop-color="rgb(${Math.max(0, cr - 28)},${Math.max(0, cg - 24)},${Math.max(0, cb - 20)})" stop-opacity="${washB.toFixed(2)}"/>
          <stop offset="100%" stop-color="rgb(${cr},${cg},${cb})" stop-opacity="0"/>
        </radialGradient>
        <radialGradient id="${g3}" cx="${wx3.toFixed(1)}%" cy="${wy3.toFixed(1)}%" r="${(34 + R.range(0, 26)).toFixed(1)}%">
          <stop offset="0%" stop-color="rgb(${cr2},${cg2},${cb2})" stop-opacity="${washC.toFixed(2)}"/>
          <stop offset="100%" stop-color="rgb(${cr},${cg},${cb})" stop-opacity="0"/>
        </radialGradient>
        <pattern id="${patId}" patternUnits="userSpaceOnUse" width="${psz.toFixed(1)}" height="${psz.toFixed(1)}"
          patternTransform="translate(${(-pox).toFixed(1)} ${(-poy).toFixed(1)}) rotate(${(R.signed(18)).toFixed(1)})">
          <image href="${paper}" width="${psz.toFixed(1)}" height="${psz.toFixed(1)}" preserveAspectRatio="xMidYMid slice"/>
        </pattern>
        <pattern id="${pat2Id}" patternUnits="userSpaceOnUse" width="${psz2.toFixed(1)}" height="${psz2.toFixed(1)}"
          patternTransform="translate(${(-pox2).toFixed(1)} ${(-poy2).toFixed(1)}) rotate(${(R.signed(25)).toFixed(1)})">
          <image href="${paper2}" width="${psz2.toFixed(1)}" height="${psz2.toFixed(1)}" preserveAspectRatio="xMidYMid slice"/>
        </pattern>
        <filter id="${toothId}" x="-2%" y="-2%" width="104%" height="104%" color-interpolation-filters="sRGB">
          <feTurbulence type="fractalNoise" baseFrequency="${toothF.toFixed(3)} ${(toothF * 1.3).toFixed(3)}" numOctaves="4" seed="${seed & 0xffff}" result="n"/>
          <feColorMatrix in="n" type="matrix" values="
            0 0 0 0 ${(cr / 255).toFixed(3)}
            0 0 0 0 ${(cg / 255).toFixed(3)}
            0 0 0 0 ${(cb / 255).toFixed(3)}
            0 0 0 ${toothOp.toFixed(3)} 0" result="grain"/>
          <feBlend in="SourceGraphic" in2="grain" mode="multiply"/>
        </filter>
      </defs>
      <g class="e-frame-plate" clip-path="url(#${clipUserId})">
        <rect class="e-frame-fill" x="0" y="0" width="100" height="100" fill="rgb(${cr},${cg},${cb})"/>
        <rect x="0" y="0" width="100" height="100" fill="url(#${g1})"/>
        <rect x="0" y="0" width="100" height="100" fill="url(#${g2})"/>
        <rect x="0" y="0" width="100" height="100" fill="url(#${g3})"/>
        <rect x="0" y="0" width="100" height="100" fill="url(#${patId})" opacity="${fiberOp.toFixed(3)}" style="mix-blend-mode:multiply"/>
        <rect x="0" y="0" width="100" height="100" fill="url(#${pat2Id})" opacity="${(fiberOp * 0.55).toFixed(3)}" style="mix-blend-mode:soft-light"/>
        <rect x="0" y="0" width="100" height="100" fill="rgb(${cr},${cg},${cb})" filter="url(#${toothId})" opacity="0.72"/>
      </g>
      <path class="e-frame-stroke" d="${d}" fill="none"
        stroke="rgba(${er},${eg},${eb},${ea.toFixed(3)})"
        stroke-width="${sw.toFixed(3)}"
        stroke-linejoin="round" stroke-linecap="round"/>
    `.trim();
    el.insertBefore(svg, el.firstChild);

    el.style.setProperty("background", "transparent", "important");
    el.style.setProperty("background-color", "transparent", "important");
    el.style.setProperty("border", "0", "important");
    el.style.setProperty("border-width", "0", "important");
    el.style.setProperty("box-shadow", "none", "important");
    el.style.filter = "none";
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
    /* solid cards: almost still — micro pose only (frames own the edge) */
    if (role === "solid") return { pose: 0.35, blur: 0.25, persp: 0.2 };
    /* body type: enough to feel projected, still readable */
    if (role === "type") return { pose: 0.55, blur: 0.35, persp: 0.22 };
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

  /**
   * Universal emulsion rules — same language on hub chrome, cards, lists.
   * side is which CAD border we replace.
   */
  const ruleHosts = () => {
    const bottoms = [
      ".top-inner",
      ".section-head",
      ".card h3",
      "article.card h3",
      ".panel h3",
      ".box h3",
      ".feature-card h3",
      ".card .facts li",
      "article.card .facts li",
      ".panel .facts li",
      "hr",
    ].join(",");
    const tops = [
      ".foot",
      "footer",
      ".site-footer",
      ".card .card-actions",
      "article.card .card-actions",
      ".panel .card-actions",
    ].join(",");
    document.querySelectorAll(bottoms).forEach((el) => attachRule(el, "bottom"));
    document.querySelectorAll(tops).forEach((el) => attachRule(el, "top"));
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
