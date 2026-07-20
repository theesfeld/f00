/**
 * f00.sh — progressive enhancement.
 * Document scroll is the product: progress, parallax, story scrub,
 * orientation rail/nav, magnetic CTAs, light tilt.
 * Works offline without JS; this layer is polish only.
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
  const clamp = (n, a, b) => Math.min(b, Math.max(a, n));
  const lerp = (a, b, t) => a + (b - a) * t;

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

  /* ══════════════════════════════════════════════════════
   * Scroll-linked motion engine
   * ══════════════════════════════════════════════════════ */
  const header = document.querySelector(".site-header");
  const hero = document.querySelector(".hero");
  const sections = [...document.querySelectorAll("[data-section]")];
  const navLinks = [...document.querySelectorAll("[data-nav]")];
  const railLinks = [...document.querySelectorAll("[data-rail]")];
  const parallaxNodes = [...document.querySelectorAll("[data-parallax]")];

  let ticking = false;
  let lastY = window.scrollY;

  function scrollMetrics() {
    const y = window.scrollY || window.pageYOffset || 0;
    const max = Math.max(
      1,
      document.documentElement.scrollHeight - window.innerHeight
    );
    const p = clamp(y / max, 0, 1);

    let heroP = 0;
    if (hero) {
      const h = hero.offsetHeight || 1;
      heroP = clamp(y / (h * 0.85), 0, 1);
    }

    return { y, p, heroP };
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
    navLinks.forEach((a) => {
      a.classList.toggle("is-active", a.getAttribute("data-nav") === id);
    });
    railLinks.forEach((a) => {
      a.classList.toggle("is-active", a.getAttribute("data-rail") === id);
    });
  }

  function applyScroll() {
    ticking = false;
    const { y, p, heroP } = scrollMetrics();
    lastY = y;

    root.style.setProperty("--scroll-p", p.toFixed(4));
    root.style.setProperty("--hero-p", heroP.toFixed(4));
    if (hero) hero.style.setProperty("--hero-p", heroP.toFixed(4));

    if (header) {
      header.classList.toggle("is-scrolled", y > 8);
    }

    // Ambient / element parallax
    if (!prefersReduced) {
      parallaxNodes.forEach((el) => {
        const factor = parseFloat(el.getAttribute("data-parallax") || "0.1");
        const shift = y * factor;
        el.style.transform = `translate3d(0, ${shift.toFixed(1)}px, 0)`;
      });
    }

    setActiveOrientation(activeSectionId(y));
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

      const base = () => {
        // Preserve hero perspective baseline when idle
        if (el.classList.contains("hero-term")) {
          return `perspective(900px) rotateY(${(-4 + parseFloat(getComputedStyle(hero || root).getPropertyValue("--hero-p") || 0) * 4).toFixed(2)}deg) rotateX(2deg)`;
        }
        return "perspective(800px) rotateX(0deg) rotateY(0deg)";
      };

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
        // ease back; clear inline after settle so CSS --hero-p can drive
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
