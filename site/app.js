/**
 * f00tils (f00.sh) — progressive enhancement.
 * Version labels, copy buttons, single scoreboard (features + benches).
 */
(() => {
  "use strict";

  const FALLBACK_VERSION = "v0.15.7";

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
    // alias for test/[
    if (map.test && !map["["]) map["["] = map.test;
    return map;
  }

  function renderStats(tools, meta) {
    const metaEl = document.getElementById("bench-meta");
    const stats = document.getElementById("bench-stats");
    const ok = (tools || []).filter((t) => t.status === "ok" && t.ratio != null);
    ok.sort((a, b) => b.ratio - a.ratio);

    if (metaEl) {
      const bits = [];
      if (meta && meta.method) bits.push(meta.method);
      if (meta && meta.machine) bits.push(meta.machine);
      if (meta && meta.system) bits.push(meta.system);
      if (meta && meta.generated_at) bits.push(meta.generated_at);
      metaEl.textContent = bits.join(" · ") || "Suite benchmarks";
    }

    if (!stats || !ok.length) return;
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
        const key = r.util === "[" ? "[" : r.util;
        const b = bi[key] || bi[r.util] || null;
        const depth =
          r.depth === "full" ? "<strong>full</strong>" : esc(r.depth);
        let g = "—";
        let f = "—";
        let x = "—";
        if (b && b.status === "ok") {
          g = fmtMs(b.time_gnu_ms);
          f = `<strong>${esc(fmtMs(b.time_f00_ms))}</strong>`;
          x = fmtRatio(b.ratio);
        } else if (r.speed === "win") {
          // speed-gate win without a published payload race on the site
          x = "<strong>win</strong>";
        } else if (r.speed && r.speed !== "—") {
          x = esc(r.speed);
        }
        return (
          `<tr class="shipped">` +
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
    if (suite) renderStats(suite.tools, suite.meta);
    renderScoreboard(progress, suite);
  });
})();
