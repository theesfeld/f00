/**
 * throw — PROJECT (optical chain)
 * Developed density through imperfect bulb / film / lens / screen.
 * Geometry of the whole plate changes. Local CoC — never flat blur.
 */
import { fbm2, clamp } from "./math.js";

export function sampleOptics(seed, rnd) {
  return {
    seed,
    bulb: {
      x: rnd.signed(0.12),
      y: rnd.signed(0.1),
      z: 0.7 + rnd.range(0, 0.4),
      intensity: 0.85 + rnd.range(0, 0.4),
      temp: 0.3 + rnd.range(0, 0.5),
    },
    film: {
      x: rnd.signed(0.035),
      y: rnd.signed(0.03),
      rx: rnd.signed(0.12),
      ry: rnd.signed(0.1),
      rz: rnd.signed(0.06),
    },
    lens: {
      z: 0.3 + rnd.range(0, 0.25),
      focus: 0.35 + rnd.range(0, 0.4),
      fstop: 0.25 + rnd.range(0, 0.55),
      soil: rnd.range(0.05, 0.4),
    },
    screen: {
      z: 1.0 + rnd.range(0, 0.45),
      tilt: rnd.signed(0.08),
      undulation: 0.04 + rnd.range(0, 0.08),
    },
  };
}

/**
 * Live optical drift — same specimen, air not frozen.
 * liveAmp (0..1): stronger when mark is large, calmer when small.
 */
export function evolveOptics(opt, time, seed, liveAmp = 1) {
  const a = Math.max(0.15, Math.min(1.2, liveAmp));
  const o = {
    seed: opt.seed,
    bulb: { ...opt.bulb },
    film: { ...opt.film },
    lens: { ...opt.lens },
    screen: { ...opt.screen },
  };
  const t = time;
  o.film.x +=
    (Math.sin(t * 0.7) * 0.008 + (fbm2(t * 0.1, 0.2, seed) - 0.5) * 0.01) * a;
  o.film.y +=
    (Math.cos(t * 0.55) * 0.006 +
      (fbm2(0.3, t * 0.12, seed ^ 1) - 0.5) * 0.008) *
    a;
  o.film.rx += Math.sin(t * 0.33) * 0.015 * a;
  o.film.ry += Math.cos(t * 0.29) * 0.012 * a;
  o.film.rz += Math.sin(t * 0.21) * 0.01 * a;
  o.bulb.intensity *= 0.92 + 0.16 * fbm2(t * 0.4, 1.1, seed ^ 2) * a;
  o.lens.focus += Math.sin(t * 0.19) * 0.04 * a;
  return o;
}

