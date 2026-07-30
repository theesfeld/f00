/* f00 hub splash — throw DEVELOP once (CPU), PROJECT living (WebGL)
 *
 * Scroll shrink is CSS transform only (compositor). GPU lives at a
 * throttled rate and is paused while the finger is moving on mobile.
 */
import {
  develop,
  rasterizeTextMask,
  sampleOptics,
} from "./throw/engine/throw.js";
import { createGlProjector } from "./throw/engine/gl-project.js";
import { mulberry32 } from "./throw/engine/math.js";

function detectMobileBudget() {
  try {
    if (window.matchMedia("(pointer: coarse)").matches) return true;
    if (/iPhone|iPad|iPod|Android/i.test(navigator.userAgent || "")) return true;
    if (window.matchMedia("(max-width: 900px)").matches) return true;
  } catch (_) {}
  return false;
}

export function mountThrowPlate(opts) {
  const canvas = opts.canvas;
  const splash = opts.splashEl;
  if (!canvas || !splash) return null;

  const reduced = !!opts.staticOnly;
  const mobile = detectMobileBudget();
  const seed =
    (opts.seed != null
      ? opts.seed
      : window.F00Projection?.seedFor?.(canvas, "plate:splash") ??
        Math.random() * 0xffffffff) >>> 0;

  const rnd = mulberry32(seed);
  const R = {
    rnd,
    range: (a, b) => a + (b - a) * rnd(),
    signed: (m) => -m + 2 * m * rnd(),
  };
  const optics = sampleOptics(seed ^ 0xc0ffee, R);

  let running = true;
  let raf = 0;
  let busy = false;
  let lastFull = 0;
  let scrollIdleAt = 0;
  let thrownMaxPx = 0;
  let pendingThrow = 0;
  let projector = null;
  let lastGlDraw = 0;
  let lastScrollY = -1;

  let maxPx = 120;
  let restPx = 56;
  let shrinkRange = 400;
  let headerH = 54;

  let smoothP = 0;
  let targetP = 0;
  let dispScale = 1;
  let lastTs = 0;
  let inkHeightFrac = 0.78;
  let baseCssH = 0;
  let restScale = 0.35;
  let travelY = 0; /* px from band-center → header-center at p=1 */

  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';

  const refreshMetrics = () => {
    const root = document.documentElement;
    const cs = getComputedStyle(root);
    const m = parseFloat(cs.getPropertyValue("--splash-max"));
    const r = parseFloat(cs.getPropertyValue("--splash-rest"));
    const sh = parseFloat(cs.getPropertyValue("--shrink-range"));
    const hh = parseFloat(cs.getPropertyValue("--header-h"));
    if (Number.isFinite(m) && m > 24) maxPx = m;
    if (Number.isFinite(r) && r > 8) restPx = r;
    if (Number.isFinite(hh) && hh > 20) headerH = hh;
    if (Number.isFinite(sh) && sh > 32) shrinkRange = sh;
    else shrinkRange = Math.max(64, (maxPx - restPx) * 0.86);

    /* vertical travel: full-viewport center → header mid (no chrome at rest) */
    const viewH = window.innerHeight || 800;
    const bandMid = viewH * 0.5;
    const headMid = headerH * 0.5;
    travelY = headMid - bandMid;
  };

  const readTargetP = () => {
    if (opts.getP) {
      const v = opts.getP();
      if (Number.isFinite(v)) return Math.max(0, Math.min(1, v));
    }
    const y = window.scrollY || 0;
    return Math.max(0, Math.min(1, y / Math.max(shrinkRange, 1)));
  };

  const applyCssScale = (p, follow) => {
    const pp = Math.max(0, Math.min(1, p));
    const targetScale = Math.max(
      0.05,
      Math.min(1.05, 1 - pp * (1 - restScale))
    );
    const f = follow ?? 1;
    dispScale += (targetScale - dispScale) * f;

    /*
     * Compositor-only: center of band + rise into header + scale.
     * Frame layout does not change with p (iOS-safe).
     * When hard-docked, travel is 0 (frame is already the header bar).
     */
    const docked =
      pp > 0.88 || document.documentElement.classList.contains("logo-docked");
    const y = docked ? 0 : travelY * pp;

    canvas.style.left = "50%";
    canvas.style.top = "50%";
    canvas.style.transformOrigin = "50% 50%";
    canvas.style.transform = [
      "translate3d(-50%, -50%, 0)",
      `translate3d(0, ${y.toFixed(2)}px, 0)`,
      `scale3d(${dispScale.toFixed(5)}, ${dispScale.toFixed(5)}, 1)`,
    ].join(" ");
    canvas.style.opacity = "1";
    canvas.dataset.throwScale = String(dispScale);
    canvas.dataset.throwP = String(pp.toFixed(4));
  };

  const ensureGl = () => {
    if (projector) return projector;
    projector = createGlProjector(canvas, {
      quality: mobile ? "fast" : "high",
    });
    return projector;
  };

  /** CPU develop once at measured max — emulsion specimen. */
  const fullDevelop = async () => {
    if (busy) return;
    busy = true;
    try {
      refreshMetrics();
      if (!(maxPx > 24)) return;

      /* iOS: smaller backing store — looks sharp enough at rest scale */
      const dpr = Math.min(
        window.devicePixelRatio || 1,
        mobile ? 1.15 : 1.5
      );
      const layoutW = maxPx * 1.35;
      const layoutH = maxPx * 0.88;
      const padX = Math.max(mobile ? 28 : 36, layoutW * (mobile ? 0.12 : 0.14));
      const padY = Math.max(mobile ? 32 : 40, layoutH * (mobile ? 0.14 : 0.16));
      const cssW = Math.ceil(layoutW + padX * 2);
      const cssH = Math.ceil(layoutH + padY * 2);

      const maxEdge = mobile ? 560 : 1100;
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
      if (document.fonts?.ready) {
        try {
          await document.fonts.ready;
        } catch (_) {}
      }

      const fontPx = Math.round(maxPx * dpr * sc);
      const mask = rasterizeTextMask(w, h, opts.text || "f00", fontPx, fontFamily);
      const density = develop(mask, w, h, {
        seed: seed ^ 0x0deb,
        temperature: 0.45 + R.range(0, 0.45),
        time: 0.5 + R.range(0, 0.5),
        agitation: R.range(0.25, 0.85),
      });

      const glp = ensureGl();
      if (!glp) {
        console.warn("[throw-plate] WebGL unavailable — no living projector");
        return;
      }
      glp.uploadDensity(density, w, h);

      let inkTop = -1;
      let inkBot = -1;
      for (let y = 0; y < h; y++) {
        for (let x = 0; x < w; x++) {
          if (density[y * w + x] > 0.08) {
            if (inkTop < 0) inkTop = y;
            inkBot = y;
            break;
          }
        }
      }
      if (inkTop < 0) {
        inkHeightFrac = 0.78;
      } else {
        inkHeightFrac = Math.max(0.4, (inkBot - inkTop + 1) / h);
      }
      baseCssH = cssH;
      const headerBudget = Math.max(
        28,
        (parseFloat(
          getComputedStyle(document.documentElement).getPropertyValue(
            "--header-h"
          )
        ) || 54) * 0.72
      );
      const targetInkH = Math.max(28, Math.min(headerBudget, restPx * 0.85));
      const fullInkH = inkHeightFrac * cssH;
      restScale =
        fullInkH > 1
          ? Math.max(0.05, Math.min(0.2, targetInkH / fullInkH))
          : Math.min(0.18, restPx / Math.max(maxPx, 1));

      thrownMaxPx = maxPx;
      canvas.style.width = `${cssW}px`;
      canvas.style.height = `${cssH}px`;
      canvas.style.willChange = "transform";
      /* promote layer once */
      canvas.style.webkitBackfaceVisibility = "hidden";
      canvas.style.backfaceVisibility = "hidden";
      applyCssScale(smoothP, 1);
      /* first living frame */
      if (!reduced) {
        glp.draw({
          time: performance.now() / 1000,
          p: smoothP,
          liveAmp: 0.28 + 0.72 * (1 - smoothP),
          optics,
          seed,
        });
        lastGlDraw = performance.now();
      }
    } finally {
      busy = false;
    }
  };

  const scheduleDevelop = (why) => {
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
      fullDevelop();
    }, why === "boot" ? 0 : 50);
  };

  const loop = (ts) => {
    if (!running) return;
    raf = requestAnimationFrame(loop);

    const dt = lastTs ? Math.min(0.048, (ts - lastTs) / 1000) : 0.016;
    lastTs = ts;

    targetP = readTargetP();
    const prevP = smoothP;
    /*
     * Snappy follow — especially on touch. Lag made shrink feel "insane"
     * even when the device could keep up with layout.
     */
    const snappiness = mobile ? 32 : 22;
    const k = 1 - Math.exp(-dt * snappiness);
    smoothP += (targetP - smoothP) * k;
    /* snap when essentially there — avoids endless micro-lerp */
    if (Math.abs(targetP - smoothP) < 0.0008) smoothP = targetP;

    applyCssScale(smoothP, 1);

    const scrollY = window.scrollY || 0;
    const scrollDelta = Math.abs(scrollY - lastScrollY);
    lastScrollY = scrollY;

    const moving =
      Math.abs(smoothP - prevP) > 0.0002 ||
      Math.abs(targetP - smoothP) > 0.001 ||
      scrollDelta > 0.5;

    if (moving) {
      scrollIdleAt = ts;
      document.documentElement.dataset.f00Scrolling = "1";
    } else if (ts - scrollIdleAt > 140) {
      if (document.documentElement.dataset.f00Scrolling === "1") {
        document.documentElement.dataset.f00Scrolling = "0";
      }
    }

    /*
     * GPU budget:
     *  - while finger is moving on mobile: pure CSS scale (0 GL)
     *  - desktop scroll: ~20fps living
     *  - idle hero: ~24fps mobile / 30fps desktop
     *  - docked: ~10fps (still alive, cheap)
     */
    if (projector && !reduced) {
      const docked = smoothP > 0.88;
      let interval = mobile ? 42 : 33;
      if (moving) interval = mobile ? 1e9 : 50;
      else if (docked) interval = mobile ? 100 : 80;

      if (ts - lastGlDraw >= interval) {
        const liveAmp = 0.28 + 0.72 * (1 - smoothP);
        projector.draw({
          time: ts / 1000,
          p: smoothP,
          liveAmp: moving ? liveAmp * 0.55 : liveAmp,
          optics,
          seed,
        });
        lastGlDraw = ts;
      }
    }

    if (
      !moving &&
      maxPx > 40 &&
      (thrownMaxPx < 40 ||
        Math.abs(maxPx - thrownMaxPx) / Math.max(thrownMaxPx, 1) > 0.06) &&
      performance.now() - lastFull > 280
    ) {
      refreshMetrics();
      scheduleDevelop("max-changed");
    }
  };

  let resizeTimer = 0;
  const onResize = () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      refreshMetrics();
      scheduleDevelop("resize");
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
    await fullDevelop();
    lastFull = performance.now();
    scrollIdleAt = performance.now();
    if (!reduced) {
      raf = requestAnimationFrame(loop);
    } else if (projector) {
      projector.draw({
        time: 0,
        p: smoothP,
        liveAmp: 0.3,
        optics,
        seed,
      });
      applyCssScale(smoothP, 1);
    }
  };
  start();

  return {
    destroy() {
      running = false;
      cancelAnimationFrame(raf);
      clearTimeout(pendingThrow);
      clearTimeout(resizeTimer);
      window.removeEventListener("resize", onResize);
      projector?.destroy();
      projector = null;
    },
    resize() {
      refreshMetrics();
      scheduleDevelop("resize");
    },
    rethrow() {
      thrownMaxPx = 0;
      scheduleDevelop("force");
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
