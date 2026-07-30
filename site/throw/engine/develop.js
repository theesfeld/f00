/**
 * throw — DEVELOP (emulsion)
 * Ideal mask → silver density. Entropy at the crystal / bath level.
 * Edges etch irregularly. Interior density mottles. Not a soft filter.
 */
import { fbm2, clamp, mulberry32 } from "./math.js";

/**
 * @param {Float32Array} mask
 * @param {number} w
 * @param {number} h
 * @param {{seed:number,temperature?:number,time?:number,agitation?:number}} params
 */
export function develop(mask, w, h, params) {
  const seed = params.seed >>> 0;
  const rnd = mulberry32(seed ^ 0x0deb01);
  const temperature = params.temperature ?? 0.5 + rnd() * 0.4;
  const time = params.time ?? 0.55 + rnd() * 0.4;
  const agitation = params.agitation ?? 0.35 + rnd() * 0.55;

  const D = new Float32Array(w * h);
  const invW = 1 / Math.max(w, 1);
  const invH = 1 / Math.max(h, 1);

  /* distance-to-edge field for irregular etch (approx via neighbor emptiness) */
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const i = y * w + x;
      const intent = mask[i];
      const u = x * invW;
      const v = y * invH;

      if (intent <= 0.001) {
        /* fog / clear area — tiny chemical fog sometimes */
        const fog = fbm2(u * 12, v * 12, seed ^ 0xf09);
        D[i] = fog > 0.92 ? (fog - 0.92) * 0.08 : 0;
        continue;
      }

      /* how close to edge? sample neighborhood emptiness */
      let empty = 0;
      const r = 2;
      for (let dy = -r; dy <= r; dy++) {
        for (let dx = -r; dx <= r; dx++) {
          const xx = x + dx;
          const yy = y + dy;
          if (xx < 0 || yy < 0 || xx >= w || yy >= h) {
            empty += 1;
            continue;
          }
          if (mask[yy * w + xx] < 0.2) empty += 1;
        }
      }
      const edge = empty / ((2 * r + 1) * (2 * r + 1));

      const crystals = fbm2(u * 40, v * 40, seed ^ 0xc1a55);
      const clump = fbm2(u * 11 + 2, v * 11, seed ^ 0x51e1);
      const bath = fbm2(u * 4 + agitation * 2, v * 3.5 - agitation, seed ^ 0xba7);
      const fiber = fbm2(u * 60, v * 8, seed ^ 0xf1be); /* directional gel stress */

      let rate =
        0.55 +
        0.55 * temperature * time +
        0.35 * (bath - 0.5) +
        0.3 * (crystals - 0.5) +
        0.25 * (clump - 0.5) +
        0.15 * (fiber - 0.5);

      /* etch: at edges, irregular eat-in / build-out — silhouette not CAD */
      const etch = fbm2(u * 22 + 1, v * 22, seed ^ 0xe7c4);
      if (edge > 0.15) {
        const bite = (etch - 0.5) * 1.4 * edge;
        rate += bite;
        /* sometimes edge vanishes (underdeveloped fringe) */
        if (etch < 0.28 && edge > 0.35) {
          D[i] = 0;
          continue;
        }
        /* sometimes edge grows (overdeveloped fringe) */
        if (etch > 0.72 && edge > 0.25 && intent > 0.3) {
          rate += 0.35;
        }
      }

      let dens = intent * clamp(rate, 0.05, 1.5);

      /* interior mottling — silver not flat black paint */
      const mottle = 0.75 + 0.5 * (fbm2(u * 18, v * 18, seed ^ 0x1117) - 0.5);
      dens *= mottle;

      /* hold islands: nearly full density, sharp-ish (order in disorder) */
      const hold = fbm2(u * 1.8, v * 1.9, seed ^ 0x5011d);
      if (hold > 0.68 && edge < 0.25) {
        dens = Math.max(dens, intent * (0.9 + 0.1 * dens));
      }

      D[i] = clamp(dens, 0, 1);
    }
  }

  return D;
}

export function rasterizeTextMask(w, h, text, fontPx, fontFamily) {
  const c = document.createElement("canvas");
  c.width = w;
  c.height = h;
  const ctx = c.getContext("2d", { willReadFrequently: true });
  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = "#000";
  ctx.font = `400 ${fontPx}px ${fontFamily}`;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText(text, w / 2, h / 2 + fontPx * 0.02);
  const img = ctx.getImageData(0, 0, w, h);
  const mask = new Float32Array(w * h);
  for (let i = 0; i < w * h; i++) {
    mask[i] = img.data[i * 4 + 3] / 255;
  }
  return mask;
}
