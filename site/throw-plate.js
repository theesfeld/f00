/* f00 hub splash — DEVELOP once, PROJECT living (throttled)
 *
 * Scroll shrink = compositor transform only.
 * iOS: 1:1 scroll map, no GL while moving, tiny backing store.
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
  const wrap =
    opts.wrapEl ||
    canvas.closest?.(".splash-wrap") ||
    document.querySelector(".splash-wrap");
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
  let lastAppliedP = -1;
  let lastAppliedScale = -1;
  let loopLive = false;

  let maxPx = 120;
  let restPx = 56;
  let shrinkRange = 400;
  let headerH = 54;

  let smoothP = 0;
  let targetP = 0;
  let dispScale = 1;
  let lastTs = 0;
  let inkTopFrac = 0.12;
  let inkHeightFrac = 0.76;
  let inkCenterFrac = 0.5;
  let baseCssW = 0;
  let baseCssH = 0;
  let restScale = 0.12;
  let travelY = 0;
  let opticalNudge = 0;

  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';
  const root = document.documentElement;

  const setScrollingFlag = (on) => {
    const cur = root.dataset.f00Scrolling === "1";
    if (on && !cur) root.dataset.f00Scrolling = "1";
    else if (!on && cur) root.dataset.f00Scrolling = "0";
  };

  const refreshMetrics = () => {
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

    const viewH = window.innerHeight || 800;
    opticalNudge = Math.min(28, Math.max(10, viewH * 0.022));
    const bandMid = viewH * 0.5 + opticalNudge;
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

  const frameEl =
    document.getElementById("splash-frame") ||
    document.querySelector(".splash-frame");

  /** Keep frame dock class in lockstep with p (don't wait on app.js rAF). */
  const syncDockClass = (wantDock) => {
    const on = !!wantDock;
    if (root.classList.contains("logo-docked") === on) return on;
    root.classList.toggle("logo-docked", on);
    if (frameEl) frameEl.classList.toggle("is-header-dock", on);
    /* reflow so wrap collapses before we plant the plate */
    if (wrap) void wrap.offsetHeight;
    return on;
  };

  /** Center on ink mass, not film pad. */
  const applyCssScale = (p, force) => {
    const pp = Math.max(0, Math.min(1, p));
    const targetScale = Math.max(
      0.04,
      Math.min(1.05, 1 - pp * (1 - restScale))
    );
    dispScale = targetScale;

    /* dock class + frame pin first, then plant */
    /* snap into header near the end of shrink — not only at full scroll */
    const hardDock = syncDockClass(pp > 0.72);

    /*
     * Ink center offset inside the plate (css px at scale 1), then scaled.
     * Positive inkCenterFrac > 0.5 → ink sits low in texture → shift up.
     */
    const inkBiasY = -(inkCenterFrac - 0.5) * baseCssH * dispScale;

    /*
     * hardDock: frame is the header bar — only ink bias.
     * free: rise from optical center toward header.
     */
    const y = hardDock
      ? inkBiasY
      : opticalNudge + travelY * pp + inkBiasY;

    if (
      !force &&
      Math.abs(pp - lastAppliedP) < 0.0004 &&
      Math.abs(dispScale - lastAppliedScale) < 0.0004
    ) {
      return;
    }
    lastAppliedP = pp;
    lastAppliedScale = dispScale;

    if (baseCssW > 0) {
      const dw = Math.max(36, baseCssW * restScale);
      const dh = Math.max(24, baseCssH * restScale);
      root.style.setProperty("--splash-dock-w", `${dw.toFixed(1)}px`);
      root.style.setProperty("--splash-dock-h", `${dh.toFixed(1)}px`);
    }

    canvas.style.transform = [
      "translate3d(-50%, -50%, 0)",
      `translate3d(0, ${y.toFixed(2)}px, 0)`,
      `scale3d(${dispScale.toFixed(5)}, ${dispScale.toFixed(5)}, 1)`,
    ].join(" ");
  };

  const ensureGl = () => {
    if (projector) return projector;
    projector = createGlProjector(canvas, {
      quality: mobile ? "fast" : "high",
    });
    return projector;
  };

  const fullDevelop = async () => {
    if (busy) return;
    busy = true;
    try {
      refreshMetrics();
      if (!(maxPx > 24)) return;

      const dpr = Math.min(window.devicePixelRatio || 1, mobile ? 1 : 1.5);
      const layoutW = maxPx * 1.35;
      const layoutH = maxPx * 0.88;
      const padX = Math.max(mobile ? 22 : 36, layoutW * (mobile ? 0.1 : 0.14));
      const padY = Math.max(mobile ? 24 : 40, layoutH * (mobile ? 0.12 : 0.16));
      const cssW = Math.ceil(layoutW + padX * 2);
      const cssH = Math.ceil(layoutH + padY * 2);

      const maxEdge = mobile ? 420 : 960;
      let w = Math.floor(cssW * dpr);
      let h = Math.floor(cssH * dpr);
      const sc = Math.min(1, maxEdge / Math.max(w, h, 1));
      w = Math.max(140, Math.floor(w * sc));
      h = Math.max(100, Math.floor(h * sc));

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
        console.warn("[throw-plate] WebGL unavailable");
        return;
      }
      glp.uploadDensity(density, w, h);

      let inkTop = -1;
      let inkBot = -1;
      for (let y = 0; y < h; y++) {
        let rowHit = false;
        for (let x = 0; x < w; x++) {
          if (density[y * w + x] > 0.08) {
            rowHit = true;
            break;
          }
        }
        if (rowHit) {
          if (inkTop < 0) inkTop = y;
          inkBot = y;
        }
      }
      if (inkTop < 0) {
        inkTopFrac = 0.12;
        inkHeightFrac = 0.76;
        inkCenterFrac = 0.5;
      } else {
        inkTopFrac = inkTop / h;
        inkHeightFrac = Math.max(0.35, (inkBot - inkTop + 1) / h);
        inkCenterFrac = inkTopFrac + inkHeightFrac * 0.5;
      }

      baseCssW = cssW;
      baseCssH = cssH;

      /* fit ink (not pad) inside header bar with a little air */
      const headerBudget = Math.max(26, headerH * 0.58);
      const targetInkH = Math.max(26, Math.min(headerBudget, restPx * 0.75));
      const fullInkH = inkHeightFrac * cssH;
      restScale =
        fullInkH > 1
          ? Math.max(0.04, Math.min(0.16, targetInkH / fullInkH))
          : Math.min(0.14, restPx / Math.max(maxPx, 1));

      thrownMaxPx = maxPx;
      canvas.style.width = `${cssW}px`;
      canvas.style.height = `${cssH}px`;
      canvas.style.left = "50%";
      canvas.style.top = "50%";
      canvas.style.transformOrigin = "50% 50%";
      canvas.style.willChange = "transform";
      canvas.style.webkitBackfaceVisibility = "hidden";
      canvas.style.backfaceVisibility = "hidden";
      canvas.style.position = "absolute";

      root.style.setProperty(
        "--splash-dock-w",
        `${Math.max(36, cssW * restScale).toFixed(1)}px`
      );
      root.style.setProperty(
        "--splash-dock-h",
        `${Math.max(24, cssH * restScale).toFixed(1)}px`
      );

      lastAppliedP = -1;
      applyCssScale(smoothP, true);

      if (!reduced) {
        glp.draw({
          time: performance.now() / 1000,
          p: smoothP,
          liveAmp: mobile ? 0.35 : 0.28 + 0.72 * (1 - smoothP),
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
    pendingThrow = setTimeout(
      () => {
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
      },
      why === "boot" ? 0 : 80
    );
  };

  const loop = (ts) => {
    if (!running || !loopLive) return;
    raf = requestAnimationFrame(loop);

    const dt = lastTs ? Math.min(0.05, (ts - lastTs) / 1000) : 0.016;
    lastTs = ts;

    targetP = readTargetP();
    const prevP = smoothP;

    /* iOS: direct map — lag felt like jank even when GPU was fine */
    if (mobile || reduced) {
      smoothP = targetP;
    } else {
      const k = 1 - Math.exp(-dt * 26);
      smoothP += (targetP - smoothP) * k;
      if (Math.abs(targetP - smoothP) < 0.0006) smoothP = targetP;
    }

    applyCssScale(smoothP, false);

    const scrollY = window.scrollY || 0;
    const scrollDelta = Math.abs(scrollY - lastScrollY);
    lastScrollY = scrollY;

    const moving =
      Math.abs(smoothP - prevP) > 0.00015 ||
      Math.abs(targetP - smoothP) > 0.0008 ||
      scrollDelta > 0.25;

    if (moving) {
      scrollIdleAt = ts;
      setScrollingFlag(true);
    } else if (ts - scrollIdleAt > 120) {
      setScrollingFlag(false);
    }

    const docked = smoothP > 0.72;

    /* GPU: mobile = still plate while scrolling; rare ticks when idle */
    if (projector && !reduced) {
      let interval = mobile ? 90 : 36;
      if (moving) interval = mobile ? 1e12 : 55;
      else if (docked) interval = mobile ? 160 : 90;

      if (ts - lastGlDraw >= interval) {
        projector.draw({
          time: ts / 1000,
          p: smoothP,
          liveAmp: mobile
            ? 0.22 + 0.35 * (1 - smoothP)
            : 0.28 + 0.72 * (1 - smoothP),
          optics,
          seed,
        });
        lastGlDraw = ts;
      }
    }

    /*
     * Mobile hard-docked + idle: stop the rAF pump entirely.
     * Only after logo-docked so we never freeze mid-travel off-screen.
     */
    if (
      mobile &&
      root.classList.contains("logo-docked") &&
      !moving &&
      ts - scrollIdleAt > 280
    ) {
      loopLive = false;
      setScrollingFlag(false);
      return;
    }

    if (
      !moving &&
      maxPx > 40 &&
      (thrownMaxPx < 40 ||
        Math.abs(maxPx - thrownMaxPx) / Math.max(thrownMaxPx, 1) > 0.06) &&
      performance.now() - lastFull > 400
    ) {
      refreshMetrics();
      scheduleDevelop("max-changed");
    }
  };

  const ensureLoop = () => {
    if (!running || reduced) return;
    if (loopLive) return;
    loopLive = true;
    lastTs = 0;
    raf = requestAnimationFrame(loop);
  };

  const onScroll = () => {
    /* kick loop; apply one scale immediately for first paint of the gesture */
    targetP = readTargetP();
    if (mobile) {
      smoothP = targetP;
      applyCssScale(smoothP, false);
    }
    setScrollingFlag(true);
    scrollIdleAt = performance.now();
    ensureLoop();
  };

  let resizeTimer = 0;
  const onResize = () => {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
      refreshMetrics();
      scheduleDevelop("resize");
      ensureLoop();
    }, 160);
  };

  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onResize, { passive: true });
  /* iOS momentum scroll often fires via touchmove more than scroll mid-gesture */
  window.addEventListener("touchmove", onScroll, { passive: true });

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
      ensureLoop();
    } else if (projector) {
      projector.draw({
        time: 0,
        p: smoothP,
        liveAmp: 0.3,
        optics,
        seed,
      });
      applyCssScale(smoothP, true);
    }
  };
  start();

  return {
    destroy() {
      running = false;
      loopLive = false;
      cancelAnimationFrame(raf);
      clearTimeout(pendingThrow);
      clearTimeout(resizeTimer);
      window.removeEventListener("scroll", onScroll);
      window.removeEventListener("resize", onResize);
      window.removeEventListener("touchmove", onScroll);
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
    wrapEl: wrap,
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
