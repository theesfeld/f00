/**
 * f00.sh front-end — progressive enhancement.
 * Scroll reveals, sticky story beats, interactive demos, install copy.
 * Works offline without JS; this layer is polish only.
 */
(() => {
  "use strict";

  const FALLBACK_VERSION = "v0.11.0";
  const prefersReduced =
    window.matchMedia &&
    window.matchMedia("(prefers-reduced-motion: reduce)").matches;

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
      const base = el.getAttribute("data-version-pin") || "";
      el.textContent = base.replace(/__VERSION__/g, tag);
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
      const text = (target ? target.textContent : btn.getAttribute("data-copy-text") || "").trim();
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

  /* ── scroll reveals (IO fallback; CSS scroll-timeline preferred) ── */
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

  /* ── sticky story: switch terminal as beats enter ────── */
  const stage = document.getElementById("story-stage");
  const beats = document.querySelectorAll("[data-story-beat]");
  const panels = document.querySelectorAll("[data-story-panel]");
  if (stage && beats.length && panels.length && "IntersectionObserver" in window) {
    const activate = (id) => {
      panels.forEach((p) => {
        p.hidden = p.getAttribute("data-story-panel") !== id;
      });
      beats.forEach((b) => {
        b.classList.toggle(
          "is-active",
          b.getAttribute("data-story-beat") === id
        );
      });
    };
    // default first
    activate(beats[0].getAttribute("data-story-beat"));
    if (!prefersReduced) {
      const sio = new IntersectionObserver(
        (entries) => {
          // pick the most visible beat
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
        { rootMargin: "-20% 0px -35% 0px", threshold: [0.25, 0.5, 0.75, 1] }
      );
      beats.forEach((b) => sio.observe(b));
    }
  }

  /* ── generic tabs (demos + install) ──────────────────── */
  function wireTabs(rootSel, tabAttr, paneAttr) {
    const root = document.querySelector(rootSel);
    if (!root) return;
    const tabs = [...root.querySelectorAll("[role=tab]")];
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

  /* ── header shadow on scroll ─────────────────────────── */
  const header = document.querySelector(".site-header");
  if (header) {
    const onScroll = () => {
      header.classList.toggle("is-scrolled", window.scrollY > 8);
    };
    onScroll();
    window.addEventListener("scroll", onScroll, { passive: true });
  }

  /* ── year ────────────────────────────────────────────── */
  document.querySelectorAll("[data-year]").forEach((el) => {
    el.textContent = String(new Date().getFullYear());
  });
})();
