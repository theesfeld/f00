/* f00.sh — progressive enhancement: particle field + boot log */
(() => {
  const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // --- boot log (low-fi) ---
  const boot = document.getElementById("boot");
  if (boot && !prefersReduced) {
    const lines = [
      "f00 bios  ·  64-bit freestanding",
      "probe dns  ·  f00.sh / coreutils / clun",
      "mount products  ·  f00tils, clun",
      "ready.",
    ];
    let i = 0;
    const tick = () => {
      if (i >= lines.length) return;
      boot.textContent += (i ? "\n" : "") + lines[i];
      i += 1;
      setTimeout(tick, 280 + Math.random() * 220);
    };
    setTimeout(tick, 350);
  } else if (boot) {
    boot.textContent = "f00 ready.";
  }

  // --- pixel field (technically modern, looks like old phosphor noise) ---
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
    // faint grid
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

    // occasional phosphor smear near logo zone
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
