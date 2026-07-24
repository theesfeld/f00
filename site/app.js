/**
 * f00tils (f00.sh) — progressive enhancement.
 * Version labels, copy buttons, benchmark table, scoreboard matrix.
 */
(() => {
  "use strict";

  const FALLBACK_VERSION = "v0.15.1";

  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

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

  /* ── suite benchmarks ────────────────────────────────── */
  function renderBench(data) {
    const body = document.getElementById("bench-body");
    const meta = document.getElementById("bench-meta");
    const nEl = document.getElementById("bench-n");
    if (!body) return;
    const tools = (data && data.tools) || [];
    const m = (data && data.meta) || {};
    if (nEl && m.n_runs) nEl.textContent = String(m.n_runs);
    if (meta) {
      meta.textContent =
        (m.generated_at ? `Generated ${m.generated_at}` : "Suite benchmarks") +
        (m.machine ? ` · ${m.machine}` : "") +
        (m.system ? ` · ${m.system}` : "") +
        (m.method ? ` · ${m.method}` : "");
    }
    const rows = tools.filter((t) => t.status === "ok");
    if (!rows.length) {
      body.innerHTML =
        '<tr><td colspan="6" class="muted">No benchmark rows.</td></tr>';
      return;
    }
    body.innerHTML = rows
      .map((t) => {
        const ratio =
          t.ratio != null ? `<strong>${esc(t.ratio.toFixed(2))}×</strong>` : "—";
        const g =
          t.time_gnu_ms != null ? t.time_gnu_ms.toFixed(3) : "—";
        const f =
          t.time_f00_ms != null
            ? `<strong>${esc(t.time_f00_ms.toFixed(3))}</strong>`
            : "—";
        const out = t.output_f00 ? esc(t.output_f00) : "—";
        return (
          `<tr>` +
          `<td><code>${esc(t.tool)}</code></td>` +
          `<td><code>${esc(t.command_f00)}</code></td>` +
          `<td class="bench-out"><code>${out}</code></td>` +
          `<td>${esc(g)}</td>` +
          `<td>${f}</td>` +
          `<td>${ratio}</td>` +
          `</tr>`
        );
      })
      .join("");
  }

  try {
    fetch("bench/suite.json", { headers: { Accept: "application/json" } })
      .then((r) => (r.ok ? r.json() : null))
      .then((data) => {
        if (data) renderBench(data);
        else {
          const body = document.getElementById("bench-body");
          if (body) {
            body.innerHTML =
              '<tr><td colspan="6" class="muted">Could not load <code>bench/suite.json</code>.</td></tr>';
          }
        }
      })
      .catch(() => {
        const body = document.getElementById("bench-body");
        if (body) {
          body.innerHTML =
            '<tr><td colspan="6" class="muted">Could not load <code>bench/suite.json</code>.</td></tr>';
        }
      });
  } catch (_) {}

  /* ── scoreboard matrix ───────────────────────────────── */
  function renderMatrix(data) {
    const body = document.getElementById("matrix-body");
    if (!body || !data || !data.rows) return;
    body.innerHTML = data.rows
      .map((r) => {
        const depth =
          r.depth === "full" ? `<strong>full</strong>` : esc(r.depth);
        return (
          `<tr class="shipped">` +
          `<td>${esc(r.n)}</td>` +
          `<td><code>${esc(r.util)}</code></td>` +
          `<td><code>${esc(r.f00)}</code></td>` +
          `<td>${esc(r.shipped)}</td>` +
          `<td>${depth}</td>` +
          `<td>${esc(r.modern)}</td>` +
          `<td>${esc(r.speed)}</td>` +
          `</tr>`
        );
      })
      .join("");
  }

  try {
    fetch("coreutils-progress.json", {
      headers: { Accept: "application/json" },
    })
      .then((r) => (r.ok ? r.json() : null))
      .then((data) => {
        if (data) renderMatrix(data);
      })
      .catch(() => {});
  } catch (_) {}

  /* ── year ────────────────────────────────────────────── */
  document.querySelectorAll("[data-year]").forEach((el) => {
    el.textContent = String(new Date().getFullYear());
  });
})();
