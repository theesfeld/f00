/**
 * throw — PROJECT stage (optical chain)
 *
 * Developed density → light through film → lens → screen.
 * Every distance/angle/cleanliness is imperfect and LOCAL where it matters.
 * Circle of confusion is not a single radius for the whole plate.
 */
import { fbm2, clamp } from "./math.js";

/**
 * @typedef {object} OpticalChain
 * @property {number} seed
 * @property {{x:number,y:number,z:number,intensity:number,temp:number}} bulb
 * @property {{x:number,y:number,rx:number,ry:number,rz:number}} film  film plane pose
 * @property {{z:number,focus:number,fstop:number,soil:number}} lens
 * @property {{z:number,tilt:number}} screen
 */

/** Sample independent but bounded optical chain for one plate throw */
export function sampleOptics(seed, rnd) {
  return {
    seed,
    bulb: {
      x: rnd.signed(0.08),
      y: rnd.signed(0.06),
      z: 0.85 + rnd.range(0, 0.25),
      intensity: 0.75 + rnd.range(0, 0.35),
      temp: 0.35 + rnd.range(0, 0.45), /* 0 cool … 1 warm — monochrome ink ignores hue, uses as falloff shape */
    },
    film: {
      x: rnd.signed(0.012),
      y: rnd.signed(0.01),
      rx: rnd.signed(0.04),
      ry: rnd.signed(0.035),
      rz: rnd.signed(0.02),
    },
    lens: {
      z: 0.35 + rnd.range(0, 0.2),
      focus: 0.4 + rnd.range(0, 0.35),
      fstop: 0.4 + rnd.range(0, 0.5),
      soil: rnd.range(0, 0.25),
    },
    screen: {
      z: 1.1 + rnd.range(0, 0.35),
      tilt: rnd.signed(0.03),
    },
  };
}

/**
 * Project density field to display RGBA buffer (premultiplied black ink).
 * @param {Float32Array} density  developed D
 * @param {number} w
 * @param {number} h
 * @param {OpticalChain} opt
 * @param {number} time  continuous time for living air (optional)
 * @returns {Uint8ClampedArray} rgba length w*h*4
 */
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
      /* screen coords centered */
      let sx = (u - 0.5) * 2;
      let sy = (v - 0.5) * 2;

      /* screen undulation — local, not flat */
      const scr =
        (fbm2(u * 4 + time * 0.05, v * 4, seed ^ 0x5c12) - 0.5) * 0.04 * opt.screen.tilt * 10;
      sx += scr * 0.5;
      sy += (fbm2(u * 3.5, v * 3.5 + time * 0.04, seed ^ 0x5c13) - 0.5) * 0.03;

      /* film plane pose: keystone + gate slip */
      const kx = opt.film.ry;
      const ky = opt.film.rx;
      let denom = 1 + kx * sx + ky * sy;
      denom = Math.max(denom, 0.55);
      let fx = sx / denom + opt.film.x * 2;
      let fy = sy / denom + opt.film.y * 2;
      /* roll */
      const cz = Math.cos(opt.film.rz);
      const sz = Math.sin(opt.film.rz);
      const rx = fx * cz - fy * sz;
      const ry = fx * sz + fy * cz;
      fx = rx;
      fy = ry;

      /* buckle: local film Z field → magnification (throw change) */
      const buckle =
        (fbm2(u * 2.5 + opt.film.x, v * 2.5, seed ^ 0xb11c) - 0.5) * 0.08;
      const mag = 1 / Math.max(1 - 1.8 * buckle, 0.7);
      fx *= mag;
      fy *= mag;

      /* map back to film pixel */
      const filmX = (fx * 0.5 + 0.5) * (w - 1);
      const filmY = (fy * 0.5 + 0.5) * (h - 1);

      /* local circle of confusion — depends on film Z vs focus plane, LOCAL */
      const filmZ =
        buckle +
        (fbm2(u * 6, v * 6, seed ^ 0xf0c1) - 0.5) * 0.05;
      const focusErr = Math.abs(filmZ - (opt.lens.focus - 0.5) * 0.1);
      const coc =
        focusErr * (2.5 + 4 * (1 - opt.lens.fstop)) * (0.4 + opt.lens.soil) * w * 0.002;

      /* islands of near-perfect focus (straightness allowed) */
      const hold = fbm2(u * 2.0, v * 2.1, seed ^ 0x5011d);
      const useCoc = hold > 0.58 ? coc * 0.05 : coc;

      let dens = 0;
      if (useCoc < 0.35) {
        dens = sampleD(filmX, filmY);
      } else {
        /* anisotropic local gather — NOT a uniform Gaussian over whole glyph */
        const em = fbm2(u * 15, v * 15, seed ^ 0xe111);
        const anX = 0.5 + em;
        const anY = 1.5 - em;
        const taps = 5;
        let acc = 0;
        let wt = 0;
        for (let ty = -taps; ty <= taps; ty++) {
          for (let tx = -taps; tx <= taps; tx++) {
            const dist = (tx * tx) * anX + (ty * ty) * anY;
            const ww =
              Math.exp(-dist / (1 + useCoc * useCoc * 0.5)) *
              (0.5 + fbm2(u * 40 + tx, v * 40 + ty, seed));
            acc += sampleD(filmX + tx * useCoc * 0.35, filmY + ty * useCoc * 0.35) * ww;
            wt += ww;
          }
        }
        dens = acc / Math.max(wt, 1e-6);
      }

      /* illumination: inverse-ish falloff from bulb, non-centered */
      const lx = u - (0.5 + opt.bulb.x);
      const ly = v - (0.5 + opt.bulb.y);
      const r2 = lx * lx + ly * ly;
      const illum =
        opt.bulb.intensity *
        (1 / (0.65 + r2 * (1.2 + opt.bulb.z))) *
        (0.85 + 0.15 * opt.bulb.temp);

      dens = clamp(dens * illum, 0, 1);

      const o = (py * w + px) * 4;
      const a = Math.round(dens * 255);
      /* premultiplied black ink */
      out[o] = 0;
      out[o + 1] = 0;
      out[o + 2] = 0;
      out[o + 3] = a;
    }
  }

  return out;
}
