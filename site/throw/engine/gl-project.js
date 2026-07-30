/**
 * throw — GPU PROJECT
 * Developed density as texture; fragment shader runs the optical chain.
 * Quality tiers: high (desktop) · fast (iOS / coarse pointers).
 */

const VERT = `
attribute vec2 a_pos;
varying vec2 v_uv;
void main() {
  v_uv = a_pos * 0.5 + 0.5;
  gl_Position = vec4(a_pos, 0.0, 1.0);
}
`;

/* Full optical chain — desktop / high-power */
const FRAG_HIGH = `
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_density;
uniform vec2 u_res;
uniform float u_time;
uniform float u_p;
uniform float u_live;
uniform float u_seed;
uniform vec2 u_film;
uniform vec2 u_tilt;
uniform float u_rz;
uniform float u_focus;
uniform float u_und;
uniform float u_bulb;
uniform vec2 u_bulbPos;

float hash(vec2 p) {
  return fract(sin(dot(p, vec2(127.1, 311.7)) + u_seed) * 43758.5453123);
}
float noise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  float a = hash(i);
  float b = hash(i + vec2(1.0, 0.0));
  float c = hash(i + vec2(0.0, 1.0));
  float d = hash(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
float fbm(vec2 p) {
  return 0.55 * noise(p) + 0.3 * noise(p * 2.03 + 1.7) + 0.15 * noise(p * 4.1 + 3.1);
}

float sampleD(vec2 uv) {
  if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) return 0.0;
  return texture2D(u_density, uv).a;
}

float sampleInk(vec2 uv, float coc) {
  float core = sampleD(uv);
  float hold = fbm(uv * 1.7 + 0.2);
  if (hold > 0.62) coc *= 0.08;
  else if (hold < 0.35) coc *= 1.55;
  if (coc < 0.45) return core;

  float em = fbm(uv * 16.0);
  float anX = 0.45 + em * 1.1;
  float anY = 1.55 - em * 0.9;
  vec2 px = vec2(coc * 0.45) / u_res;
  float acc = 0.0;
  float wt = 0.0;
  for (int y = -2; y <= 2; y++) {
    for (int x = -2; x <= 2; x++) {
      float fx = float(x);
      float fy = float(y);
      float dist = fx * fx * anX + fy * fy * anY;
      float w = exp(-dist / (1.0 + coc));
      w *= 0.55 + 0.45 * fract(em * 7.1 + fx * 0.17 + fy * 0.31);
      acc += sampleD(uv + vec2(fx, fy) * px) * w;
      wt += w;
    }
  }
  return acc / max(wt, 1e-4);
}

void main() {
  float t = u_time;
  float live = clamp(u_live, 0.15, 1.2);
  float hero = 1.0 - clamp(u_p, 0.0, 1.0);

  vec2 uv = v_uv;
  float sx = (uv.x - 0.5) * 2.0;
  float sy = (uv.y - 0.5) * 2.0;

  float und = u_und * live;
  sx += (fbm(vec2(uv.x * 4.0 + t * 0.11, uv.y * 4.0)) - 0.5) * und * 1.05;
  sy += (fbm(vec2(uv.x * 3.6, uv.y * 3.6 + t * 0.09)) - 0.5) * und * 0.9;

  float kx = u_tilt.y * live * 0.85;
  float ky = u_tilt.x * live * 0.85;
  float denom = max(1.0 + kx * sx + ky * sy, 0.72);
  float fx = sx / denom + u_film.x * 1.8 * live;
  float fy = sy / denom + u_film.y * 1.8 * live;

  float cz = cos(u_rz * live * 0.9);
  float sz = sin(u_rz * live * 0.9);
  float rxx = fx * cz - fy * sz;
  float ryy = fx * sz + fy * cz;
  fx = rxx;
  fy = ryy;

  float buckle = (fbm(vec2(uv.x * 2.6 + t * 0.05, uv.y * 2.6 - t * 0.03)) - 0.5) * 0.09 * live;
  float mag = 1.0 / max(1.0 - 1.8 * buckle, 0.78);
  fx *= mag;
  fy *= mag;

  vec2 filmUV = vec2(fx * 0.5 + 0.5, fy * 0.5 + 0.5);

  float filmZ = buckle + (fbm(uv * 8.0) - 0.5) * 0.08;
  float focusErr = abs(filmZ - (u_focus - 0.5) * 0.12);
  float coc = focusErr * (3.0 + 4.0 * live) * min(u_res.x, u_res.y) * 0.0035;
  coc *= 0.55 + 0.55 * hero;

  float dens = sampleInk(filmUV, coc);

  vec2 lp = uv - (0.5 + u_bulbPos * live);
  float r2 = dot(lp, lp);
  float flicker = 0.9 + 0.12 * fbm(vec2(t * 1.1, 0.7));
  float illum = u_bulb * flicker * (1.0 / (0.55 + r2 * 1.6));
  dens = clamp(dens * illum, 0.0, 1.0);

  gl_FragColor = vec4(vec3(0.0), dens);
}
`;

