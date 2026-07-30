/* f00 hub splash — throw at REAL max size, live with CSS
 *
 * Develop+project once at optical max. Scroll = CSS scale + soft pose.
 * Re-throw when --splash-max lands (app measures after boot) — never stuck
 * at the CSS fallback ~18rem.
 */
import {
  throwTextPlate,
  reprojectPlate,
  displayToCanvas,
} from "./throw/engine/throw.js";

export function mountThrowPlate(opts) {
  const canvas = opts.canvas;
  const splash = opts.splashEl;
  if (!canvas || !splash) return null;

  const reduced = !!opts.staticOnly;
  const seed =
    (opts.seed != null
      ? opts.seed
      : window.F00Projection?.seedFor?.(canvas, "plate:splash") ??
        Math.random() * 0xffffffff) >>> 0;

  let running = true;
  let raf = 0;
  let cache = null;
  let busy = false;
  let lastFull = 0;
  let lastScrollP = -1;
  let scrollIdleAt = 0;
  let lastIdleRep = 0;
  let thrownMaxPx = 0;
  let pendingThrow = 0;

  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';

  const readP = () => {
    if (opts.getP) return opts.getP();
    const v = parseFloat(
      getComputedStyle(document.documentElement).getPropertyValue("--p")
    );
    return Number.isFinite(v) ? v : 0;
  };

  const measureCss = () => {
    const fontPx = opts.getFontPx?.() || 120;
    const root = document.documentElement;
    const maxPx =
      parseFloat(getComputedStyle(root).getPropertyValue("--splash-max")) ||
      fontPx;
    const restPx =
      parseFloat(getComputedStyle(root).getPropertyValue("--splash-rest")) ||
      fontPx * 0.35;
    return { fontPx, maxPx, restPx };
  };

  /**
   * CSS pose: scale from max→rest with p + gentle organic breath.
   * Canvas is drawn at max; scale only shrinks — never re-raster on scroll.
   */
  const applyLivePose = (p, time) => {
    const { maxPx, restPx } = measureCss();
    const pp = Math.max(0, Math.min(1, p));
    const s =
      maxPx > 0
        ? (restPx + (maxPx - restPx) * (1 - pp)) / maxPx
        : 1 - pp * 0.55;
    const scale = Math.max(0.12, Math.min(1.05, s));

    const breath = 0.22 + 0.78 * (1 - pp);
    const t = time || 0;
    const phase = t * 0.31 + pp * 1.7;
    const ox = Math.sin(phase) * 0.55 * breath + Math.sin(pp * 2.4) * 0.35;
    const oy = Math.cos(phase * 0.87) * 0.4 * breath + Math.sin(pp * 1.9) * 0.2;
    const rot =
      Math.sin(phase * 0.45 + seed * 1e-9) * 0.22 * breath +
      Math.sin(pp * 1.3) * 0.12;
    const skewX =
      Math.sin(phase * 0.21) * 0.18 * breath + Math.cos(pp * 2.1) * 0.08;

    canvas.style.transformOrigin = "50% 0%";
    canvas.style.transform = [
      "translate(-50%, 0)",
      `scale(${scale.toFixed(4)})`,
      `translate(${ox.toFixed(2)}px, ${oy.toFixed(2)}px)`,
      `rotate(${rot.toFixed(3)}deg)`,
      `skewX(${skewX.toFixed(3)}deg)`,
    ].join(" ");
    canvas.dataset.throwScale = String(scale);
    canvas.dataset.throwP = String(pp.toFixed(3));
  };

  const fullThrow = async (time) => {
    if (busy) return;
    busy = true;
    try {
      const { maxPx } = measureCss();
      if (!(maxPx > 24)) return;

      /* size plate from measured max — ignore early small splash rect */
      const dpr = Math.min(window.devicePixelRatio || 1, 1.25);
      /* glyph box ≈ 1.3×w × 0.86×h of font; light pad for film fringe only */
      const layoutW = maxPx * 1.4;
      const layoutH = maxPx * 0.9;
      const padX = Math.max(18, layoutW * 0.08);
      const padY = Math.max(18, layoutH * 0.1);
      const cssW = Math.ceil(layoutW + padX * 2);
      const cssH = Math.ceil(layoutH + padY * 2);

      /* raster cap — CSS size stays full so scale looks correct */
      const maxEdge = 1000;
      let w = Math.floor(cssW * dpr);
      let h = Math.floor(cssH * dpr);
      const sc = Math.min(1, maxEdge / Math.max(w, h, 1));
      w = Math.max(200, Math.floor(w * sc));
      h = Math.max(140, Math.floor(h * sc));

      if (document.fonts?.load) {
        try {
          await document.fonts.load(
            `400 ${Math.round(maxPx * dpr * sc)}px Onyx`
          );
        } catch (_) {}
      }
      if (document.fonts?.ready) {
        try {
          await document.fonts.ready;
        } catch (_) {}
      }

      const result = throwTextPlate({
        text: opts.text || "f00",
        width: w,
        height: h,
        fontPx: Math.round(maxPx * dpr * sc),
        fontFamily,
        seed,
        time: time || 0,
      });
      cache = {
        density: result.density,
        optics: result.optics,
        w,
        h,
        cssW,
        cssH,
        maxPx,
      };
      thrownMaxPx = maxPx;
      displayToCanvas(canvas, result);
      canvas.style.width = `${cssW}px`;
      canvas.style.height = `${cssH}px`;
      canvas.style.left = "50%";
      canvas.style.top = "0";
      applyLivePose(readP(), time || 0);
    } finally {
      busy = false;
    }
  };

  const scheduleThrow = (why) => {
    clearTimeout(pendingThrow);
    pendingThrow = setTimeout(() => {
      if (!running) return;
      const { maxPx } = measureCss();
      if (
        thrownMaxPx > 0 &&
        Math.abs(maxPx - thrownMaxPx) / Math.max(thrownMaxPx, 1) < 0.04
      ) {
        return; /* already at this size */
      }
      lastFull = performance.now();
      fullThrow(performance.now() / 1000);
    }, why === "boot" ? 0 : 40);
  };

  const idleReproject = (time, p) => {
    if (!cache || busy || reduced) return;
    const liveAmp = 0.25 + 0.55 * (1 - Math.max(0, Math.min(1, p)));
    const result = reprojectPlate(
      cache.density,
      cache.w,
      cache.h,
      cache.optics,
      seed,
      time,
      liveAmp
    );
    displayToCanvas(canvas, result);
  };

  const loop = (ts) => {
    if (!running) return;
    raf = requestAnimationFrame(loop);
    const p = readP();
    const t = ts / 1000;
    applyLivePose(p, t);

    /* catch app.js measure landing after our first throw */
    const { maxPx } = measureCss();
    if (
      maxPx > 40 &&
      (thrownMaxPx < 40 ||
        Math.abs(maxPx - thrownMaxPx) / Math.max(thrownMaxPx, 1) > 0.06)
    ) {
      if (performance.now() - lastFull > 180) scheduleThrow("max-changed");
    }

    if (Math.abs(p - lastScrollP) > 0.002) {
      lastScrollP = p;
      scrollIdleAt = ts;
    } else if (
      !reduced &&
      cache &&
      ts - scrollIdleAt > 280 &&
      ts - lastIdleRep > 900
    ) {
      lastIdleRep = ts;
      idleReproject(t, p);
    }

    const op = opts.getOpacity?.() ?? 1;
    canvas.style.opacity = String(Math.max(0, Math.min(1, op)));
  };

  let resizeTimer = 0;
  const onResize = () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => scheduleThrow("resize"), 120);
  };
  window.addEventListener("resize", onResize, { passive: true });

  const start = async () => {
    if (document.fonts?.ready) {
      try {
        await document.fonts.ready;
      } catch (_) {}
    }
    /* wait one frame so app.js measure can set --splash-max */
    await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
    await fullThrow(0);
    lastFull = performance.now();
    scrollIdleAt = performance.now();
    if (!reduced) raf = requestAnimationFrame(loop);
    else applyLivePose(readP(), 0);
  };
  start();

  return {
    destroy() {
      running = false;
      cancelAnimationFrame(raf);
      clearTimeout(pendingThrow);
      clearTimeout(resizeTimer);
      window.removeEventListener("resize", onResize);
    },
    resize() {
      scheduleThrow("resize");
    },
    rethrow() {
      thrownMaxPx = 0;
      scheduleThrow("force");
    },
  };
}

window.F00ThrowPlate = { mount: mountThrowPlate };

const boot = () => {
  const canvas = document.querySelector("canvas.splash-film");
  const splash = document.querySelector(".splash");
  const wrap = document.querySelector(".splash-wrap");
  if (!canvas || !splash || !wrap) return;
  if (wrap.dataset.throwMounted === "1") return;
  wrap.dataset.throwMounted = "1";
  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const handle = mountThrowPlate({
    canvas,
    splashEl: splash,
    text: "f00",
    staticOnly: reduced,
    getFontPx: () => {
      const fs = parseFloat(getComputedStyle(splash).fontSize);
      return Number.isFinite(fs) ? fs : 120;
    },
    getP: () => {
      const v = parseFloat(
        getComputedStyle(document.documentElement).getPropertyValue("--p")
      );
      return Number.isFinite(v) ? v : 0;
    },
    getOpacity: () => {
      const v = parseFloat(
        getComputedStyle(document.documentElement).getPropertyValue("--dock-op")
      );
      return Number.isFinite(v) ? v : 1;
    },
  });
  if (handle) wrap.classList.add("is-film");
  window.__f00ThrowHandle = handle;
};
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", boot);
} else {
  boot();
}
