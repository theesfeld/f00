/* f00.sh — progressive enhancement: stars + fit splash + particle field */
(() => {
  // Product repos on the hub (add a card when a product releases → list it here too).
  const F00_REPOS = [
    "theesfeld/f00",
    "theesfeld/f00tils",
    "theesfeld/clun",
  ];

  const starsEl = document.getElementById("stars");
  if (starsEl) {
    Promise.all(
      F00_REPOS.map((repo) =>
        fetch(`https://api.github.com/repos/${repo}`, {
          headers: { Accept: "application/vnd.github+json" },
        }).then((r) => (r.ok ? r.json() : null))
      )
    )
      .then((rows) => {
        const total = rows.reduce(
          (n, row) => n + (row && typeof row.stargazers_count === "number" ? row.stargazers_count : 0),
          0
        );
        const label = total === 1 ? "1 star" : `${total.toLocaleString("en-US")} stars`;
        starsEl.textContent = `★ ${label}`;
        starsEl.title = `Across ${F00_REPOS.length} f00 repos`;
      })
      .catch(() => {
        starsEl.hidden = true;
      });
  }

  // Scale "f00" to the same width as the product card row.
  // Measure natural text width (max-content) — never width:100%, that broke the previous fit.
  const splash = document.querySelector(".splash");
  const frame = document.querySelector(".splash-frame");
  const grid = document.querySelector(".products .grid");
  const fitSplash = () => {
    if (!splash || !frame) return;
    const target = Math.floor(
      (grid || frame).getBoundingClientRect().width
    );
    if (target <= 0) return;

    splash.style.width = "max-content";
    splash.style.display = "inline-block";
    splash.style.transform = "none";

    // Binary search font-size so glyph row width == card row width.
    let lo = 32;
    let hi = Math.min(900, Math.ceil(target * 1.2));
    while (lo < hi) {
      const mid = Math.ceil((lo + hi) / 2);
      splash.style.fontSize = `${mid}px`;
      const w = splash.getBoundingClientRect().width;
      if (w <= target) lo = mid;
      else hi = mid - 1;
    }
    splash.style.fontSize = `${lo}px`;

    // Final nudge if still slightly under/over.
    let w = splash.getBoundingClientRect().width;
    if (w > 0 && Math.abs(target - w) > 1) {
      const scaled = Math.max(32, Math.floor(lo * (target / w)));
      splash.style.fontSize = `${scaled}px`;
      w = splash.getBoundingClientRect().width;
      if (w > target) {
        splash.style.fontSize = `${Math.max(32, scaled - 1)}px`;
      }
    }
  };
  if (splash && frame) {
    fitSplash();
    requestAnimationFrame(fitSplash);
    if (typeof ResizeObserver !== "undefined") {
      const ro = new ResizeObserver(fitSplash);
      ro.observe(frame);
      if (grid) ro.observe(grid);
    } else {
      window.addEventListener("resize", fitSplash, { passive: true });
    }
    if (document.fonts && document.fonts.ready) {
      document.fonts.ready.then(fitSplash).catch(() => {});
    }
  }

  const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  const canvas = document.getElementById("field");
  if (!canvas || prefersReduced) return;
  const ctx = canvas.getContext("2d", { alpha: true });
  if (!ctx) return;

  let w = 0;
  let h = 0;
  let dpr = 1;
  let dots = [];
  let raf = 0;

  const resize = () => {
    dpr = Math.min(window.devicePixelRatio || 1, 2);
    w = window.innerWidth;
    h = window.innerHeight;
    canvas.width = Math.floor(w * dpr);
    canvas.height = Math.floor(h * dpr);
    canvas.style.width = `${w}px`;
    canvas.style.height = `${h}px`;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    const count = Math.floor((w * h) / 14000);
    dots = Array.from({ length: count }, () => ({
      x: Math.random() * w,
      y: Math.random() * h,
      v: 0.15 + Math.random() * 0.55,
      s: Math.random() < 0.15 ? 2 : 1,
      a: 0.15 + Math.random() * 0.55,
    }));
  };

  const frame = (t) => {
    ctx.clearRect(0, 0, w, h);
    ctx.strokeStyle = "rgba(150, 240, 106, 0.04)";
    ctx.lineWidth = 1;
    const step = 48;
    const ox = (t * 0.01) % step;
    for (let x = -step + ox; x < w + step; x += step) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, h);
      ctx.stroke();
    }
    for (let y = 0; y < h; y += step) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(w, y);
      ctx.stroke();
    }

    for (const d of dots) {
      d.y += d.v;
      if (d.y > h + 4) {
        d.y = -4;
        d.x = Math.random() * w;
      }
      ctx.fillStyle = `rgba(150, 240, 106, ${d.a})`;
      ctx.fillRect(Math.floor(d.x), Math.floor(d.y), d.s, d.s);
    }

    if (Math.random() < 0.02) {
      const y = h * (0.18 + Math.random() * 0.2);
      ctx.fillStyle = "rgba(150, 240, 106, 0.03)";
      ctx.fillRect(0, y, w, 2 + Math.random() * 3);
    }

    raf = requestAnimationFrame(frame);
  };

  window.addEventListener("resize", resize, { passive: true });
  resize();
  raf = requestAnimationFrame(frame);

  document.addEventListener("visibilitychange", () => {
    if (document.hidden) cancelAnimationFrame(raf);
    else raf = requestAnimationFrame(frame);
  });
})();