/* Mobile / iOS — same look, far fewer ALU + texture samples */
const FRAG_FAST = `
precision mediump float;
varying vec2 v_uv;
uniform sampler2D u_density;
uniform vec2 u_res;
uniform float u_time;
uniform float u_p;
uniform float u_live;
uniform float u_seed;
uniform vec2 u_film;
uniform vec2 u_tilt;
uniform float u_rz;
uniform float u_focus;
uniform float u_und;
uniform float u_bulb;
uniform vec2 u_bulbPos;

float hash(vec2 p) {
  return fract(sin(dot(p, vec2(127.1, 311.7)) + u_seed) * 43758.5453123);
}
float noise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  float a = hash(i);
  float b = hash(i + vec2(1.0, 0.0));
  float c = hash(i + vec2(0.0, 1.0));
  float d = hash(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

float sampleD(vec2 uv) {
  if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) return 0.0;
  return texture2D(u_density, uv).a;
}

void main() {
  float t = u_time;
  float live = clamp(u_live, 0.15, 1.2);

  vec2 uv = v_uv;
  float sx = (uv.x - 0.5) * 2.0;
  float sy = (uv.y - 0.5) * 2.0;

  /* one-octave undulation — still living, ~1/3 the cost of fbm */
  float und = u_und * live;
  sx += (noise(vec2(uv.x * 3.2 + t * 0.1, uv.y * 3.2)) - 0.5) * und;
  sy += (noise(vec2(uv.x * 2.9, uv.y * 2.9 + t * 0.08)) - 0.5) * und * 0.85;

  float kx = u_tilt.y * live * 0.75;
  float ky = u_tilt.x * live * 0.75;
  float denom = max(1.0 + kx * sx + ky * sy, 0.75);
  float fx = sx / denom + u_film.x * 1.5 * live;
  float fy = sy / denom + u_film.y * 1.5 * live;

  float cz = cos(u_rz * live * 0.85);
  float sz = sin(u_rz * live * 0.85);
  float rxx = fx * cz - fy * sz;
  float ryy = fx * sz + fy * cz;

  /* soft buckle without multi-octave fbm */
  float buckle = (noise(vec2(uv.x * 2.4 + t * 0.05, uv.y * 2.4)) - 0.5) * 0.07 * live;
  float mag = 1.0 / max(1.0 - 1.6 * buckle, 0.8);
  rxx *= mag;
  ryy *= mag;

  vec2 filmUV = vec2(rxx * 0.5 + 0.5, ryy * 0.5 + 0.5);

  /* no CoC kernel — single tap (scale already softens via CSS on mobile) */
  float dens = sampleD(filmUV);

  vec2 lp = uv - (0.5 + u_bulbPos * live);
  float illum = u_bulb * (0.92 + 0.08 * sin(t * 1.05)) * (1.0 / (0.55 + dot(lp, lp) * 1.6));
  dens = clamp(dens * illum, 0.0, 1.0);

  gl_FragColor = vec4(vec3(0.0), dens);
}
`;

function compile(gl, type, src) {
  const s = gl.createShader(type);
  gl.shaderSource(s, src);
  gl.compileShader(s);
  if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {
    console.warn("[throw-gl]", gl.getShaderInfoLog(s));
    gl.deleteShader(s);
    return null;
  }
  return s;
}

function createProgram(gl, vsSrc, fsSrc) {
  const vs = compile(gl, gl.VERTEX_SHADER, vsSrc);
  const fs = compile(gl, gl.FRAGMENT_SHADER, fsSrc);
  if (!vs || !fs) return null;
  const p = gl.createProgram();
  gl.attachShader(p, vs);
  gl.attachShader(p, fs);
  gl.linkProgram(p);
  gl.deleteShader(vs);
  gl.deleteShader(fs);
  if (!gl.getProgramParameter(p, gl.LINK_STATUS)) {
    console.warn("[throw-gl] link", gl.getProgramInfoLog(p));
    return null;
  }
  return p;
}

function preferFastQuality() {
  try {
    if (window.matchMedia("(pointer: coarse)").matches) return true;
    if (window.matchMedia("(max-width: 900px)").matches) return true;
    const ua = navigator.userAgent || "";
    if (/iPhone|iPad|iPod|Android/i.test(ua)) return true;
    /* save-data / low-end hints */
    if (navigator.connection?.saveData) return true;
    if ((navigator.hardwareConcurrency || 8) <= 4) return true;
  } catch (_) {}
  return false;
}

/**
 * @param {HTMLCanvasElement} canvas
 * @param {{ quality?: "high"|"fast" }} [opts]
 */
