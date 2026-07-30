/** throw — seeded noise & small vector helpers (math is the only uniformity) */

export function mulberry32(seed) {
  let s = seed >>> 0 || 1;
  return () => {
    s |= 0;
    s = (s + 0x6d2b79f5) | 0;
    let t = Math.imul(s ^ (s >>> 15), 1 | s);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export function hash2(ix, iy, seed) {
  let n = Math.imul(ix, 374761393) ^ Math.imul(iy, 668265263) ^ seed;
  n = Math.imul(n ^ (n >>> 13), 1274126177);
  return ((n ^ (n >>> 16)) >>> 0) / 4294967296;
}

export function smoothstep(t) {
  return t * t * (3 - 2 * t);
}

/** Value noise [0,1] */
export function valueNoise2(x, y, seed) {
  const x0 = Math.floor(x);
  const y0 = Math.floor(y);
  const fx = smoothstep(x - x0);
  const fy = smoothstep(y - y0);
  const a = hash2(x0, y0, seed);
  const b = hash2(x0 + 1, y0, seed);
  const c = hash2(x0, y0 + 1, seed);
  const d = hash2(x0 + 1, y0 + 1, seed);
  return a + (b - a) * fx + (c - a) * fy + (a - b - c + d) * fx * fy;
}

/** Multi-octave — each octave different seed offset so not a tiled pattern */
export function fbm2(x, y, seed, octaves = 4) {
  let v = 0;
  let a = 0.5;
  let f = 1;
  let s = seed;
  for (let i = 0; i < octaves; i++) {
    v += a * valueNoise2(x * f, y * f, s);
    s = (s * 1664525 + 1013904223) >>> 0;
    f *= 2.03;
    a *= 0.5;
  }
  return v;
}

export function clamp(x, a, b) {
  return Math.max(a, Math.min(b, x));
}
