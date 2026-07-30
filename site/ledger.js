/* f00 open ledger — render site/ledger.json (SSOT) */
(() => {
  const URLS = ["/ledger.json", "ledger.json", "https://f00.sh/ledger.json"];
  const root = document.getElementById("ledger-root");
  const balEl = document.getElementById("ledger-balances");
  const metaEl = document.getElementById("ledger-meta");
  if (!root) return;

  const esc = (s) =>
    String(s ?? "")
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");

  const load = async () => {
    for (const u of URLS) {
      try {
        const r = await fetch(u, { credentials: "omit", cache: "no-store" });
        if (!r.ok) continue;
        return await r.json();
      } catch {
        /* next */
      }
    }
    return null;
  };

  const balances = (entries) => {
    const map = Object.create(null);
    for (const e of entries || []) {
      const cur = (e.currency || "USD").toUpperCase();
      if (!map[cur]) map[cur] = 0;
      const n = parseFloat(e.amount);
      if (!Number.isFinite(n)) continue;
      if (e.direction === "in") map[cur] += n;
      else if (e.direction === "out") map[cur] -= n;
    }
    return map;
  };

  const fmtAmt = (n, cur) => {
    const abs = Math.abs(n);
    const s =
      abs >= 100
        ? abs.toFixed(2)
        : abs >= 1
          ? abs.toFixed(2)
          : abs.toFixed(4).replace(/0+$/, "").replace(/\.$/, "");
    return `${n < 0 ? "−" : ""}${s} ${cur}`;
  };

  const paint = (data) => {
    if (!data || !Array.isArray(data.entries)) {
      root.innerHTML =
        '<p class="ledger-empty">Ledger unavailable. Source: <code class="mono">/ledger.json</code></p>';
      return;
    }

    const pol = data.policy || {};
    if (metaEl) {
      metaEl.innerHTML = [
        pol.open ? "open books" : null,
        pol.no_payroll ? "no payroll" : null,
        pol.one_pool ? "one pool" : null,
        pol.even_split_unrestricted_gifts ? "even split gifts" : null,
        data.updated ? `updated ${esc(data.updated)}` : null,
      ]
        .filter(Boolean)
        .map((t) => `<span class="mono">${esc(t)}</span>`)
        .join(" · ");
    }

    const bal = balances(data.entries);
    if (balEl) {
      const keys = Object.keys(bal);
      if (!keys.length) {
        balEl.innerHTML = '<p class="mono">balances: empty</p>';
      } else {
        balEl.innerHTML =
          '<ul class="facts mono ledger-bal">' +
          keys
            .map((c) => {
              const v = bal[c];
              const cls = v < 0 ? "ledger-neg" : "ledger-pos";
              return `<li class="${cls}">${esc(c)}: ${esc(fmtAmt(v, c))}</li>`;
            })
            .join("") +
          "</ul>";
      }
    }

    const rows = [...data.entries].sort((a, b) => {
      const da = a.date || "";
      const db = b.date || "";
      if (da !== db) return db.localeCompare(da);
      return String(b.id || "").localeCompare(String(a.id || ""));
    });

    if (!rows.length) {
      root.innerHTML = '<p class="ledger-empty mono">no entries yet</p>';
      return;
    }

    const body = rows
      .map((e) => {
        const dir = e.direction || "none";
        const sign =
          dir === "in" ? "+" : dir === "out" ? "−" : "·";
        const amt = e.amount != null ? `${sign}${esc(e.amount)}` : "—";
        const rec = e.receipt
          ? `<a href="${esc(e.receipt)}" rel="noopener">receipt</a>`
          : "—";
        return `<tr class="ledger-row ledger-${esc(dir)}">
          <td class="mono">${esc(e.date || "—")}</td>
          <td class="mono">${esc(e.kind || "—")}</td>
          <td class="mono ledger-amt">${amt} <span class="ledger-cur">${esc(e.currency || "")}</span></td>
          <td class="mono">${esc(e.rail || "—")}</td>
          <td>${esc(e.memo || "")}</td>
          <td class="mono">${esc(e.program || "—")}</td>
          <td class="mono">${esc(e.steward || "—")}</td>
          <td class="mono">${rec}</td>
        </tr>`;
      })
      .join("");

    root.innerHTML = `<div class="ledger-wrap card e-frame-host">
      <table class="ledger-table">
        <thead>
          <tr>
            <th>date</th>
            <th>kind</th>
            <th>amount</th>
            <th>rail</th>
            <th>memo</th>
            <th>program</th>
            <th>steward</th>
            <th>proof</th>
          </tr>
        </thead>
        <tbody>${body}</tbody>
      </table>
    </div>
    <p class="ledger-source mono">
      SSOT:
      <a href="/ledger.json">ledger.json</a>
      · edit in git · every penny public
    </p>`;
  };

  load().then(paint);
})();
