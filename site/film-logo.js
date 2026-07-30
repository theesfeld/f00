/* f00 — WebGL splash: flat black ink + warp hits + soft bloom
 * No grain, no RGB fringe, no “3D” plates — just the letterforms warping.
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
    uniform float u_hit;      /* 0..1 quick warp event */
    uniform vec2 u_hitAxis;   /* stretch axes for the hit */
    uniform float u_hitSkew;

    float hash(vec2 p) {
      return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
    }

    vec4 sampleInk(vec2 uv) {
      if (uv.x < -0.05 || uv.x > 1.05 || uv.y < -0.05 || uv.y > 1.05) {
        return vec4(0.0);
      }
      return texture2D(u_tex, clamp(uv, 0.0, 1.0));
    }

    /* soft alpha bloom only — keeps ink flat black, no color fringe */
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

    vec2 warpUV(vec2 uv, float t, float hit, vec2 axis, float skew) {
      vec2 c = uv - 0.5;

      /* tiny always-on idle (barely there — film resting in the gate) */
      float idle = 0.55 + 0.45 * (1.0 - clamp(u_p, 0.0, 1.0));
      c.x += sin(t * 1.1 + uv.y * 5.5) * 0.0009 * idle;
      c.y += sin(t * 0.85 + uv.x * 4.5) * 0.0007 * idle;

      /* BIG quick hit: stretch / squash / skew — decays via u_hit */
      float h = hit * hit; /* ease-out shape from host decay */
      c.x *= 1.0 + axis.x * h * 0.22;
      c.y *= 1.0 + axis.y * h * 0.28;
      c.x += c.y * skew * h * 0.18;
      c.y += c.x * skew * h * 0.06;
      /* short vertical yank */
      c.y += sin(h * 3.14159) * axis.y * 0.012;

      return c + 0.5;
    }

    void main() {
      float t = u_time;
      vec2 uv = warpUV(v_uv, t, u_hit, u_hitAxis, u_hitSkew);

      vec4 s = sampleInk(uv);
      float a = s.a;

      /* soft bloom of the ink (still black — no colored glow) */
      float hero = 1.0 - clamp(u_p, 0.0, 1.0);
      float ba = bloomA(uv, 1.4 + 2.2 * hero);
      a = max(a, ba * (0.12 + 0.1 * hero));

      /* flat near-black ink — no grade, no grain, no RGB split */
      vec3 col = vec3(0.04, 0.04, 0.04);
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
      hit: gl.getUniformLocation(prog, "u_hit"),
      hitAxis: gl.getUniformLocation(prog, "u_hitAxis"),
      hitSkew: gl.getUniformLocation(prog, "u_hitSkew"),
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
    let hit = 0;
    let hitAxis = [0, 1];
    let hitSkew = 0;
    let nextHit = 0.8 + Math.random() * 1.2;
    let running = true;
    let raf = 0;
    const t0 = performance.now();

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
      const padX = Math.max(12, sb.width * 0.14);
      const padY = Math.max(12, sb.height * 0.2);
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
      ctx.shadowColor = "rgba(9,9,9,0.25)";
      ctx.shadowBlur = Math.max(1, fontPx * 0.01);
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

    const fireHit = () => {
      /* random stretch axis + skew — big, short */
      const mode = Math.floor(Math.random() * 4);
      if (mode === 0) hitAxis = [0, 1.15]; /* vertical stretch */
      else if (mode === 1) hitAxis = [0, -0.95]; /* vertical squash */
      else if (mode === 2) hitAxis = [1.05, 0.25]; /* wide */
      else hitAxis = [-0.35, 0.9]; /* tall skew-ish */
      hitSkew = (Math.random() < 0.5 ? -1 : 1) * (0.4 + Math.random() * 1.1);
      hit = 1.0;
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
      if (now >= nextHit) {
        fireHit();
        const p = opts.getP ? opts.getP() : 0;
        /* denser hits when logo is large; still occasional when small */
        nextHit = now + 0.7 + Math.random() * (1.4 + p * 2.2);
      }
      /* super quick decay — snap stretch, not a long goo */
      hit *= 0.78;
      if (hit < 0.02) hit = 0;

      const op = opts.getOpacity ? opts.getOpacity() : 1;
      canvas.style.opacity = String(Math.max(0, Math.min(1, op)));

      const p = opts.getP ? opts.getP() : 0;

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
      gl.uniform1f(u.hit, hit);
      gl.uniform2f(u.hitAxis, hitAxis[0], hitAxis[1]);
      gl.uniform1f(u.hitSkew, hitSkew);

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
