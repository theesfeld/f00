/* f00 — projection entropy (org-wide)
 *
 * When a surface is projected onto the display, seed a unique specimen.
 * ORDER IN DISORDER: variance is never uniform (not one global blur, not
 * identical box offsets). Snowflakes / zebra / drip — same species, never
 * clones. Zen band: readable, usable. No metronome. No CAD perfection.
 */
(() => {
  if (typeof document === "undefined") return;
  if (document.documentElement.dataset.f00Entropy === "1") return;
  document.documentElement.dataset.f00Entropy = "1";

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const root = document.documentElement;

  /* crypto-backed seed for this throw — never persisted */
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
    /* mulberry32 */
    s |= 0;
    s = (s + 0x6d2b79f5) | 0;
    let t = Math.imul(s ^ (s >>> 15), 1 | s);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
  const range = (a, b) => a + (b - a) * rnd();
  const signed = (m) => range(-m, m);

  const set = (k, v) => root.style.setProperty(k, v);

  /* —— field / emulsion / type (document specimen) —— */
  const bgX = 50 + signed(4.2);
  const bgY = 50 + signed(3.6);
  const bgS = 1 + signed(0.018);
  set("--e-bg-x", `${bgX.toFixed(3)}%`);
  set("--e-bg-y", `${bgY.toFixed(3)}%`);
  set("--e-bg-scale", bgS.toFixed(4));
  /* type tracking: independent X-ish feel via tracking only (still readable) */
  set("--e-track", `${signed(0.012).toFixed(4)}em`);
  /* chrome rule: imperfect, not one global line weight for all edges */
  set("--e-line-t", `${range(0.8, 1.25).toFixed(3)}px`);
  set("--e-line-b", `${range(0.8, 1.25).toFixed(3)}px`);
  set("--e-chrome-op", range(0.48, 0.72).toFixed(3));
  set("--e-chrome-op-b", range(0.45, 0.7).toFixed(3));
  set("--e-paper", range(0.96, 1.0).toFixed(4));
  /* legacy single-line fallbacks */
  set("--e-line", `var(--e-line-b)`);

  /* —— each box is its own organ under the skin (still grid-usable) —— */
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

  const paintBoxes = (rootEl) => {
    rootEl.querySelectorAll(boxSel).forEach((el) => {
      if (el.dataset.f00EntropyBox === "1") return;
      el.dataset.f00EntropyBox = "1";
      /* non-uniform: each edge of each box is its own slight throw */
      el.style.setProperty("--e-rot", `${signed(0.28).toFixed(3)}deg`);
      el.style.setProperty("--e-x", `${signed(1.8).toFixed(2)}px`);
      el.style.setProperty("--e-y", `${signed(1.6).toFixed(2)}px`);
      el.style.setProperty("--e-bw-t", `${range(1.45, 2.55).toFixed(2)}px`);
      el.style.setProperty("--e-bw-r", `${range(1.45, 2.55).toFixed(2)}px`);
      el.style.setProperty("--e-bw-b", `${range(1.45, 2.55).toFixed(2)}px`);
      el.style.setProperty("--e-bw-l", `${range(1.45, 2.55).toFixed(2)}px`);
      el.style.setProperty("--e-bw", `var(--e-bw-t)`); /* fallback */
      el.style.setProperty("--e-op", range(0.97, 1).toFixed(4));
    });
  };

  paintBoxes(document);

  /* SPA / catalog cards painted later */
  const mo = new MutationObserver((muts) => {
    for (const m of muts) {
      m.addedNodes.forEach((n) => {
        if (n.nodeType !== 1) return;
        if (n.matches && n.matches(boxSel)) paintBoxes(n.parentNode || document);
        else if (n.querySelectorAll) paintBoxes(n);
      });
    }
  });
  mo.observe(document.documentElement, { childList: true, subtree: true });

  /* continuous field air only — never jumps cards under the cursor */
  if (!reduced) {
    let bx = bgX;
    let by = bgY;
    let t0 = performance.now();
    let last = t0;
    /* independent slow phases (not a metronome loop length) */
    const p1 = range(0.05, 0.12);
    const p2 = range(0.07, 0.15);
    const p3 = range(0.03, 0.09);
    const a1 = range(0.35, 0.9);
    const a2 = range(0.25, 0.7);

    const tick = (now) => {
      const dt = Math.min(0.05, (now - last) / 1000);
      last = now;
      const t = (now - t0) / 1000;
      /* organic walk toward a moving attractor */
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
