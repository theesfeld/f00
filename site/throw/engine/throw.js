/**
 * throw — public API
 *
 * structure (mask) → develop → project → pixels for the browser
 */
import { mulberry32 } from "./math.js";
import { develop, rasterizeTextMask } from "./develop.js";
import { project, sampleOptics } from "./project.js";

export { develop, project, sampleOptics, rasterizeTextMask };

/**
 * Full throw for a text plate (e.g. logo).
 * @param {object} opts
 * @param {string} opts.text
 * @param {number} opts.width
 * @param {number} opts.height
 * @param {number} opts.fontPx
 * @param {string} [opts.fontFamily]
 * @param {number} [opts.seed]
 * @param {number} [opts.time]
 */
export function throwTextPlate(opts) {
  const seed =
    opts.seed != null
      ? opts.seed >>> 0
      : (Math.random() * 0xffffffff) >>> 0;
  const rnd = mulberry32(seed);
  const R = {
    rnd,
    range: (a, b) => a + (b - a) * rnd(),
    signed: (m) => -m + 2 * m * rnd(),
  };

  const w = opts.width | 0;
  const h = opts.height | 0;
  const fontFamily =
    opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';

  const mask = rasterizeTextMask(
    w,
    h,
    opts.text || "f00",
    opts.fontPx,
    fontFamily
  );

  const density = develop(mask, w, h, {
    seed: seed ^ 0x0deb,
    temperature: 0.4 + R.range(0, 0.4),
    time: 0.5 + R.range(0, 0.45),
    agitation: R.range(0.2, 0.7),
  });

  const opt = sampleOptics(seed ^ 0xc0ffee, R);
  const rgba = project(density, w, h, opt, opts.time || 0);

  return { seed, width: w, height: h, rgba, optics: opt };
}

/**
 * Blit throw result to a canvas (display stage).
 */
export function displayToCanvas(canvas, result) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  if (canvas.width !== result.width || canvas.height !== result.height) {
    canvas.width = result.width;
    canvas.height = result.height;
  }
  const img = new ImageData(
    new Uint8ClampedArray(result.rgba.buffer.slice(0)),
    result.width,
    result.height
  );
  /* ImageData needs owned buffer */
  const copy = new Uint8ClampedArray(result.rgba.length);
  copy.set(result.rgba);
  ctx.putImageData(new ImageData(copy, result.width, result.height), 0, 0);
}
