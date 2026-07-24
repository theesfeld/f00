/**
 * f00tils (f00.sh) — progressive enhancement.
 * Version labels, copy buttons, combined scoreboard (features + benches).
 */
(() => {
  "use strict";

  const FALLBACK_VERSION = "v0.15.1";
  const HIGHLIGHT_N = 8;

  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function setVersionLabels(tag) {
    if (!tag) return;
    document.querySelectorAll("[data-version]").forEach((el) => {
      el.textContent = tag;
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

  document.querySelectorAll("[data-copy]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const sel = btn.getAttribute("data-copy");
      const target = sel ? document.querySelector(sel) : null;
      const text = (target ? target.textContent : "").trim();
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
        navigator.clipboard.writeText(text).then(done).catch(() => {
          const ta = document.createElement("textarea");
          ta.value = text;
          document.body.appendChild(ta);
          ta.select();
          try {
            document.execCommand("copy");
          } catch (_) {}
          document.body.removeChild(ta);
          done();
        });
      }
    });
  });

  function fmtMs(v) {
    if (v == null || Number.isNaN(v)) return "—";
    return Number(v).toFixed(2);
  }

  function fmtRatio(v) {
    if (v == null || Number.isNaN(v)) return "—";
    return `<strong>${esc(Number(v).toFixed(2))}×</strong>`;
  }

  function benchIndex(tools) {
    const map = Object.create(null);
    (tools || []).forEach((t) => {
      if (t && t.tool) map[t.tool] = t;
    });
    return map;
  }

  function renderHighlights(tools, meta) {
    const body = document.getElementById("highlight-body");
    const metaEl = document.getElementById("bench-meta");
    const stats = document.getElementById("bench-stats");
    if (!body) return;

    const ok = (tools || []).filter((t) => t.status === "ok" && t.ratio != null);
    ok.sort((a, b) => b.ratio - a.ratio);

    if (metaEl) {
      metaEl.textContent = meta
        ? `${meta.method || "median"} · ${meta.machine || ""} · ${meta.system || ""} · ${meta.generated_at || ""}`.replace(
            /\s+·\s+$/,
            ""
          )
        : "";
    }

    if (stats && ok.length) {
      stats.hidden = false;
      const ratios = ok.map((t) => t.ratio).sort((a, b) => a - b);
      const mid = ratios[Math.floor(ratios.length / 2)];
      const best = ok[0];
      const set = (id, v) => {
        const el = document.getElementById(id);
        if (el) el.textContent = v;
      };
      set("stat-tools", String(ok.length));
      set("stat-median", `${mid.toFixed(2)}×`);
      set("stat-best", `${best.tool} ${best.ratio.toFixed(1)}×`);
      set("stat-n", meta && meta.n_runs ? String(meta.n_runs) : "—");
    }

    const top = ok.slice(0, HIGHLIGHT_N);
    if (!top.length) {
      body.innerHTML =
        '<tr><td colspan="5" class="muted">No timed tools in suite.json.</td></tr>';
      return;
    }
    body.innerHTML = top
      .map(
        (t) =>
          `<tr>` +
          `<td><code>${esc(t.tool)}</code></td>` +
          `<td><code class="cmd">${esc(t.command_f00 || "")}</code></td>` +
          `<td>${esc(fmtMs(t.time_gnu_ms))}</td>` +
          `<td><strong>${esc(fmtMs(t.time_f00_ms))}</strong></td>` +
          `<td>${fmtRatio(t.ratio)}</td>` +
          `</tr>`
      )
      .join("");
  }

  function renderScoreboard(progress, bench) {
    const body = document.getElementById("scoreboard-body");
    if (!body) return;
    const rows = (progress && progress.rows) || [];
    const bi = benchIndex(bench && bench.tools);

    if (!rows.length) {
      body.innerHTML =
        '<tr><td colspan="9" class="muted">Missing coreutils-progress.json</td></tr>';
      return;
    }

    body.innerHTML = rows
      .map((r) => {
        const b = bi[r.util] || null;
        const depth =
          r.depth === "full" ? "<strong>full</strong>" : esc(r.depth);
        const g = b && b.status === "ok" ? fmtMs(b.time_gnu_ms) : "—";
        const f =
          b && b.status === "ok"
            ? `<strong>${esc(fmtMs(b.time_f00_ms))}</strong>`
            : "—";
        const x =
          b && b.status === "ok" && b.ratio != null ? fmtRatio(b.ratio) : "—";
        // speed column from progress when no bench sample
        const speedNote =
          b && b.status === "ok"
            ? ""
            : r.speed && r.speed !== "—"
              ? ` title="gate: ${esc(r.speed)}"`
              : "";
        return (
          `<tr class="shipped"${speedNote}>` +
          `<td>${esc(r.n)}</td>` +
          `<td><code>${esc(r.util)}</code></td>` +
          `<td><code>${esc(r.f00)}</code></td>` +
          `<td>${esc(r.shipped)}</td>` +
          `<td>${depth}</td>` +
          `<td>${esc(r.modern)}</td>` +
          `<td>${esc(g)}</td>` +
          `<td>${f}</td>` +
          `<td>${x}</td>` +
          `</tr>`
        );
      })
      .join("");
  }

  Promise.all([
    fetch("coreutils-progress.json", { headers: { Accept: "application/json" } })
      .then((r) => (r.ok ? r.json() : null))
      .catch(() => null),
    fetch("bench/suite.json", { headers: { Accept: "application/json" } })
      .then((r) => (r.ok ? r.json() : null))
      .catch(() => null),
  ]).then(([progress, suite]) => {
    if (suite) renderHighlights(suite.tools, suite.meta);
    renderScoreboard(progress, suite);
  });
})();
