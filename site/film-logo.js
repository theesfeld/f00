/* f00 — WebGL Technicolor film plate for the splash mark
 * Real fragment shading: weave warp, dye-transfer CA, bloom, grain, gate.
 * Falls back silently if WebGL unavailable / reduced motion.
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

  /* Technicolor 3-strip / dye-transfer · gate weave · bloom · grain */
  const FRAG = `
    precision highp float;
    varying vec2 v_uv;
    uniform sampler2D u_tex;
    uniform vec2 u_res;
    uniform float u_time;
    uniform float u_p;
    uniform float u_intensity;
    uniform float u_flash;
    uniform float u_tear;
    uniform vec2 u_tearDir;

    float hash(vec2 p) {
      return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
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

    /* continuous film transport weave + rare gate bump — warps the plate */
    vec2 filmWarp(vec2 uv, float t, float amp) {
      float weaveX = sin(t * 1.7 + uv.y * 14.0) * 0.00115
                   + sin(t * 4.2 + uv.y * 3.0) * 0.00055;
      float weaveY = sin(t * 1.35 + uv.x * 10.0) * 0.00085
                   + sin(t * 0.55) * 0.0004;
      /* intermittent projector gate hit */
      float gateHit = step(0.992, noise(vec2(floor(t * 10.0), 7.7)));
      weaveY += gateHit * sin(t * 40.0) * 0.0035;
      /* soft vertical stretch — film stretch through gate */
      float stretch = sin(t * 0.28) * 0.004 * (uv.y - 0.5);
      vec2 c = uv - 0.5;
      float r2 = dot(c, c);
      vec2 barrel = c * r2 * 0.008;
      return uv + (vec2(weaveX, weaveY + stretch) + barrel) * amp;
    }

    vec4 samplePlate(vec2 uv) {
      if (uv.x < -0.02 || uv.x > 1.02 || uv.y < -0.02 || uv.y > 1.02) {
        return vec4(0.0);
      }
      return texture2D(u_tex, clamp(uv, 0.0, 1.0));
    }

    /* bloom on alpha-weighted ink so the field stays clean */
    vec4 bloomTap(vec2 uv, float spread) {
      vec2 px = spread / u_res;
      vec4 acc = vec4(0.0);
      float wsum = 0.0;
      for (int y = -2; y <= 2; y++) {
        for (int x = -2; x <= 2; x++) {
          float w = exp(-0.35 * float(x * x + y * y));
          vec4 s = samplePlate(uv + vec2(float(x), float(y)) * px);
          acc += s * w;
          wsum += w;
        }
      }
      return acc / wsum;
    }

    void main() {
      float hero = 1.0 - clamp(u_p, 0.0, 1.0);
      float amp = u_intensity * (0.85 + 0.65 * hero);
      float t = u_time;

      vec2 uv = filmWarp(v_uv, t, amp * 1.35);

      /* 3-strip dye-transfer misregistration */
      float tear = u_tear * amp;
      float caPx = (1.0 + 2.2 * hero + tear * 4.0) * amp;
      vec2 px = vec2(caPx, caPx * 0.4) / u_res;
      vec2 rOff = px * vec2(1.15, 0.25) + u_tearDir * tear * 0.006;
      vec2 bOff = px * vec2(-1.05, 0.2) - u_tearDir * tear * 0.005;
      vec2 gOff = px * vec2(0.12, -0.7);

      vec4 sR = samplePlate(uv + rOff);
      vec4 sG = samplePlate(uv + gOff);
      vec4 sB = samplePlate(uv + bOff);
      vec4 s0 = samplePlate(uv);

      float a = max(s0.a, max(sR.a, max(sG.a, sB.a)));
      /* luminance from each plate + dye fringing at edges */
      vec3 col = vec3(sR.r, sG.g, sB.b);
      float edge = abs(sR.a - sB.a) + abs(sR.a - sG.a);
      /* poppy / sky plate fringe (Technicolor dye, not LED RGB) */
      col += vec3(0.55, 0.12, 0.04) * sR.a * edge * 0.35 * hero * amp;
      col += vec3(0.05, 0.18, 0.55) * sB.a * edge * 0.3 * hero * amp;

      /* bloom — projection light through celluloid */
      vec4 blo = bloomTap(uv, 2.0 + 5.0 * hero * amp);
      float blum = blo.a;
      col += blo.rgb * blum * (0.28 + 0.55 * hero) * amp;
      col += vec3(0.12, 0.06, 0.02) * blum * (0.2 + 0.5 * hero) * amp;
      a = max(a, blum * (0.18 + 0.3 * hero) * amp);

      /* Technicolor grade on the ink */
      col.r = pow(max(col.r, 0.0), 0.93);
      col.g = pow(max(col.g, 0.0), 0.99);
      col.b = pow(max(col.b, 0.0), 1.05);
      col *= mix(vec3(1.0), vec3(1.08, 0.98, 0.9), 0.4 * hero);

      /* emulsion grain in the plate */
      float gn = noise(v_uv * u_res * 0.45 + vec2(t * 26.0, t * 19.0));
      col += (gn - 0.5) * (0.045 + 0.07 * hero) * amp * a;

      /* exposure hitch / flash frame */
      col += vec3(0.28, 0.16, 0.08) * u_flash * a;
      a = max(a, u_flash * 0.15 * s0.a);

      /* sparse gate dust */
      float dirt = step(0.9982, noise(vec2(v_uv.y * 90.0, floor(t * 9.0))));
      col *= 1.0 - dirt * 0.45 * a * amp;

      a = clamp(a, 0.0, 1.0);
      /* MUST premultiply — otherwise transparent pixels paint a black slab */
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

  /**
   * @param {object} opts
   * @param {HTMLCanvasElement} opts.canvas
   * @param {HTMLElement} opts.splashEl  DOM mark used for layout size
   * @param {() => number} opts.getFontPx
   * @param {() => number} opts.getP
   * @param {() => number} opts.getOpacity  dock opacity
   * @param {string} [opts.fontFamily]
   * @param {string} [opts.text]
   * @param {string} [opts.ink]
   */
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
      intensity: gl.getUniformLocation(prog, "u_intensity"),
      flash: gl.getUniformLocation(prog, "u_flash"),
      tear: gl.getUniformLocation(prog, "u_tear"),
      tearDir: gl.getUniformLocation(prog, "u_tearDir"),
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
    let flash = 0;
    let tear = 0;
    let tearDir = [1, 0];
    let nextEvent = 0.6;
    let running = true;
    let raf = 0;
    const t0 = performance.now();

    const fontFamily =
      opts.fontFamily ||
      '"Onyx", "Times New Roman", Times, serif';
    const text = opts.text || "f00";
    const ink = opts.ink || "#090909";

    const paintTexture = (fontPx) => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const el = opts.splashEl;
      const sb = el
        ? el.getBoundingClientRect()
        : { width: fontPx * 1.55, height: fontPx * 0.86 };
      /* pad around the layout box for bloom / CA / warp (not a huge rect) */
      const padX = Math.max(12, sb.width * 0.12);
      const padY = Math.max(12, sb.height * 0.16);
      const w = Math.ceil(sb.width + padX * 2);
      const h = Math.ceil(sb.height + padY * 2);
      off.width = Math.max(2, Math.floor(w * dpr));
      off.height = Math.max(2, Math.floor(h * dpr));
      /* width reset clears state */
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      ctx.clearRect(0, 0, w, h);
      ctx.font = `400 ${fontPx}px ${fontFamily}`;
      ctx.textBaseline = "middle";
      ctx.textAlign = "center";
      ctx.fillStyle = ink;
      /* soft ink so bloom has emulsion to catch */
      ctx.shadowColor = "rgba(9,9,9,0.4)";
      ctx.shadowBlur = Math.max(2, fontPx * 0.016);
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

    const scheduleEvents = (now) => {
      if (now < nextEvent) return;
      const p = opts.getP();
      const hero = 1 - p;
      /* film events denser while the mark is large */
      if (Math.random() < 0.62 + hero * 0.2) {
        flash = 0.4 + Math.random() * 0.75 * (0.45 + hero);
      }
      if (Math.random() < 0.5 + hero * 0.3) {
        tear = 0.65 + Math.random() * 1.1;
        const a = Math.random() * Math.PI * 2;
        tearDir = [Math.cos(a), Math.sin(a)];
      }
      nextEvent = now + 0.55 + Math.random() * (1.1 + p * 2.0);
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
      scheduleEvents(now);
      flash *= 0.88;
      tear *= 0.91;
      if (flash < 0.01) flash = 0;
      if (tear < 0.02) tear = 0;

      const p = opts.getP();
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
      gl.uniform1f(u.intensity, 1.0);
      gl.uniform1f(u.flash, flash);
      gl.uniform1f(u.tear, tear);
      gl.uniform2f(u.tearDir, tearDir[0], tearDir[1]);

      gl.enable(gl.BLEND);
      gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
      gl.clearColor(0, 0, 0, 0);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.drawArrays(gl.TRIANGLES, 0, 6);
    };

    /* wait for font then start */
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
