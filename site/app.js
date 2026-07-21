/**
 * f00.sh — progressive enhancement.
 * Version labels, copy buttons, tabs, mobile nav, GitHub stats.
 * Works offline without JS.
 */
(() => {
  "use strict";

  const FALLBACK_VERSION = "v0.11.0";

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

  const applyGithub = (data) => {
    setAll("[data-github-stars]", formatCount(data.stargazers_count || 0));
    setAll("[data-github-forks]", formatCount(data.forks_count || 0));
    setAll("[data-github-watchers]", formatCount(data.subscribers_count || 0));
    setAll("[data-github-issues]", formatCount(data.open_issues_count || 0));
  };

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
})();
