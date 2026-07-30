/**
 * throw — structure → develop → project → display
 */
import { mulberry32 } from "./math.js";
import { develop, rasterizeTextMask } from "./develop.js";
import { project, sampleOptics, evolveOptics } from "./project.js";

export { develop, project, sampleOptics, evolveOptics, rasterizeTextMask };

/**
 * One full throw. Returns density + base optics so callers can reproject
 * without redeveloping (same specimen, living light).
 */
export function throwTextPlate(opts) {
  const seed =
    opts.seed != null ? opts.seed >>> 0 : (Math.random() * 0xffffffff) >>> 0;
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
    temperature: 0.45 + R.range(0, 0.45),
    time: 0.5 + R.range(0, 0.5),
    agitation: R.range(0.25, 0.85),
  });

  const optics = sampleOptics(seed ^ 0xc0ffee, R);
  const live = evolveOptics(optics, opts.time || 0, seed);
  const rgba = project(density, w, h, live, opts.time || 0);

  return { seed, width: w, height: h, rgba, density, optics };
}

/** Reproject same developed density with living optics (cheaper). */
export function reprojectPlate(density, w, h, optics, seed, time, liveAmp = 1) {
  const live = evolveOptics(optics, time, seed, liveAmp);
  const rgba = project(density, w, h, live, time);
  return { seed, width: w, height: h, rgba, density, optics };
}

export function displayToCanvas(canvas, result) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  if (canvas.width !== result.width || canvas.height !== result.height) {
    canvas.width = result.width;
    canvas.height = result.height;
  }
  const copy = new Uint8ClampedArray(result.rgba.length);
  copy.set(result.rgba);
  ctx.putImageData(new ImageData(copy, result.width, result.height), 0, 0);
}
