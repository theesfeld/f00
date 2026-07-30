/* f00 — WebGL film PROJECTOR for the splash plate
 *
 * Dream: this is not a font. It is a 70mm frame — dye on film — thrown by a
 * lamp through a lens onto a screen. Geometry changes because the PLATE moves
 * relative to lamp/lens (weave, buckle, tilt, throw), not because glyphs skew.
 *
 * MUST: gate weave · plate tilt (homography) · buckle Z→scale · always-on wave
 * SHOULD: focus breathing (blur+scale) · soft lamp falloff on alpha
 * NEVER: RGB fringe · grain · per-glyph morph · snap stretch
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
    /* register event — same physical channels, amplified (0..1 sine envelope) */
    uniform float u_event;
    uniform vec2 u_evWeave;   /* extra gate slip direction */
    uniform vec2 u_evTilt;    /* extra pitch/yaw */
    uniform vec2 u_evPivot;   /* buckle / slip focus in UV */
    uniform float u_evBuckle;

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
    /* low-frequency only — optical scale, not texture grain */
    float fbm2(vec2 p) {
      return 0.66 * noise(p) + 0.34 * noise(p * 1.97 + 3.1);
    }
    vec2 n2(float t, float s) {
      return vec2(
        fbm2(vec2(t * 0.17 + s, t * 0.13 + s * 2.1)) * 2.0 - 1.0,
        fbm2(vec2(t * 0.11 + s * 3.3, t * 0.19 + 1.7)) * 2.0 - 1.0
      );
    }

    vec4 sampleInk(vec2 uv) {
      if (uv.x < -0.06 || uv.x > 1.06 || uv.y < -0.06 || uv.y > 1.06) {
        return vec4(0.0);
      }
      return texture2D(u_tex, clamp(uv, 0.0, 1.0));
    }

    /* soft alpha bloom = focus softness of the thrown silhouette */
    float bloomA(vec2 uv, float spreadPx) {
      vec2 px = spreadPx / u_res;
      float acc = 0.0;
      float wsum = 0.0;
      for (int y = -2; y <= 2; y++) {
        for (int x = -2; x <= 2; x++) {
          float w = exp(-0.45 * float(x * x + y * y));
          acc += sampleInk(uv + vec2(float(x), float(y)) * px).a * w;
          wsum += w;
        }
      }
      return acc / wsum;
    }

    /*
     * Inverse optical path: screen UV → film UV.
     * Whole-frame geometry only. Flat black plate.
     */
    vec2 projectUV(vec2 uv, float t, out float defocus) {
      float hero = 1.0 - clamp(u_p, 0.0, 1.0);
      float live = 0.55 + 0.45 * hero;
      float e = u_event;
      e = e * e * (3.0 - 2.0 * e);

      vec2 p = uv - 0.5;

      /* 1) Gate weave — rigid plate slide in the aperture (always) */
      vec2 weave = n2(t * 0.85, 1.0) * (0.0016 * live);
      weave += u_evWeave * e * 0.0045;
      p -= weave;

      /* 2) Film buckle Z — bow toward lamp: center magnifies (throw change) */
      float zIdle = (fbm2(uv * 1.1 + vec2(t * 0.07, t * 0.05)) - 0.5) * 0.012 * live;
      /* continuous micro-wave on the plate (always slight) */
      zIdle += sin(uv.x * 4.2 + t * 0.55) * cos(uv.y * 3.3 - t * 0.42) * 0.0035 * live;
      float r2p = dot(p, p);
      float zEv = 0.0;
      if (e > 0.001) {
        vec2 d = uv - u_evPivot;
        zEv = u_evBuckle * e * exp(-dot(d, d) * 2.8);
      }
      float Z = zIdle + zEv;
      /* perspective-ish local scale from plate distance */
      float mag = 1.0 / max(1.0 - 2.4 * Z, 0.82);
      p *= mag;

      /* 3) Plate tilt → soft keystone (whole projected frame) */
      vec2 tilt = n2(t * 0.22, 2.0) * (0.012 * live);
      tilt += u_evTilt * e * 0.035;
      float denom = 1.0 + tilt.y * p.y + tilt.x * p.x;
      p /= max(denom, 0.78);

      /* 4) Throw / focus breathing — global scale of the throw */
      float breath = 1.0 + 0.006 * live * sin(t * 0.75 + fbm2(vec2(t * 0.2, 4.0)) * 2.0);
      breath += e * 0.012 * sin(e * 3.14159);
      p /= breath;

      /* 5) Screen / air undulation AFTER projection (soft, always-on wave) */
      vec2 air = vec2(
        sin(p.y * 5.5 + t * 0.38) * 0.0018,
        cos(p.x * 4.8 - t * 0.33) * 0.0015
      ) * live;
      air += n2(t * 0.12 + p.x + p.y, 4.0) * (0.0011 * live);
      air += e * 0.003 * n2(t + 9.0, 5.0);
      p += air;

      /* defocus amount for bloom (optics, not grain) */
      defocus = abs(Z) * 40.0 + length(tilt) * 8.0 + abs(breath - 1.0) * 25.0 + e * 0.8;

      return p + 0.5;
    }

    void main() {
      float defocus = 0.0;
      vec2 uv = projectUV(v_uv, u_time, defocus);

      float a = sampleInk(uv).a;

      /* focus breathing — soft silhouette, still black */
      float spread = 0.35 + defocus * 1.1;
      float ba = bloomA(uv, spread);
      a = max(a, ba * (0.08 + 0.1 * clamp(defocus, 0.0, 1.5)));

      /* soft lamp falloff on the thrown plate (alpha only) */
      vec2 q = v_uv - 0.5;
      float fall = 1.0 - 0.05 * dot(q, q) * 4.0;
      a *= clamp(fall, 0.9, 1.0);

      vec3 col = vec3(0.04);
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

  function swellEnvelope(phase) {
    if (phase <= 0 || phase >= 1) return 0;
    return Math.sin(phase * Math.PI);
  }

  function mountFilmLogo(opts) {
    const canvas = opts.canvas;
    if (!canvas) return null;
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
      event: gl.getUniformLocation(prog, "u_event"),
      evWeave: gl.getUniformLocation(prog, "u_evWeave"),
      evTilt: gl.getUniformLocation(prog, "u_evTilt"),
      evPivot: gl.getUniformLocation(prog, "u_evPivot"),
      evBuckle: gl.getUniformLocation(prog, "u_evBuckle"),
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

    let ev = null;
    let nextEv = 1.4 + Math.random() * 1.8;

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
      /* pad for keystone / weave so plate edges don't clip the throw */
      const padX = Math.max(20, sb.width * 0.2);
      const padY = Math.max(20, sb.height * 0.26);
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
      ctx.shadowColor = "rgba(9,9,9,0.18)";
      ctx.shadowBlur = Math.max(1, fontPx * 0.006);
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

    const startEvent = (now) => {
      const ang = Math.random() * Math.PI * 2;
      ev = {
        t0: now,
        /* long organic register slip — not a snap */
        dur: 1.1 + Math.random() * 1.3,
        weave: [Math.cos(ang) * (0.6 + Math.random() * 0.8), Math.sin(ang) * (0.4 + Math.random() * 0.6)],
        tilt: [
          (Math.random() < 0.5 ? -1 : 1) * (0.4 + Math.random() * 0.9),
          (Math.random() < 0.5 ? -1 : 1) * (0.4 + Math.random() * 0.9),
        ],
        pivot: [0.32 + Math.random() * 0.36, 0.32 + Math.random() * 0.36],
        buckle: (Math.random() < 0.5 ? -1 : 1) * (0.012 + Math.random() * 0.02),
      };
    };

    const frame = () => {
      if (!running) return;
      raf = requestAnimationFrame(frame);
      const fontPx = opts.getFontPx();
      if (!fontPx || fontPx < 8) return;
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      if (Math.abs(fontPx - lastFont) > 0.5 || dpr !== lastDpr) {
        paintTexture(fontPx);
      }

      const now = (performance.now() - t0) / 1000;
      const p = opts.getP ? opts.getP() : 0;

      if (!ev && now >= nextEv) startEvent(now);

      let eventAmt = 0;
      let evWeave = [0, 0];
      let evTilt = [0, 0];
      let evPivot = [0.5, 0.5];
      let evBuckle = 0;
      if (ev) {
        const phase = (now - ev.t0) / ev.dur;
        if (phase >= 1) {
          ev = null;
          nextEv = now + 1.6 + Math.random() * (2.4 + p * 2.8);
        } else {
          eventAmt = swellEnvelope(phase);
          evWeave = ev.weave;
          evTilt = ev.tilt;
          evPivot = ev.pivot;
          evBuckle = ev.buckle;
        }
      }

      const op = opts.getOpacity ? opts.getOpacity() : 1;
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
      gl.uniform1f(u.event, eventAmt);
      gl.uniform2f(u.evWeave, evWeave[0], evWeave[1]);
      gl.uniform2f(u.evTilt, evTilt[0], evTilt[1]);
      gl.uniform2f(u.evPivot, evPivot[0], evPivot[1]);
      gl.uniform1f(u.evBuckle, evBuckle);

      gl.enable(gl.BLEND);
      gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
      gl.clearColor(0, 0, 0, 0);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.drawArrays(gl.TRIANGLES, 0, 6);
    };

    const start = () => {
      paintTexture(opts.getFontPx() || 120);
      running = true;
      raf = requestAnimationFrame(frame);
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
