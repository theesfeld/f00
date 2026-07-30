/**
 * throw — DEVELOP stage (film emulsion)
 *
 * Manufactured binary intent (mask) → silver density field after development.
 * Entropy enters as crystal distribution + development temperature/time —
 * never a flat “opacity = mask” and never a uniform soft filter.
 */
import { fbm2, clamp, mulberry32 } from "./math.js";

/**
 * @typedef {object} DevelopParams
 * @property {number} seed
 * @property {number} [temperature]  development heat (affects grain clump)
 * @property {number} [time]         development duration
 * @property {number} [agitation]    non-uniform bath motion
 */

/**
 * Build a developed density map from an ideal mask (0..1 alpha intent).
 * Output density D in 0..1 — local, non-uniform.
 *
 * @param {Float32Array} mask  length w*h, ideal coverage
 * @param {number} w
 * @param {number} h
 * @param {DevelopParams} params
 * @returns {Float32Array} density
 */
export function develop(mask, w, h, params) {
  const seed = params.seed >>> 0;
  const rnd = mulberry32(seed ^ 0x0deb01);
  const temperature = params.temperature ?? 0.45 + rnd() * 0.35;
  const time = params.time ?? 0.55 + rnd() * 0.4;
  const agitation = params.agitation ?? 0.3 + rnd() * 0.5;

  const D = new Float32Array(w * h);
  const invW = 1 / Math.max(w, 1);
  const invH = 1 / Math.max(h, 1);

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const i = y * w + x;
      const intent = mask[i];
      if (intent <= 0.001) {
        D[i] = 0;
        continue;
      }

      const u = x * invW;
      const v = y * invH;

      /* crystal field — different scale/seed than neighbors; not a texture stamp */
      const crystals = fbm2(u * 28, v * 28, seed ^ 0xc1a55);
      const clump = fbm2(u * 9 + 2, v * 9, seed ^ 0x51e1);
      const bath = fbm2(u * 3.5 + agitation, v * 3.1 - agitation, seed ^ 0xba7);

      /* development rate varies with heat, time, local bath */
      const rate =
        0.35 +
        0.4 * temperature * time +
        0.25 * bath +
        0.2 * (crystals - 0.5) +
        0.15 * (clump - 0.5);

      /* density: intent times local development — never flat copy of mask */
      let dens = intent * clamp(rate, 0.15, 1.35);

      /* sparse “thin emulsion” islands keep nearly full fidelity (straightness can remain) */
      const hold = fbm2(u * 2.2, v * 2.0, seed ^ 0x5011d);
      if (hold > 0.62) {
        dens = intent * (0.85 + 0.15 * dens);
      }

      D[i] = clamp(dens, 0, 1);
    }
  }

  return D;
}

/**
 * Rasterize simple black text intent into a mask (for plates).
 * This is the manufactured idea only — never shown raw.
 */
export function rasterizeTextMask(w, h, text, fontPx, fontFamily, inkPad) {
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