export function createGlProjector(canvas, opts = {}) {
  const fast = opts.quality === "fast" || (opts.quality !== "high" && preferFastQuality());
  const glOpts = {
    alpha: true,
    premultipliedAlpha: true,
    antialias: false,
    depth: false,
    stencil: false,
    /* preserveDrawingBuffer forces a full copy every frame on mobile GPUs */
    preserveDrawingBuffer: false,
    powerPreference: fast ? "default" : "high-performance",
    desynchronized: true,
  };
  const gl =
    canvas.getContext("webgl", glOpts) ||
    canvas.getContext("experimental-webgl", glOpts);
  if (!gl) return null;

  const prog = createProgram(gl, VERT, fast ? FRAG_FAST : FRAG_HIGH);
  if (!prog) return null;

  const buf = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buf);
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
    gl.STATIC_DRAW
  );

  const locPos = gl.getAttribLocation(prog, "a_pos");
  const U = {
    density: gl.getUniformLocation(prog, "u_density"),
    res: gl.getUniformLocation(prog, "u_res"),
    time: gl.getUniformLocation(prog, "u_time"),
    p: gl.getUniformLocation(prog, "u_p"),
    live: gl.getUniformLocation(prog, "u_live"),
    seed: gl.getUniformLocation(prog, "u_seed"),
    film: gl.getUniformLocation(prog, "u_film"),
    tilt: gl.getUniformLocation(prog, "u_tilt"),
    rz: gl.getUniformLocation(prog, "u_rz"),
    focus: gl.getUniformLocation(prog, "u_focus"),
    und: gl.getUniformLocation(prog, "u_und"),
    bulb: gl.getUniformLocation(prog, "u_bulb"),
    bulbPos: gl.getUniformLocation(prog, "u_bulbPos"),
  };

  const tex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, tex);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

  let tw = 0;
  let th = 0;
  let destroyed = false;

  const uploadDensity = (density, w, h) => {
    if (destroyed) return;
    const rgba = new Uint8Array(w * h * 4);
    for (let i = 0; i < w * h; i++) {
      const a = Math.max(0, Math.min(255, (density[i] * 255 + 0.5) | 0));
      rgba[i * 4] = 0;
      rgba[i * 4 + 1] = 0;
      rgba[i * 4 + 2] = 0;
      rgba[i * 4 + 3] = a;
    }
    tw = w;
    th = h;
    if (canvas.width !== w || canvas.height !== h) {
      canvas.width = w;
      canvas.height = h;
    }
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);
    gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.RGBA,
      w,
      h,
      0,
      gl.RGBA,
      gl.UNSIGNED_BYTE,
      rgba
    );
  };

  /**
   * @param {{ time:number, p:number, liveAmp:number, optics:object, seed:number }} state
   */
  const draw = (state) => {
    if (destroyed || !tw) return;
    const o = state.optics;
    const live = state.liveAmp != null ? state.liveAmp : 1;
    const t = state.time || 0;

    const filmX =
      o.film.x +
      (Math.sin(t * 0.55) * 0.014 + Math.sin(t * 0.19 + state.seed) * 0.01) *
        live;
    const filmY =
      o.film.y +
      (Math.cos(t * 0.47) * 0.012 + Math.cos(t * 0.17) * 0.008) * live;
    const tiltX = o.film.rx * 0.65 + Math.sin(t * 0.31) * 0.022 * live;
    const tiltY = o.film.ry * 0.65 + Math.cos(t * 0.27) * 0.018 * live;
    const rz = o.film.rz * 0.7 + Math.sin(t * 0.23) * 0.014 * live;
    const focus = o.lens.focus + Math.sin(t * 0.21) * 0.05 * live;
    const und = (o.screen.undulation || 0.055) * (0.9 + 0.45 * live);
    const bulb =
      o.bulb.intensity *
      (0.9 + 0.18 * (0.5 + 0.5 * Math.sin(t * 0.37 + 1.1)));

    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);

    gl.useProgram(prog);
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.enableVertexAttribArray(locPos);
    gl.vertexAttribPointer(locPos, 2, gl.FLOAT, false, 0, 0);

    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.uniform1i(U.density, 0);
    gl.uniform2f(U.res, canvas.width, canvas.height);
    gl.uniform1f(U.time, t);
    gl.uniform1f(U.p, state.p || 0);
    gl.uniform1f(U.live, live);
    gl.uniform1f(U.seed, (state.seed % 10000) * 0.001);
    gl.uniform2f(U.film, filmX, filmY);
    gl.uniform2f(U.tilt, tiltX, tiltY);
    gl.uniform1f(U.rz, rz);
    gl.uniform1f(U.focus, focus);
    gl.uniform1f(U.und, und);
    gl.uniform1f(U.bulb, bulb);
    gl.uniform2f(U.bulbPos, o.bulb.x, o.bulb.y);

    gl.drawArrays(gl.TRIANGLES, 0, 6);
  };

  const destroy = () => {
    destroyed = true;
    try {
      gl.deleteTexture(tex);
      gl.deleteBuffer(buf);
      gl.deleteProgram(prog);
    } catch (_) {}
  };

  return { uploadDensity, draw, destroy, gl, quality: fast ? "fast" : "high" };
}
