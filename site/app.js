/**
 * f00.sh — progressive enhancement.
 * Document scroll is the product: cinematic stage, continuous
 * section enter/exit, parallax depth, story scrub, orientation,
 * magnetic CTAs, light tilt. Works offline without JS.
 */
(() => {
  "use strict";

  const FALLBACK_VERSION = "v0.11.0";
  const prefersReduced =
    window.matchMedia &&
    window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const finePointer =
    window.matchMedia && window.matchMedia("(pointer: fine)").matches;

  const root = document.documentElement;
  const body = document.body;
  const clamp = (n, a, b) => Math.min(b, Math.max(a, n));
  const lerp = (a, b, t) => a + (b - a) * t;
  const smoothstep = (e0, e1, x) => {
    const t = clamp((x - e0) / (e1 - e0), 0, 1);
    return t * t * (3 - 2 * t);
  };

  /* ── version labels from GitHub latest ───────────────── */
  function setVersionLabels(tag) {
    if (!tag) return;
    document.querySelectorAll("[data-version]").forEach((el) => {
      el.textContent = tag;
    });
    document.querySelectorAll("[data-version-href]").forEach((el) => {
      el.setAttribute(
        "href",
        "https://github.com/theesfeld/f00/releases/tag/" +
          encodeURIComponent(tag)
      );
    });
    document.querySelectorAll("[data-version-pin]").forEach((el) => {
      if (!el.dataset.versionTemplate) {
        el.dataset.versionTemplate =
          el.getAttribute("data-version-pin") || el.textContent || "";
      }
      el.textContent = el.dataset.versionTemplate.replace(/__VERSION__/g, tag);
    });
  }

  setVersionLabels(FALLBACK_VERSION);
  try {
    fetch("https://api.github.com/repos/theesfeld/f00/releases/latest", {
      headers: { Accept: "application/vnd.github+json" },
    })
      .then((r) => (r.ok ? r.json() : null))
      .then((data) => {
        if (data && data.tag_name) setVersionLabels(data.tag_name);
      })
      .catch(() => {});
  } catch (_) {}

  /* ── copy install ────────────────────────────────────── */
  document.querySelectorAll("[data-copy]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const sel = btn.getAttribute("data-copy");
      const target = sel ? document.querySelector(sel) : null;
      const text = (
        target
          ? target.textContent
          : btn.getAttribute("data-copy-text") || ""
      ).trim();
      if (!text) return;
      const done = () => {
        const prev = btn.textContent;
        btn.textContent = "copied";
        btn.classList.add("copied");
        setTimeout(() => {
          btn.textContent = prev;
          btn.classList.remove("copied");
        }, 1400);
      };
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(text).then(done).catch(fallback);
      } else {
        fallback();
      }
      function fallback() {
        const ta = document.createElement("textarea");
        ta.value = text;
        document.body.appendChild(ta);
        ta.select();
        try {
          document.execCommand("copy");
        } catch (_) {}
        document.body.removeChild(ta);
        done();
      }
    });
  });

  /* ── mobile nav ──────────────────────────────────────── */
  const menuBtn = document.querySelector(".menu-button");
  const nav = document.querySelector(".site-nav");
  if (menuBtn && nav) {
    menuBtn.addEventListener("click", () => {
      const open = menuBtn.getAttribute("aria-expanded") !== "true";
      menuBtn.setAttribute("aria-expanded", String(open));
      nav.classList.toggle("open", open);
      document.body.classList.toggle("nav-open", open);
    });
    nav.querySelectorAll("a").forEach((a) => {
      a.addEventListener("click", () => {
        menuBtn.setAttribute("aria-expanded", "false");
        nav.classList.remove("open");
        document.body.classList.remove("nav-open");
      });
    });
  }

  /* ── scroll reveals (IO) ─────────────────────────────── */
  if (!prefersReduced && "IntersectionObserver" in window) {
    const reveal = document.querySelectorAll(".reveal");
    const io = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            e.target.classList.add("is-visible");
            io.unobserve(e.target);
          }
        });
      },
      { rootMargin: "0px 0px -8% 0px", threshold: 0.12 }
    );
    reveal.forEach((el) => io.observe(el));
  } else {
    document.querySelectorAll(".reveal").forEach((el) => {
      el.classList.add("is-visible");
    });
  }

  /* ── sticky story: scrub terminal as beats enter ─────── */
  const stage = document.getElementById("story-stage");
  const beats = document.querySelectorAll("[data-story-beat]");
  const panels = document.querySelectorAll("[data-story-panel]");
  if (stage && beats.length && panels.length && "IntersectionObserver" in window) {
    const activate = (id) => {
      panels.forEach((p) => {
        const on = p.getAttribute("data-story-panel") === id;
        p.hidden = !on;
        p.classList.toggle("is-active", on);
      });
      beats.forEach((b) => {
        b.classList.toggle(
          "is-active",
          b.getAttribute("data-story-beat") === id
        );
      });
    };
    activate(beats[0].getAttribute("data-story-beat"));
    if (!prefersReduced) {
      const sio = new IntersectionObserver(
        (entries) => {
          let best = null;
          let bestRatio = 0;
          entries.forEach((e) => {
            if (e.isIntersecting && e.intersectionRatio >= bestRatio) {
              bestRatio = e.intersectionRatio;
              best = e.target;
            }
          });
          if (best) activate(best.getAttribute("data-story-beat"));
        },
        { rootMargin: "-22% 0px -38% 0px", threshold: [0.2, 0.4, 0.6, 0.8, 1] }
      );
      beats.forEach((b) => sio.observe(b));
    }
  }

  /* ── generic tabs (demos + install + bench) ──────────── */
  function wireTabs(rootSel, tabAttr, paneAttr) {
    const rootEl = document.querySelector(rootSel);
    if (!rootEl) return;
    const tabs = [...rootEl.querySelectorAll("[role=tab]")];
    const panes = [...document.querySelectorAll(`[${paneAttr}]`)];
    const select = (name) => {
      tabs.forEach((t) => {
        const on = t.getAttribute(tabAttr) === name;
        t.setAttribute("aria-selected", String(on));
        t.tabIndex = on ? 0 : -1;
      });
      panes.forEach((p) => {
        p.hidden = p.getAttribute(paneAttr) !== name;
      });
    };
    tabs.forEach((t) => {
      t.addEventListener("click", () => select(t.getAttribute(tabAttr)));
      t.addEventListener("keydown", (e) => {
        const i = tabs.indexOf(t);
        let next = i;
        if (e.key === "ArrowRight") next = (i + 1) % tabs.length;
        if (e.key === "ArrowLeft") next = (i - 1 + tabs.length) % tabs.length;
        if (next !== i) {
          e.preventDefault();
          tabs[next].focus();
          select(tabs[next].getAttribute(tabAttr));
        }
      });
    });
    if (tabs[0]) select(tabs[0].getAttribute(tabAttr));
  }

  wireTabs("[data-demo-tabs]", "data-demo-tab", "data-demo-pane");
  wireTabs("[data-install-tabs]", "data-install-tab", "data-install-pane");
  wireTabs("[data-bench-tabs]", "data-bench-tab", "data-bench-pane");

  /* ── year ────────────────────────────────────────────── */
  document.querySelectorAll("[data-year]").forEach((el) => {
    el.textContent = String(new Date().getFullYear());
  });

  /* ── live GitHub popularity ──────────────────────────── */
  const formatCount = (n) => {
    if (typeof n !== "number" || Number.isNaN(n)) return "—";
    if (n >= 1000) return `${(n / 1000).toFixed(n >= 10000 ? 0 : 1)}k`;
    return String(n);
  };

  const setAll = (selector, value) => {
    document.querySelectorAll(selector).forEach((node) => {
      node.textContent = value;
    });
  };

  const fillBars = (stats) => {
    const max = Math.max(
      stats.stars,
      stats.forks,
      stats.watchers,
      stats.issues,
      1
    );
    const map = {
      stars: stats.stars,
      forks: stats.forks,
      watchers: stats.watchers,
      issues: stats.issues,
    };
    Object.entries(map).forEach(([key, value]) => {
      document.querySelectorAll(`[data-stat-bar="${key}"]`).forEach((bar) => {
        bar.style.setProperty("--fill", `${Math.round((value / max) * 100)}%`);
      });
    });
  };

  const applyGithub = (data) => {
    const stats = {
      stars: data.stargazers_count || 0,
      forks: data.forks_count || 0,
      watchers: data.subscribers_count || 0,
      issues: data.open_issues_count || 0,
    };
    setAll("[data-github-stars]", formatCount(stats.stars));
    setAll("[data-github-forks]", formatCount(stats.forks));
    setAll("[data-github-watchers]", formatCount(stats.watchers));
    setAll("[data-github-issues]", formatCount(stats.issues));
    fillBars(stats);
  };

  applyGithub({
    stargazers_count: 0,
    forks_count: 0,
    subscribers_count: 0,
    open_issues_count: 0,
  });

  try {
    fetch("https://api.github.com/repos/theesfeld/f00", {
      headers: { Accept: "application/vnd.github+json" },
    })
      .then((res) => (res.ok ? res.json() : null))
      .then((data) => {
        if (data) applyGithub(data);
      })
      .catch(() => {});
  } catch (_) {}

  /* ══════════════════════════════════════════════════════
   * Cinematic scroll engine
   * ══════════════════════════════════════════════════════ */
  const header = document.querySelector(".site-header");
  const hero = document.querySelector(".hero");
  const sections = [...document.querySelectorAll("[data-section]")];
  const cinemaSections = [...document.querySelectorAll('[data-motion="cinema"]')];
  const navLinks = [...document.querySelectorAll("[data-nav]")];
  const railLinks = [...document.querySelectorAll("[data-rail]")];
  const parallaxNodes = [...document.querySelectorAll("[data-parallax]")];

  const ATMOSPHERES = {
    top: { a: "61, 255, 154", b: "125, 255, 224", c: "240, 193, 74" },
    story: { a: "61, 255, 154", b: "90, 200, 255", c: "125, 255, 224" },
    features: { a: "125, 255, 224", b: "61, 255, 154", c: "180, 255, 200" },
    demos: { a: "100, 180, 255", b: "61, 255, 154", c: "125, 220, 255" },
    bench: { a: "240, 193, 74", b: "61, 255, 154", c: "255, 160, 90" },
    install: { a: "61, 255, 154", b: "255, 138, 101", c: "125, 255, 224" },
    github: { a: "240, 193, 74", b: "255, 224, 138", c: "61, 255, 154" },
    docs: { a: "143, 154, 171", b: "61, 255, 154", c: "125, 255, 224" },
  };

  let ticking = false;
  let lastActiveId = "";
  let lastScrolled = null;
  let lastAtmKey = "";

  function scrollMetrics() {
    const y = window.scrollY || window.pageYOffset || 0;
    const vh = window.innerHeight || 1;
    const max = Math.max(
      1,
      document.documentElement.scrollHeight - vh
    );
    const p = clamp(y / max, 0, 1);

    let heroP = 0;
    if (hero) {
      const h = hero.offsetHeight || 1;
      heroP = clamp(y / (h * 0.85), 0, 1);
    }

    return { y, p, heroP, vh, max };
  }

  function activeSectionId(y) {
    const probe = y + window.innerHeight * 0.28;
    let current = sections[0] ? sections[0].getAttribute("data-section") : "top";
    for (const sec of sections) {
      const top = sec.offsetTop;
      if (top <= probe) current = sec.getAttribute("data-section");
    }
    return current;
  }

  function setActiveOrientation(id) {
    if (id === lastActiveId) return;
    lastActiveId = id;
    if (body) body.dataset.activeSection = id;
    navLinks.forEach((a) => {
      a.classList.toggle("is-active", a.getAttribute("data-nav") === id);
    });
    railLinks.forEach((a) => {
      a.classList.toggle("is-active", a.getAttribute("data-rail") === id);
    });
  }

  function setAtmosphere(id) {
    if (id === lastAtmKey) return;
    lastAtmKey = id;
    const atm = ATMOSPHERES[id] || ATMOSPHERES.top;
    root.style.setProperty("--atm-a", atm.a);
    root.style.setProperty("--atm-b", atm.b);
    root.style.setProperty("--atm-c", atm.c);
  }

  /**
   * Continuous enter → hold → exit for a cinema section.
   * enterOnly: no exit fade (finale blocks that can't scroll past hold zone).
   */
  function cinemaForRect(rect, vh, enterOnly) {
    // Enter: full cinema needs top near upper third; enter-only finishes earlier
    // so short end sections can resolve without more document height.
    const enterEnd = enterOnly ? vh * 0.7 : vh * 0.22;
    const enter = smoothstep(vh * 0.98, enterEnd, rect.top);
    // Exit: bottom climbs through upper half — skipped for finale sections
    const exit = enterOnly
      ? 1
      : smoothstep(vh * 0.08, vh * 0.58, rect.bottom);
    const vis = clamp(enter * exit, 0, 1);

    // Slide: full cinema through; enter-only only rises in (never lifts out)
    const anchor = rect.top + Math.min(rect.height * 0.28, vh * 0.22);
    let slide = clamp((anchor - vh * 0.42) / vh, -1.15, 1.15) * 64;
    if (enterOnly) {
      slide = Math.max(0, slide) * (1 - vis);
    }

    const scale = 0.94 + vis * 0.06;
    const blur = (1 - vis) * 9;

    return { vis, slide, scale, blur };
  }

  function applyCinemaSections(vh, y, maxScroll) {
    if (prefersReduced) {
      cinemaSections.forEach((sec) => {
        sec.style.setProperty("--sec-vis", "1");
        sec.style.setProperty("--sec-slide", "0");
        sec.style.setProperty("--sec-scale", "1");
        sec.style.setProperty("--sec-blur", "0");
      });
      return;
    }

    const n = cinemaSections.length;
    const nearEnd = maxScroll <= 1 || y >= maxScroll - 4;

    cinemaSections.forEach((sec, i) => {
      const rect = sec.getBoundingClientRect();
      // Last two sections: enter-only so page end never soft-exits
      const enterOnly = i >= n - 2;

      // Skip work when fully off-screen
      if (rect.bottom < -80 || rect.top > vh + 80) {
        sec.style.setProperty("--sec-vis", "0");
        sec.style.setProperty("--sec-slide", rect.top > 0 ? "48" : "-40");
        sec.style.setProperty("--sec-scale", "0.94");
        sec.style.setProperty("--sec-blur", "8");
        return;
      }

      let m = cinemaForRect(rect, vh, enterOnly);

      // At max scroll, pin finale sections fully sharp if any of them is on-screen
      if (enterOnly && nearEnd && rect.bottom > 0 && rect.top < vh) {
        m = { vis: 1, slide: 0, scale: 1, blur: 0 };
      }

      sec.style.setProperty("--sec-vis", m.vis.toFixed(4));
      sec.style.setProperty("--sec-slide", m.slide.toFixed(2));
      sec.style.setProperty("--sec-scale", m.scale.toFixed(4));
      sec.style.setProperty("--sec-blur", m.blur.toFixed(2));
    });
  }

  function applyParallax(y) {
    if (prefersReduced) return;
    parallaxNodes.forEach((el) => {
      const fy = parseFloat(el.getAttribute("data-parallax") || "0");
      const fx = parseFloat(el.getAttribute("data-parallax-x") || "0");
      const fr = parseFloat(el.getAttribute("data-parallax-r") || "0");
      const baseRot = parseFloat(el.getAttribute("data-base-rot") || "0");
      const skew = el.getAttribute("data-base-skew") || "";
      const tx = y * fx;
      const ty = y * fy;
      const rot = baseRot + y * fr;
      const parts = [
        `translate3d(${tx.toFixed(1)}px, ${ty.toFixed(1)}px, 0)`,
        `rotate(${rot.toFixed(3)}deg)`,
      ];
      if (skew) parts.push(skew);
      el.style.transform = parts.join(" ");
    });
  }

  function applyScroll() {
    ticking = false;
    const { y, p, heroP, vh, max } = scrollMetrics();

    root.style.setProperty("--scroll-p", p.toFixed(4));
    root.style.setProperty("--hero-p", heroP.toFixed(4));
    root.style.setProperty(
      "--atm-intensity",
      (0.75 + p * 0.35).toFixed(3)
    );
    if (hero) hero.style.setProperty("--hero-p", heroP.toFixed(4));

    if (header) {
      const scrolled = y > 8;
      if (scrolled !== lastScrolled) {
        lastScrolled = scrolled;
        header.classList.toggle("is-scrolled", scrolled);
      }
    }

    applyParallax(y);
    applyCinemaSections(vh, y, max);

    const active = activeSectionId(y);
    setActiveOrientation(active);
    setAtmosphere(active);
  }

  function onScroll() {
    if (ticking) return;
    ticking = true;
    requestAnimationFrame(applyScroll);
  }

  applyScroll();
  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onScroll, { passive: true });

  /* ── magnetic CTAs ───────────────────────────────────── */
  if (!prefersReduced && finePointer) {
    document.querySelectorAll("[data-magnetic]").forEach((el) => {
      const strength = 14;
      let raf = 0;
      let tx = 0;
      let ty = 0;
      let cx = 0;
      let cy = 0;

      const tick = () => {
        cx = lerp(cx, tx, 0.18);
        cy = lerp(cy, ty, 0.18);
        el.style.transform = `translate3d(${cx.toFixed(2)}px, ${cy.toFixed(2)}px, 0)`;
        if (Math.abs(cx - tx) > 0.05 || Math.abs(cy - ty) > 0.05) {
          raf = requestAnimationFrame(tick);
        } else {
          raf = 0;
        }
      };

      el.addEventListener("pointermove", (e) => {
        const r = el.getBoundingClientRect();
        const dx = e.clientX - (r.left + r.width / 2);
        const dy = e.clientY - (r.top + r.height / 2);
        tx = clamp(dx / (r.width / 2), -1, 1) * strength;
        ty = clamp(dy / (r.height / 2), -1, 1) * strength;
        if (!raf) raf = requestAnimationFrame(tick);
      });

      el.addEventListener("pointerleave", () => {
        tx = 0;
        ty = 0;
        if (!raf) raf = requestAnimationFrame(tick);
      });
    });
  }

  /* ── light tilt / hot focus ──────────────────────────── */
  if (!prefersReduced && finePointer) {
    document.querySelectorAll("[data-tilt]").forEach((el) => {
      const max = el.classList.contains("hero-term") ? 5 : 7;
      let raf = 0;
      let rx = 0;
      let ry = 0;
      let tx = 0;
      let ty = 0;

      const tick = () => {
        rx = lerp(rx, tx, 0.16);
        ry = lerp(ry, ty, 0.16);
        if (el.classList.contains("hero-term")) {
          el.style.transform = `perspective(900px) rotateY(${ry.toFixed(2)}deg) rotateX(${rx.toFixed(2)}deg)`;
        } else {
          el.style.transform = `perspective(800px) rotateX(${rx.toFixed(2)}deg) rotateY(${ry.toFixed(2)}deg) translateZ(0)`;
        }
        if (Math.abs(rx - tx) > 0.05 || Math.abs(ry - ty) > 0.05) {
          raf = requestAnimationFrame(tick);
        } else {
          raf = 0;
        }
      };

      el.addEventListener("pointermove", (e) => {
        const r = el.getBoundingClientRect();
        const px = (e.clientX - r.left) / r.width - 0.5;
        const py = (e.clientY - r.top) / r.height - 0.5;
        ty = px * max * 2;
        tx = -py * max * 2;
        if (!raf) raf = requestAnimationFrame(tick);
      });

      el.addEventListener("pointerleave", () => {
        tx = 0;
        ty = el.classList.contains("hero-term") ? -4 : 0;
        const settle = () => {
          rx = lerp(rx, tx, 0.16);
          ry = lerp(ry, ty, 0.16);
          if (Math.abs(rx - tx) > 0.08 || Math.abs(ry - ty) > 0.08) {
            if (el.classList.contains("hero-term")) {
              el.style.transform = `perspective(900px) rotateY(${ry.toFixed(2)}deg) rotateX(${rx.toFixed(2)}deg)`;
            } else {
              el.style.transform = `perspective(800px) rotateX(${rx.toFixed(2)}deg) rotateY(${ry.toFixed(2)}deg)`;
            }
            raf = requestAnimationFrame(settle);
          } else {
            el.style.transform = "";
            raf = 0;
          }
        };
        if (!raf) raf = requestAnimationFrame(settle);
      });
    });
  }

  /* ── bench shell also counts as reveal for bar fill ──── */
  const benchShell = document.querySelector(".bench-shell");
  if (benchShell && "IntersectionObserver" in window) {
    const bio = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            e.target.classList.add("is-visible");
            bio.unobserve(e.target);
          }
        });
      },
      { threshold: 0.2 }
    );
    bio.observe(benchShell);
    if (prefersReduced) benchShell.classList.add("is-visible");
  }
})();
