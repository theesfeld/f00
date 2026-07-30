/* f00 hub — splash mark via throw engine (develop → project → display)
 * Not CSS blur. Not uniform softness. Structure → emulsion → optics.
 */
import {
  throwTextPlate,
  displayToCanvas,
} from "./throw/engine/throw.js";

export function mountThrowPlate(opts) {
  const canvas = opts.canvas;
  const splash = opts.splashEl;
  if (!canvas || !splash) return null;

  const reduced = !!opts.staticOnly;
  let seed =
    (opts.seed != null
      ? opts.seed
      : window.F00Projection?.seedFor?.(canvas, "plate:splash") ??
        (Math.random() * 0xffffffff)) >>> 0;
  let running = true;
  let raf = 0;
  let lastThrow = 0;

  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';

  const render = async (time) => {
    const fontPx = opts.getFontPx?.() || 120;
    if (fontPx < 8) return;
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const sb = splash.getBoundingClientRect();
    const padX = Math.max(24, sb.width * 0.2);
    const padY = Math.max(24, sb.height * 0.26);
    const cssW = Math.ceil(sb.width + padX * 2);
    const cssH = Math.ceil(sb.height + padY * 2);
    const w = Math.max(64, Math.floor(cssW * dpr));
    const h = Math.max(64, Math.floor(cssH * dpr));

    if (document.fonts?.load) {
      try {
        await document.fonts.load(`400 ${fontPx * dpr}px Onyx`);
      } catch (_) {}
    }

    const result = throwTextPlate({
      text: opts.text || "f00",
      width: w,
      height: h,
      fontPx: fontPx * dpr,
      fontFamily,
      seed,
      time: time || 0,
    });
    displayToCanvas(canvas, result);
    canvas.style.width = `${cssW}px`;
    canvas.style.height = `${cssH}px`;
    const op = opts.getOpacity?.() ?? 1;
    canvas.style.opacity = String(Math.max(0, Math.min(1, op)));
  };

  const loop = (ts) => {
    if (!running) return;
    raf = requestAnimationFrame(loop);
    /* re-throw density with time is expensive; reproject every ~120ms for living air */
    if (ts - lastThrow > 120) {
      lastThrow = ts;
      /* keep seed — same specimen, living optics via time in project() */
      render(ts / 1000);
    }
    const op = opts.getOpacity?.() ?? 1;
    canvas.style.opacity = String(Math.max(0, Math.min(1, op)));
  };

  const start = () => {
    render(0).then(() => {
      if (!reduced) {
        lastThrow = performance.now();
        raf = requestAnimationFrame(loop);
      }
    });
  };
  start();

  return {
    destroy() {
      running = false;
      cancelAnimationFrame(raf);
    },
    resize() {
      render(performance.now() / 1000);
    },
  };
}

window.F00ThrowPlate = { mount: mountThrowPlate };

/* self-mount splash — module order vs classic defer is unreliable */
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
