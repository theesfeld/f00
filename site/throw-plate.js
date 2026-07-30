/* f00 hub splash — throw once, live with CSS
 *
 * Develop+project ONCE at max optical size (the look you love).
 * Scroll = CSS scale + gentle organic pose. Never re-raster every tick.
 * Optics evolve softly with p (large → more breath; docked → calm).
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
  let baseCssW = 0;
  let baseCssH = 0;

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
    const sb = splash.getBoundingClientRect();
    const root = document.documentElement;
    const maxPx =
      parseFloat(getComputedStyle(root).getPropertyValue("--splash-max")) ||
      fontPx;
    const restPx =
      parseFloat(getComputedStyle(root).getPropertyValue("--splash-rest")) ||
      fontPx * 0.35;
    return { fontPx, maxPx, restPx, sb };
  };

  /**
   * CSS pose for living plate: scale (scroll) + gentle organic motion.
   * Origin top-center so shrink rises under the header.
   * Keeps translate(-50%,0) so plate centers over the DOM mark.
   */
  const applyLivePose = (p, time) => {
    const { maxPx, restPx } = measureCss();
    const pp = Math.max(0, Math.min(1, p));
    const s =
      maxPx > 0
        ? (restPx + (maxPx - restPx) * (1 - pp)) / maxPx
        : 1 - pp * 0.55;
    const scale = Math.max(0.12, Math.min(1.05, s));

    /* breath amp: more free when large, calmer when docked — never wild */
    const breath = 0.22 + 0.78 * (1 - pp);
    const t = time || 0;
    /* slow entropic gate — size change itself modulates phase */
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
      const { maxPx, sb } = measureCss();
      if (sb.width < 4 && maxPx < 40) return;
      const dpr = Math.min(window.devicePixelRatio || 1, 1.25);
      /* develop at max optical size once — scale with CSS for shrink */
      const layoutW = Math.max(sb.width || maxPx * 1.5, maxPx * 1.55);
      const layoutH = Math.max(sb.height || maxPx * 0.9, maxPx * 0.95);
      const padX = Math.max(28, layoutW * 0.18);
      const padY = Math.max(28, layoutH * 0.22);
      const cssW = Math.ceil(layoutW + padX * 2);
      const cssH = Math.ceil(layoutH + padY * 2);
      /* cap raster — look stays; scroll stays free */
      const maxEdge = 900;
      let w = Math.floor(cssW * dpr);
      let h = Math.floor(cssH * dpr);
      const sc = Math.min(1, maxEdge / Math.max(w, h, 1));
      w = Math.max(160, Math.floor(w * sc));
      h = Math.max(120, Math.floor(h * sc));

      if (document.fonts?.load) {
        try {
          await document.fonts.load(
            `400 ${Math.round(maxPx * dpr * sc)}px Onyx`
          );
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
      baseCssW = cssW;
      baseCssH = cssH;
      displayToCanvas(canvas, result);
      canvas.style.width = `${cssW}px`;
      canvas.style.height = `${cssH}px`;
      /* top-center over splash mark (left:50% in CSS) */
      canvas.style.left = "50%";
      canvas.style.top = "0";
      applyLivePose(readP(), time || 0);
    } finally {
      busy = false;
    }
  };

  /** Cheap idle reproject only — never on scroll ticks. */
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

    /* track scroll motion — reproject only after settle */
    if (Math.abs(p - lastScrollP) > 0.002) {
      lastScrollP = p;
      scrollIdleAt = ts;
    } else if (
      !reduced &&
      cache &&
      ts - scrollIdleAt > 280 &&
      ts - lastIdleRep > 900
    ) {
      /* ~1fps while parked — living light, not scroll thrash */
      lastIdleRep = ts;
      idleReproject(t, p);
    }

    const op = opts.getOpacity?.() ?? 1;
    canvas.style.opacity = String(Math.max(0, Math.min(1, op)));
  };

  let resizeTimer = 0;
  const onResize = () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      if (performance.now() - lastFull < 250) return;
      lastFull = performance.now();
      fullThrow(performance.now() / 1000);
    }, 160);
  };
  window.addEventListener("resize", onResize, { passive: true });

  fullThrow(0).then(() => {
    lastFull = performance.now();
    scrollIdleAt = performance.now();
    if (!reduced) {
      raf = requestAnimationFrame(loop);
    } else {
      applyLivePose(readP(), 0);
    }
  });

  return {
    destroy() {
      running = false;
      cancelAnimationFrame(raf);
      window.removeEventListener("resize", onResize);
      clearTimeout(resizeTimer);
    },
    resize() {
      onResize();
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