export function project(density, w, h, opt, time = 0) {
  const out = new Uint8ClampedArray(w * h * 4);
  const invW = 1 / Math.max(w - 1, 1);
  const invH = 1 / Math.max(h - 1, 1);
  const seed = opt.seed;

  const sampleD = (x, y) => {
    const xx = clamp(x, 0, w - 1);
    const yy = clamp(y, 0, h - 1);
    const x0 = Math.floor(xx);
    const y0 = Math.floor(yy);
    const x1 = Math.min(x0 + 1, w - 1);
    const y1 = Math.min(y0 + 1, h - 1);
    const fx = xx - x0;
    const fy = yy - y0;
    const a = density[y0 * w + x0];
    const b = density[y0 * w + x1];
    const c = density[y1 * w + x0];
    const d = density[y1 * w + x1];
    return a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy;
  };

  for (let py = 0; py < h; py++) {
    for (let px = 0; px < w; px++) {
      const u = px * invW;
      const v = py * invH;
      let sx = (u - 0.5) * 2;
      let sy = (v - 0.5) * 2;

      /* screen surface — never flat */
      const und = opt.screen.undulation || 0.05;
      sx +=
        (fbm2(u * 5 + time * 0.08, v * 5, seed ^ 0x5c12) - 0.5) *
        und *
        (1 + Math.abs(opt.screen.tilt) * 8);
      sy +=
        (fbm2(u * 4.2, v * 4.2 + time * 0.07, seed ^ 0x5c13) - 0.5) * und * 0.9;

      /* film plane keystone + gate */
      const kx = opt.film.ry;
      const ky = opt.film.rx;
      let denom = 1 + kx * sx + ky * sy;
      denom = Math.max(denom, 0.5);
      let fx = sx / denom + opt.film.x * 2.5;
      let fy = sy / denom + opt.film.y * 2.5;

      const cz = Math.cos(opt.film.rz);
      const sz = Math.sin(opt.film.rz);
      const rxx = fx * cz - fy * sz;
      const ryy = fx * sz + fy * cz;
      fx = rxx;
      fy = ryy;

      /* buckle magnification field */
      const buckle =
        (fbm2(u * 3 + time * 0.03, v * 3, seed ^ 0xb11c) - 0.5) * 0.14;
      const mag = 1 / Math.max(1 - 2.2 * buckle, 0.62);
      fx *= mag;
      fy *= mag;

      const filmX = (fx * 0.5 + 0.5) * (w - 1);
      const filmY = (fy * 0.5 + 0.5) * (h - 1);

      /* local CoC from focus error + soil — SPATIAL, not one blur */
      const filmZ =
        buckle + (fbm2(u * 8, v * 8, seed ^ 0xf0c1) - 0.5) * 0.08;
      const focusErr = Math.abs(filmZ - (opt.lens.focus - 0.5) * 0.12);
      let coc =
        focusErr *
        (3 + 6 * (1 - opt.lens.fstop)) *
        (0.5 + opt.lens.soil) *
        Math.min(w, h) *
        0.0035;

      const hold = fbm2(u * 1.7 + 0.2, v * 1.8, seed ^ 0x5011d);
      /* order: some regions nearly no CoC (straightness survives) */
      if (hold > 0.62) coc *= 0.08;
      else if (hold < 0.35) coc *= 1.6;

      let dens = 0;
      if (coc < 0.55) {
        dens = sampleD(filmX, filmY);
      } else {
        /* anisotropic soft CoC — few taps, spatial not flat blur */
        const em = fbm2(u * 16, v * 16, seed ^ 0xe111);
        const anX = 0.45 + em * 1.1;
        const anY = 1.55 - em * 0.9;
        const taps = 2;
        let acc = 0;
        let wt = 0;
        const step = coc * 0.45;
        for (let ty = -taps; ty <= taps; ty++) {
          for (let tx = -taps; tx <= taps; tx++) {
            const dist = tx * tx * anX + ty * ty * anY;
            let ww = Math.exp(-dist / (1 + coc));
            /* cheap spatial weight — avoid fbm per tap */
            ww *= 0.55 + 0.45 * ((tx * 0.17 + ty * 0.31 + em) % 1);
            acc += sampleD(filmX + tx * step, filmY + ty * step) * ww;
            wt += ww;
          }
        }
        dens = acc / Math.max(wt, 1e-6);
      }

      /* bulb illumination — off-center, never flat */
      const lx = u - (0.5 + opt.bulb.x);
      const ly = v - (0.5 + opt.bulb.y);
      const r2 = lx * lx + ly * ly;
      const flicker =
        0.88 + 0.2 * fbm2(time * 1.1, 0.7, seed ^ 0xb11b);
      const illum =
        opt.bulb.intensity *
        flicker *
        (1 / (0.55 + r2 * (1.4 + opt.bulb.z * 1.2)));

      dens = clamp(dens * illum, 0, 1);

      const o = (py * w + px) * 4;
      const a = Math.round(dens * 255);
      out[o] = 0;
      out[o + 1] = 0;
      out[o + 2] = 0;
      out[o + 3] = a;
    }
  }

  return out;
}
