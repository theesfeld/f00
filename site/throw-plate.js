/* f00 hub splash — throw engine: develop once, project continuously */
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
  let cache = null; /* { density, optics, w, h, cssW, cssH, fontKey } */
  let lastGeom = "";
  let busy = false;

  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';

  const geomKey = (fontPx, cssW, cssH, dpr) =>
    `${fontPx}|${cssW}|${cssH}|${dpr}`;

  const fullThrow = async (time) => {
    if (busy) return;
    busy = true;
    try {
      const fontPx = opts.getFontPx?.() || 120;
      if (fontPx < 8) return;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const sb = splash.getBoundingClientRect();
      if (sb.width < 8 || sb.height < 8) return;
      const padX = Math.max(32, sb.width * 0.22);
      const padY = Math.max(32, sb.height * 0.28);
      const cssW = Math.ceil(sb.width + padX * 2);
      const cssH = Math.ceil(sb.height + padY * 2);
      /* cap pixels for performance while keeping detail */
      const maxEdge = 1400;
      let w = Math.floor(cssW * dpr);
      let h = Math.floor(cssH * dpr);
      const scale = Math.min(1, maxEdge / Math.max(w, h));
      w = Math.max(128, Math.floor(w * scale));
      h = Math.max(96, Math.floor(h * scale));
      const fontKey = geomKey(fontPx, cssW, cssH, dpr);

      if (document.fonts?.load) {
        try {
          await document.fonts.load(`400 ${Math.round(fontPx * dpr * scale)}px Onyx`);
        } catch (_) {}
      }

      const result = throwTextPlate({
        text: opts.text || "f00",
        width: w,
        height: h,
        fontPx: Math.round(fontPx * dpr * scale),
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
        fontKey,
      };
      displayToCanvas(canvas, result);
      canvas.style.width = `${cssW}px`;
      canvas.style.height = `${cssH}px`;
      lastGeom = fontKey;
    } finally {
      busy = false;
    }
  };

  const reproject = (time) => {
    if (!cache || busy) return;
    const result = reprojectPlate(
      cache.density,
      cache.w,
      cache.h,
      cache.optics,
      seed,
      time
    );
    displayToCanvas(canvas, result);
  };

  let lastRep = 0;
  const loop = (ts) => {
    if (!running) return;
    raf = requestAnimationFrame(loop);
    const fontPx = opts.getFontPx?.() || 120;
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const sb = splash.getBoundingClientRect();
    const cssW = Math.ceil(sb.width + Math.max(32, sb.width * 0.22) * 2);
    const cssH = Math.ceil(sb.height + Math.max(32, sb.height * 0.28) * 2);
    const gk = geomKey(fontPx, cssW, cssH, dpr);
    if (gk !== lastGeom) {
      fullThrow(ts / 1000);
    } else if (ts - lastRep > 50) {
      lastRep = ts;
      reproject(ts / 1000);
    }
    const op = opts.getOpacity?.() ?? 1;
    canvas.style.opacity = String(Math.max(0, Math.min(1, op)));
  };

  fullThrow(0).then(() => {
    if (!reduced) {
      lastRep = performance.now();
      raf = requestAnimationFrame(loop);
    }
  });

  return {
    destroy() {
      running = false;
      cancelAnimationFrame(raf);
    },
    resize() {
      fullThrow(performance.now() / 1000);
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
