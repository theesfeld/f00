/* f00 hub splash — throw once at max, butter-smooth CSS scale on scroll
 *
 * Layout size stays locked at max (no font-size reflow).
 * Scroll drives a lerped p → pure GPU transform. Never re-raster mid-scroll.
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
  let lastIdleRep = 0;
  let scrollIdleAt = 0;
  let thrownMaxPx = 0;
  let pendingThrow = 0;

  /* cached metrics — no getComputedStyle in the hot path */
  let maxPx = 120;
  let restPx = 56;
  let shrinkRange = 400;

  /* smoothed progress — kills discrete scroll-tick jumps */
  let smoothP = 0;
  let smoothOp = 1;
  let targetP = 0;
  let targetOp = 1;

  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';

  const refreshMetrics = () => {
    const root = document.documentElement;
    const cs = getComputedStyle(root);
    const m = parseFloat(cs.getPropertyValue("--splash-max"));
    const r = parseFloat(cs.getPropertyValue("--splash-rest"));
    const sh = parseFloat(cs.getPropertyValue("--shrink-range"));
    if (Number.isFinite(m) && m > 24) maxPx = m;
    if (Number.isFinite(r) && r > 8) restPx = r;
    if (Number.isFinite(sh) && sh > 32) shrinkRange = sh;
    else shrinkRange = Math.max(64, (maxPx - restPx) * 0.86);
  };

  const readTargetP = () => {
    if (opts.getP) {
      const v = opts.getP();
      if (Number.isFinite(v)) return Math.max(0, Math.min(1, v));
    }
    /* direct from scroll — no CSS lag */
    const y = window.scrollY || 0;
    return Math.max(0, Math.min(1, y / Math.max(shrinkRange, 1)));
  };

  const readTargetOp = () => {
    if (opts.getOpacity) {
      const v = opts.getOpacity();
      if (Number.isFinite(v)) return Math.max(0, Math.min(1, v));
    }
    const y = window.scrollY || 0;
    if (y <= shrinkRange) return 1;
    const restH = restPx * 0.84;
    const fadeDist = Math.max(72, restH * 0.95);
    return Math.max(0, 1 - (y - shrinkRange) / fadeDist);
  };

  /**
   * GPU-only pose. Scale is the scroll story; breath is slow + tiny.
   */
  const applyLivePose = (p, op, time) => {
    const pp = Math.max(0, Math.min(1, p));
    const s =
      maxPx > 0
        ? (restPx + (maxPx - restPx) * (1 - pp)) / maxPx
        : 1 - pp * 0.55;
    const scale = Math.max(0.12, Math.min(1.05, s));

    /* slow organic drift — never high-frequency jitter */
    const breath = 0.18 + 0.55 * (1 - pp);
    const t = time || 0;
    const phase = t * 0.18; /* ~slow */
    const ox = Math.sin(phase) * 0.35 * breath;
    const oy = Math.cos(phase * 0.91) * 0.25 * breath;
    const rot = Math.sin(phase * 0.37 + seed * 1e-9) * 0.12 * breath;

    canvas.style.transform = [
      "translate3d(-50%, 0, 0)",
      `scale3d(${scale.toFixed(5)}, ${scale.toFixed(5)}, 1)`,
      `translate3d(${ox.toFixed(3)}px, ${oy.toFixed(3)}px, 0)`,
      `rotate(${rot.toFixed(4)}deg)`,
    ].join(" ");
    canvas.style.opacity = String(op);
    canvas.dataset.throwScale = String(scale);
    canvas.dataset.throwP = String(pp.toFixed(4));
  };

  const fullThrow = async (time) => {
    if (busy) return;
    busy = true;
    try {
      refreshMetrics();
      if (!(maxPx > 24)) return;

      const dpr = Math.min(window.devicePixelRatio || 1, 1.25);
      const layoutW = maxPx * 1.4;
      const layoutH = maxPx * 0.9;
      const padX = Math.max(18, layoutW * 0.08);
      const padY = Math.max(18, layoutH * 0.1);
      const cssW = Math.ceil(layoutW + padX * 2);
      const cssH = Math.ceil(layoutH + padY * 2);

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
      canvas.style.transformOrigin = "50% 0%";
      canvas.style.willChange = "transform, opacity";
      applyLivePose(smoothP, smoothOp, time || 0);
    } finally {
      busy = false;
    }
  };

  const scheduleThrow = (why) => {
    clearTimeout(pendingThrow);
    pendingThrow = setTimeout(() => {
      if (!running) return;
      refreshMetrics();
      if (
        thrownMaxPx > 0 &&
        Math.abs(maxPx - thrownMaxPx) / Math.max(thrownMaxPx, 1) < 0.04 &&
        why !== "force"
      ) {
        return;
      }
      lastFull = performance.now();
      fullThrow(performance.now() / 1000);
    }, why === "boot" ? 0 : 50);
  };

  let lastTs = 0;
  const loop = (ts) => {
    if (!running) return;
    raf = requestAnimationFrame(loop);

    const dt = lastTs ? Math.min(0.05, (ts - lastTs) / 1000) : 0.016;
    lastTs = ts;

    targetP = readTargetP();
    targetOp = readTargetOp();

    /* exp lerp — higher when catching up fast scroll, still smooth */
    const k = 1 - Math.exp(-dt * 14); /* ~snappy but not snapped */
    const prevP = smoothP;
    smoothP += (targetP - smoothP) * k;
    smoothOp += (targetOp - smoothOp) * k;
    /* snap when essentially there — avoid eternal micro-drift */
    if (Math.abs(targetP - smoothP) < 0.0004) smoothP = targetP;
    if (Math.abs(targetOp - smoothOp) < 0.001) smoothOp = targetOp;

    applyLivePose(smoothP, smoothOp, ts / 1000);

    /* signal scroll activity for entropy / others */
    const moving = Math.abs(smoothP - prevP) > 0.0002 || Math.abs(targetP - smoothP) > 0.002;
    if (moving) {
      scrollIdleAt = ts;
      document.documentElement.dataset.f00Scrolling = "1";
    } else if (ts - scrollIdleAt > 140) {
      if (document.documentElement.dataset.f00Scrolling === "1") {
        document.documentElement.dataset.f00Scrolling = "0";
      }
    }

    /* re-throw only if measured max jumped (not on scroll) */
    if (
      !moving &&
      maxPx > 40 &&
      (thrownMaxPx < 40 ||
        Math.abs(maxPx - thrownMaxPx) / Math.max(thrownMaxPx, 1) > 0.06) &&
      performance.now() - lastFull > 240
    ) {
      refreshMetrics();
      scheduleThrow("max-changed");
    }

    /* living light only after full settle — never during motion */
    if (
      !reduced &&
      cache &&
      !busy &&
      !moving &&
      ts - scrollIdleAt > 1200 &&
      ts - lastIdleRep > 2500
    ) {
      lastIdleRep = ts;
      const liveAmp = 0.2 + 0.45 * (1 - smoothP);
      const result = reprojectPlate(
        cache.density,
        cache.w,
        cache.h,
        cache.optics,
        seed,
        ts / 1000,
        liveAmp
      );
      displayToCanvas(canvas, result);
    }
  };

  let resizeTimer = 0;
  const onResize = () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      refreshMetrics();
      scheduleThrow("resize");
    }, 140);
  };
  window.addEventListener("resize", onResize, { passive: true });

  const start = async () => {
    refreshMetrics();
    if (document.fonts?.ready) {
      try {
        await document.fonts.ready;
      } catch (_) {}
    }
    await new Promise((r) =>
      requestAnimationFrame(() => requestAnimationFrame(r))
    );
    refreshMetrics();
    targetP = readTargetP();
    smoothP = targetP;
    targetOp = readTargetOp();
    smoothOp = targetOp;
    await fullThrow(0);
    lastFull = performance.now();
    scrollIdleAt = performance.now();
    if (!reduced) raf = requestAnimationFrame(loop);
    else applyLivePose(smoothP, smoothOp, 0);
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
      refreshMetrics();
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
    /* live scroll math inside plate — no CSS-var lag */
    getP: null,
    getOpacity: null,
  });
  if (handle) wrap.classList.add("is-film");
  window.__f00ThrowHandle = handle;
};
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", boot);
} else {
  boot();
}
