/* f00 — continuous film PROJECTOR (entropy, no neutral state)
 *
 * DOCTRINE: whenever this surface is projected onto the display
 * (never “page load”), treat it as a new organic thing — a unique throw
 * of light through plate → lens → screen. No exact duplicate projections.
 * No rest pose. No timers. Brand identity fixed; optics always unique.
 *
 * Dream: light → imperfect film plate → lens → screen.
 * Not a font. Patterns + chaos drive optics forever.
 */
(() => {
  const VERT = `
    attribute vec2 a_pos;
    varying vec2 v_uv;
    void main() {
      v_uv = a_pos * 0.5 + 0.5;
      gl_Position = vec4(a_pos, 0.0, 1.0);
    }
  `;

  const FRAG = `
    precision highp float;
    varying vec2 v_uv;
    uniform sampler2D u_tex;
    uniform vec2 u_res;
    uniform float u_time;
    uniform float u_p;
    /* continuous dynamical readouts — never “event on/off” */
    uniform vec2 u_weave;
    uniform vec2 u_tilt;
    uniform float u_buckle;
    uniform float u_breath;
    uniform float u_wave;
    uniform float u_energy; /* soft emergent amplitude from dynamics */
    uniform float u_edgeFloor; /* soft-edge floor px — unique per projection */
    uniform float u_lamp; /* lamp falloff strength — unique per projection */

    float hash(vec2 p) {
      return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
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
    float fbm2(vec2 p) {
      return 0.65 * noise(p) + 0.35 * noise(p * 1.93 + 2.7);
    }

    /*
     * Soft projected edge: multi-tap alpha ALWAYS blurred.
     * Floor prevents CAD-perfect silhouettes. Anisotropic = horizontal gate.
     * u_edgeFloor is unique per projection (specimen bleed).
     */
    float sampleInkSoft(vec2 uv, float defocusPx) {
      float floorPx = u_edgeFloor; /* never razor; varies per throw */
      float spread = floorPx + defocusPx;
      vec2 px = vec2(1.2, 0.88) * spread / u_res;
      float acc = 0.0;
      float wsum = 0.0;
      for (int y = -3; y <= 3; y++) {
        for (int x = -3; x <= 3; x++) {
          float w = exp(-0.32 * float(x * x + y * y));
          vec2 suv = uv + vec2(float(x), float(y)) * px;
          float a = 0.0;
          if (suv.x > -0.05 && suv.x < 1.05 && suv.y > -0.05 && suv.y < 1.05) {
            a = texture2D(u_tex, clamp(suv, 0.0, 1.0)).a;
          }
          acc += a * w;
          wsum += w;
        }
      }
      float soft = acc / wsum;
      float core = 0.0;
      if (uv.x > 0.0 && uv.x < 1.0 && uv.y > 0.0 && uv.y < 1.0) {
        core = texture2D(u_tex, uv).a;
      }
      /* dye bleed: soft under + around edge, never binary mask */
      return max(soft * 0.92, core * 0.55 + soft * 0.45);
    }

    /*
     * Inverse optical path: screen → film.
     * Whole plate only. Floors baked into uniforms from host.
     */
    vec2 projectUV(vec2 uv, float t, out float defocus) {
      float hero = 1.0 - clamp(u_p, 0.0, 1.0);
      float live = 0.6 + 0.4 * hero;
      vec2 p = uv - 0.5;

      /* 1 gate weave — rigid plate slip (always) */
      p -= u_weave * live;

      /* 2 continuous buckle field + energy-coupled swell (never flat) */
      float zField = (fbm2(uv * 1.05 + vec2(t * 0.055, t * 0.04)) - 0.5);
      zField += sin(uv.x * 3.7 + t * 0.41 + u_wave) * cos(uv.y * 3.1 - t * 0.37) * 0.45;
      float Z = zField * (0.0045 + 0.006 * u_energy) * live + u_buckle * live;
      float mag = 1.0 / max(1.0 - 2.6 * Z, 0.8);
      p *= mag;

      /* 3 plate tilt → soft keystone of the whole throw */
      vec2 tilt = u_tilt * live;
      float denom = 1.0 + tilt.y * p.y + tilt.x * p.x;
      p /= max(denom, 0.76);

      /* 4 throw breathing */
      float breath = u_breath;
      p /= breath;

      /* 5 always-on screen/air undulation */
      float w = u_wave;
      vec2 air = vec2(
        sin(p.y * 5.2 + t * 0.33 + w) * (0.0022 + 0.0015 * u_energy),
        cos(p.x * 4.6 - t * 0.29 - w * 0.7) * (0.0019 + 0.0012 * u_energy)
      ) * live;
      air += (vec2(fbm2(p * 2.0 + t * 0.08), fbm2(p.yx * 2.0 - t * 0.07)) - 0.5)
           * (0.0014 + 0.001 * u_energy) * live;
      p += air;

      defocus = 0.55
        + abs(Z) * 55.0
        + length(tilt) * 14.0
        + abs(breath - 1.0) * 40.0
        + u_energy * 0.9;

      return p + 0.5;
    }

    void main() {
      float defocus = 0.0;
      vec2 uv = projectUV(v_uv, u_time, defocus);
      float a = sampleInkSoft(uv, defocus);

      /* soft lamp falloff — throw never evenly lit (strength per projection) */
      vec2 q = v_uv - 0.5;
      a *= clamp(1.0 - u_lamp * dot(q, q) * 4.0, 0.84, 1.0);

      /* flat black ink, imperfect edge already in alpha */
      vec3 col = vec3(0.045);
      a = clamp(a, 0.0, 1.0);
      gl_FragColor = vec4(col * a, a);
    }
  `;

  function compile(gl, type, src) {
    const s = gl.createShader(type);
    gl.shaderSource(s, src);
    gl.compileShader(s);
    if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {
      console.warn("[film-logo] shader", gl.getShaderInfoLog(s));
      gl.deleteShader(s);
      return null;
    }
    return s;
  }

  function createProgram(gl, vs, fs) {
    const p = gl.createProgram();
    gl.attachShader(p, vs);
    gl.attachShader(p, fs);
    gl.linkProgram(p);
    if (!gl.getProgramParameter(p, gl.LINK_STATUS)) {
      console.warn("[film-logo] link", gl.getProgramInfoLog(p));
      return null;
    }
    return p;
  }

  const FLOOR = 0.008;
  function floorAbs(v, m) {
    const mm = m == null ? FLOOR : m;
    if (v === 0 || !Number.isFinite(v)) return mm;
    return Math.sign(v) * Math.max(Math.abs(v), mm);
  }

  function randn() {
    /* Box-Muller — continuous noise, not a schedule */
    let u = 0;
    let v = 0;
    while (u === 0) u = Math.random();
    while (v === 0) v = Math.random();
    return Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);
  }

  /**
   * Continuous dynamical system (Rössler-ish + OU walks).
   * Uniforms are READOUTS — never idle/event modes, never reset-to-identity.
   */
  function createDynamics() {
    /* asymmetric ICs — never the origin */
    const s = {
      x: 0.4 + Math.random() * 0.3,
      y: -0.2 + Math.random() * 0.4,
      z: 0.15 + Math.random() * 0.2,
      bx: (Math.random() - 0.5) * 0.4,
      by: (Math.random() - 0.5) * 0.4,
      w: 0.55 + Math.random() * 0.35,
      phi: Math.random() * Math.PI * 2,
      e: 0.12 + Math.random() * 0.08, /* energy channel never starts at 0 */
    };
    const a = 0.2;
    const c = 5.7;

    return {
      update(dt) {
        dt = Math.min(Math.max(dt, 0), 0.05);
        const { x, y, z } = s;
        const bz = 0.2 + 0.06 * s.bx;
        /* Euler is enough; incommensurate drifts break metronome */
        s.x += dt * (-y - z);
        s.y += dt * (x + a * y);
        s.z += dt * (bz + z * (x - c));
        /* soft clamp chaos so it stays optical, not nauseating */
        s.x = Math.tanh(s.x * 0.15) * 6;
        s.y = Math.tanh(s.y * 0.15) * 6;
        s.z = Math.tanh(s.z * 0.08) * 8;

        const sq = Math.sqrt(dt);
        s.bx += dt * (-0.035 * s.bx) + 0.018 * randn() * sq;
        s.by += dt * (-0.035 * s.by) + 0.018 * randn() * sq;
        s.w += dt * (-0.07 * (s.w - (0.65 + 0.25 * s.by)));
        s.w = Math.max(0.35, Math.min(1.4, s.w));
        s.phi += dt * (s.w + 0.12 * s.y);

        const E = s.x * s.x + s.y * s.y;
        /* continuous energy — soft follow, hysteresis-ish, NEVER parks at 0 */
        const eTarget =
          E > 12 ? 0.85 : E > 7 ? 0.45 : E > 3 ? 0.22 : 0.1;
        s.e += dt * (eTarget - s.e) * (E > 10 ? 1.6 : 0.7);
        s.e = Math.max(0.06, Math.min(1, s.e));

        const n = Math.hypot(s.x, s.y) || 1;
        const en = s.e;

        return {
          weave: [
            floorAbs(0.0014 * (s.x / n) + 0.0009 * Math.sin(s.phi) + 0.0005 * s.bx, 0.0007),
            floorAbs(0.0012 * (s.y / n) + 0.0007 * Math.cos(s.phi * 0.9) + 0.0004 * s.by, 0.0006),
          ],
          tilt: [
            floorAbs(0.01 * s.y * 0.08 + 0.006 * s.bx + 0.004 * en * (s.x / n), 0.004),
            floorAbs(0.01 * s.x * 0.08 + 0.006 * s.by + 0.004 * en * (s.y / n), 0.004),
          ],
          buckle: floorAbs(0.0022 * Math.tanh(s.z * 0.2) + 0.0015 * en * Math.sin(s.phi), 0.0009),
          breath: 1 + floorAbs(0.0045 * Math.sin(s.phi) + 0.002 * s.bx + 0.003 * en, 0.0015),
          wave: floorAbs(s.phi * 0.15 + s.by * 0.5, 0.05),
          energy: en,
        };
      },
    };
  }

  function mountFilmLogo(opts) {
    const canvas = opts.canvas;
    if (!canvas) return null;
    /* each mount = a new projection onto the display (never reuse a specimen) */
    const staticOnly = !!opts.staticOnly;
    const gl =
      canvas.getContext("webgl", {
        alpha: true,
        premultipliedAlpha: true,
        antialias: false,
        depth: false,
        stencil: false,
      }) ||
      canvas.getContext("experimental-webgl", {
        alpha: true,
        premultipliedAlpha: true,
      });
    if (!gl) return null;

    const vs = compile(gl, gl.VERTEX_SHADER, VERT);
    const fs = compile(gl, gl.FRAGMENT_SHADER, FRAG);
    if (!vs || !fs) return null;
    const prog = createProgram(gl, vs, fs);
    if (!prog) return null;

    const buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
      gl.STATIC_DRAW
    );

    const locPos = gl.getAttribLocation(prog, "a_pos");
    const u = {
      tex: gl.getUniformLocation(prog, "u_tex"),
      res: gl.getUniformLocation(prog, "u_res"),
      time: gl.getUniformLocation(prog, "u_time"),
      p: gl.getUniformLocation(prog, "u_p"),
      weave: gl.getUniformLocation(prog, "u_weave"),
      tilt: gl.getUniformLocation(prog, "u_tilt"),
      buckle: gl.getUniformLocation(prog, "u_buckle"),
      breath: gl.getUniformLocation(prog, "u_breath"),
      wave: gl.getUniformLocation(prog, "u_wave"),
      energy: gl.getUniformLocation(prog, "u_energy"),
      edgeFloor: gl.getUniformLocation(prog, "u_edgeFloor"),
      lamp: gl.getUniformLocation(prog, "u_lamp"),
    };

    const tex = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    const off = document.createElement("canvas");
    const ctx = off.getContext("2d");
    let lastFont = 0;
    let lastDpr = 0;
    let running = true;
    let raf = 0;
    const t0 = performance.now();
    let lastNow = t0;
    const dyn = createDynamics();

    /*
     * Specimen entropy for THIS projection onto the display.
     * Never persisted — no sessionStorage. Fresh organic thing every throw.
     */
    const instanceSeed = Math.random() * 1000;
    const edgeFloor = 0.7 + Math.random() * 0.55; /* px soft edge */
    const lampAmt = 0.055 + Math.random() * 0.04;
    const padScale = 0.9 + Math.random() * 0.25;
    const inkBlur = 0.01 + Math.random() * 0.012;

    const fontFamily =
      opts.fontFamily || '"Onyx", "Times New Roman", Times, serif';
    const text = opts.text || "f00";
    const ink = opts.ink || "#090909";

    const paintTexture = (fontPx) => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const el = opts.splashEl;
      const sb = el
        ? el.getBoundingClientRect()
        : { width: fontPx * 1.55, height: fontPx * 0.86 };
      const padX = Math.max(24, sb.width * 0.22 * padScale);
      const padY = Math.max(24, sb.height * 0.28 * padScale);
      const w = Math.ceil(sb.width + padX * 2);
      const h = Math.ceil(sb.height + padY * 2);
      off.width = Math.max(2, Math.floor(w * dpr));
      off.height = Math.max(2, Math.floor(h * dpr));
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, w, h);
      ctx.font = `400 ${fontPx}px ${fontFamily}`;
      ctx.textBaseline = "middle";
      ctx.textAlign = "center";
      ctx.fillStyle = ink;
      /* soft ink already on the plate (pre-lens) — unique bleed per throw */
      ctx.shadowColor = "rgba(9,9,9,0.35)";
      ctx.shadowBlur = Math.max(2, fontPx * inkBlur);
      ctx.fillText(text, w / 2, h / 2 + fontPx * 0.02);

      gl.bindTexture(gl.TEXTURE_2D, tex);
      gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
      gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, off);

      canvas.width = off.width;
      canvas.height = off.height;
      canvas.style.width = `${w}px`;
      canvas.style.height = `${h}px`;
      gl.viewport(0, 0, canvas.width, canvas.height);
      lastFont = fontPx;
      lastDpr = dpr;
    };

    const drawOnce = (st, now, p, op) => {
      canvas.style.opacity = String(Math.max(0, Math.min(1, op)));
      gl.useProgram(prog);
      gl.bindBuffer(gl.ARRAY_BUFFER, buf);
      gl.enableVertexAttribArray(locPos);
      gl.vertexAttribPointer(locPos, 2, gl.FLOAT, false, 0, 0);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, tex);
      gl.uniform1i(u.tex, 0);
      gl.uniform2f(u.res, canvas.width, canvas.height);
      gl.uniform1f(u.time, now);
      gl.uniform1f(u.p, p);
      gl.uniform2f(u.weave, st.weave[0], st.weave[1]);
      gl.uniform2f(u.tilt, st.tilt[0], st.tilt[1]);
      gl.uniform1f(u.buckle, st.buckle);
      gl.uniform1f(u.breath, st.breath);
      gl.uniform1f(u.wave, st.wave);
      gl.uniform1f(u.energy, st.energy);
      gl.uniform1f(u.edgeFloor, edgeFloor);
      gl.uniform1f(u.lamp, lampAmt);
      gl.enable(gl.BLEND);
      gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
      gl.clearColor(0, 0, 0, 0);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.drawArrays(gl.TRIANGLES, 0, 6);
    };

    const frame = (ts) => {
      if (!running) return;
      if (!staticOnly) raf = requestAnimationFrame(frame);
      const fontPx = opts.getFontPx();
      if (!fontPx || fontPx < 8) return;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      if (Math.abs(fontPx - lastFont) > 0.5 || dpr !== lastDpr) {
        paintTexture(fontPx);
      }

      const nowMs = ts || performance.now();
      const dt = staticOnly
        ? 0
        : Math.min(0.05, Math.max(0, (nowMs - lastNow) / 1000));
      lastNow = nowMs;
      const now = (nowMs - t0) / 1000 + instanceSeed * 0.01;

      const st = dyn.update(staticOnly ? 0.016 : dt);
      const p = opts.getP ? opts.getP() : 0;
      const op = opts.getOpacity ? opts.getOpacity() : 1;
      drawOnce(st, now, p, op);
    };

    const start = () => {
      paintTexture(opts.getFontPx() || 120);
      running = true;
      lastNow = performance.now();
      if (staticOnly) {
        /* unique soft specimen, frozen — still not a CAD clone */
        frame(lastNow);
      } else {
        raf = requestAnimationFrame(frame);
      }
    };
    if (document.fonts && document.fonts.load) {
      document.fonts
        .load(`400 120px ${fontFamily}`)
        .then(start)
        .catch(start);
    } else {
      start();
    }

    return {
      destroy() {
        running = false;
        cancelAnimationFrame(raf);
      },
      resize() {
        lastFont = 0;
      },
    };
  }

  window.F00FilmLogo = { mount: mountFilmLogo };
})();
