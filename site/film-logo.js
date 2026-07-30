/* f00 — WebGL splash: flat black ink on a bending film cel
 * Organic continuous warp (paint on acetate) + soft bloom.
 * No grain, no RGB fringe — analog twist / stretch / bend only.
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
    /* swell envelope 0..1 (smooth in host); organic bend event */
    uniform float u_swell;
    uniform vec2 u_pivot;
    uniform float u_twist;   /* radians-ish at peak */
    uniform vec2 u_pull;     /* stretch direction bias */
    uniform float u_wave;    /* secondary fold strength */

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
    /* smooth multi-octave field — wet-cel / heat shimmer continuity */
    float fbm(vec2 p) {
      float v = 0.0;
      float a = 0.5;
      for (int i = 0; i < 4; i++) {
        v += a * noise(p);
        p = p * 2.03 + vec2(1.7, 9.2);
        a *= 0.5;
      }
      return v;
    }

    vec4 sampleInk(vec2 uv) {
      if (uv.x < -0.08 || uv.x > 1.08 || uv.y < -0.08 || uv.y > 1.08) {
        return vec4(0.0);
      }
      return texture2D(u_tex, clamp(uv, 0.0, 1.0));
    }

    float bloomA(vec2 uv, float spread) {
      vec2 px = spread / u_res;
      float acc = 0.0;
      float wsum = 0.0;
      for (int y = -2; y <= 2; y++) {
        for (int x = -2; x <= 2; x++) {
          float w = exp(-0.4 * float(x * x + y * y));
          acc += sampleInk(uv + vec2(float(x), float(y)) * px).a * w;
          wsum += w;
        }
      }
      return acc / wsum;
    }

    /*
     * Cel warp: paint stuck to acetate.
     * Continuous low-frequency bend + a slow swell that twists/pulls
     * around a moving pivot (not a uniform scale snap).
     */
    vec2 celWarp(vec2 uv, float t) {
      float hero = 1.0 - clamp(u_p, 0.0, 1.0);
      float live = 0.5 + 0.5 * hero;

      /* continuous organic field — always breathing */
      vec2 flow = vec2(t * 0.11, t * 0.09);
      float n1 = fbm(uv * 2.4 + flow);
      float n2 = fbm(uv * 3.1 - flow.yx * 1.3 + 4.0);
      float n3 = fbm(uv * 1.2 + flow * 0.4 + 11.0);

      vec2 drift = vec2(
        (n1 - 0.5) * 0.014 + (n3 - 0.5) * 0.006,
        (n2 - 0.5) * 0.012 + (n3 - 0.5) * 0.005
      ) * live;

      /* slow S-curve bend along the word (acetate curl) */
      float curl = sin(uv.x * 3.14159 + t * 0.35) * cos(uv.y * 2.2 - t * 0.22);
      drift.y += curl * 0.006 * live;
      drift.x += sin(uv.y * 2.8 + t * 0.28) * 0.004 * live;

      /* domain-warped second pass — more fluid, less “sine wave” */
      vec2 warped = uv + drift;
      float m1 = fbm(warped * 2.8 + flow * 1.4);
      float m2 = fbm(warped * 2.2 - flow * 1.1 + 2.5);
      drift += vec2(m1 - 0.5, m2 - 0.5) * 0.008 * live;

      vec2 p = uv + drift;

      /* swell event: twist + radial pull around pivot (eased envelope) */
      float s = u_swell;
      /* smoothstep-ish already from host; reinforce ease */
      s = s * s * (3.0 - 2.0 * s);
      if (s > 0.001) {
        vec2 piv = u_pivot;
        vec2 d = p - piv;
        float r = length(d);
        float fall = exp(-r * r * 3.2); /* soft localized bend */

        /* twist: rotate around pivot, stronger near center, fades out */
        float ang = u_twist * s * fall;
        float ca = cos(ang);
        float sa = sin(ang);
        d = vec2(ca * d.x - sa * d.y, sa * d.x + ca * d.y);

        /* stretch along pull axis — fluid, not a box scale */
        vec2 axis = normalize(u_pull + vec2(0.0001));
        vec2 ortho = vec2(-axis.y, axis.x);
        float along = dot(d, axis);
        float across = dot(d, ortho);
        along *= 1.0 + 0.2 * s * fall;
        across *= 1.0 - 0.12 * s * fall;
        d = axis * along + ortho * across;

        /* secondary fold wave through the cel */
        float fold = sin(along * 9.0 + t * 1.4 + u_wave * 3.0) * 0.012 * s * fall * u_wave;
        d += ortho * fold;

        /* gentle whole-plate shear */
        d.x += d.y * 0.08 * s * u_twist;

        p = piv + d;
      }

      return p;
    }

    void main() {
      vec2 uv = celWarp(v_uv, u_time);

      vec4 ink = sampleInk(uv);
      float a = ink.a;

      float hero = 1.0 - clamp(u_p, 0.0, 1.0);
      float ba = bloomA(uv, 1.5 + 2.0 * hero);
      a = max(a, ba * (0.1 + 0.08 * hero));

      /* flat black only */
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

  /** Smooth 0→1→0 envelope over duration (organic swell, not snap). */
  function swellEnvelope(phase) {
    /* phase 0..1 through the event */
    if (phase <= 0 || phase >= 1) return 0;
    /* ease in-out sine */
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
      swell: gl.getUniformLocation(prog, "u_swell"),
      pivot: gl.getUniformLocation(prog, "u_pivot"),
      twist: gl.getUniformLocation(prog, "u_twist"),
      pull: gl.getUniformLocation(prog, "u_pull"),
      wave: gl.getUniformLocation(prog, "u_wave"),
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

    /* active swell event (fluid envelope) */
    let ev = null;
    let nextEv = 1.2 + Math.random() * 1.5;

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
      /* extra pad so bent/twisted ink stays inside the canvas */
      const padX = Math.max(16, sb.width * 0.18);
      const padY = Math.max(16, sb.height * 0.24);
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
      ctx.shadowColor = "rgba(9,9,9,0.2)";
      ctx.shadowBlur = Math.max(1, fontPx * 0.008);
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

    const startSwell = (now) => {
      const ang = Math.random() * Math.PI * 2;
      ev = {
        t0: now,
        /* longer, slower — acetate twisting in the hand / heat */
        dur: 0.85 + Math.random() * 1.15,
        pivot: [
          0.35 + Math.random() * 0.3,
          0.35 + Math.random() * 0.3,
        ],
        twist: (Math.random() < 0.5 ? -1 : 1) * (0.12 + Math.random() * 0.28),
        pull: [Math.cos(ang), Math.sin(ang)],
        wave: 0.4 + Math.random() * 1.1,
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

      if (!ev && now >= nextEv) {
        startSwell(now);
      }

      let swell = 0;
      let pivot = [0.5, 0.5];
      let twist = 0;
      let pull = [1, 0];
      let wave = 0;
      if (ev) {
        const phase = (now - ev.t0) / ev.dur;
        if (phase >= 1) {
          ev = null;
          nextEv = now + 1.1 + Math.random() * (1.8 + p * 2.5);
        } else {
          swell = swellEnvelope(phase);
          pivot = ev.pivot;
          twist = ev.twist;
          pull = ev.pull;
          wave = ev.wave;
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
      gl.uniform1f(u.swell, swell);
      gl.uniform2f(u.pivot, pivot[0], pivot[1]);
      gl.uniform1f(u.twist, twist);
      gl.uniform2f(u.pull, pull[0], pull[1]);
      gl.uniform1f(u.wave, wave);

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
