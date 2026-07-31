/**
 * f00 shared chrome SSOT — collective header + footer.
 *
 * Live: https://f00.sh/theme/f00-chrome.js
 * Mount: elements with data-f00-chrome="header"|"footer"
 * Local attrs: data-page, data-mode (hub|project), data-repo, data-project, data-left
 *
 * Pure render functions are Node-testable (no DOM required).
 */
(function (root, factory) {
  const api = factory();
  if (typeof module !== "undefined" && module.exports) {
    module.exports = api;
  }
  if (root) {
    root.f00Chrome = api;
  }
  if (typeof document !== "undefined") {
    const run = () => {
      try {
        api.bootstrap(document);
        api.watchSpa(document);
      } catch (e) {
        /* never break the page */
        if (typeof console !== "undefined" && console.warn) {
          console.warn("[f00-chrome]", e);
        }
      }
    };
    if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", run, { once: true });
    } else {
      run();
    }
  }
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  const HUB = "https://f00.sh";
  const ORG_GITHUB = "https://github.com/f00-sh";
  const CONTACT_EMAIL = "tj@f00.sh";
  const CONTACT_MAILTO = "mailto:tj@f00.sh";
  const CONTACT_PHONE_DISPLAY = "USA (973) 382-9681";
  const CONTACT_TEL = "tel:+19733829681";
  const CHROME_HREF = "https://f00.sh/theme/f00-chrome.js?v=4";
  const CHROME_MARKER = "data-f00-chrome-script";

  /** hostname → project chrome (from catalog domains; Worker has no catalog at parse time) */
  const DOMAIN_PROJECTS = {
    "coreutils.f00.sh": { project: "f00tils", repo: "f00-sh/f00tils" },
    "clun.f00.sh": { project: "clun", repo: "f00-sh/clun" },
    "cel.f00.sh": { project: "cel", repo: "f00-sh/cel" },
    "trn.f00.sh": { project: "trn", repo: "f00-sh/trn" },
    "heartbox.f00.sh": { project: "heartbox", repo: "f00-sh/heartbox" },
    "joule.f00.sh": { project: "joule", repo: "f00-sh/joule" },
    "throw.f00.sh": { project: "throw", repo: "f00-sh/throw" },
  };

  const PAGE_LABELS = {
    index: null,
    home: null,
    who: "who",
    constitution: "constitution",
    plan: "plan",
    donate: "give",
    ledger: "ledger",
  };

  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function normalizeConfig(raw) {
    const c = raw && typeof raw === "object" ? raw : {};
    const page = String(c.page || "index").toLowerCase();
    const mode = c.mode === "project" ? "project" : "hub";
    const project = c.project ? String(c.project) : "";
    const repo = c.repo ? String(c.repo) : "";
    const left = c.left != null && c.left !== "" ? String(c.left) : null;
    return {
      page,
      mode,
      project,
      repo,
      left,
      home: c.home || HUB + "/",
      brandMark: c.brandMark || "f00",
      brandSub:
        c.brandSub != null
          ? String(c.brandSub)
          : project
            ? "/" + project.replace(/^\//, "")
            : ".sh",
    };
  }

  function configFromAttrs(attrs) {
    const get = (k) => {
      if (!attrs) return null;
      if (typeof attrs.getAttribute === "function") {
        return attrs.getAttribute(k);
      }
      return attrs[k] != null ? attrs[k] : null;
    };
    return normalizeConfig({
      page: get("data-page") || get("page"),
      mode: get("data-mode") || get("mode"),
      project: get("data-project") || get("project"),
      repo: get("data-repo") || get("repo"),
      left: get("data-left") || get("left"),
      brandSub: get("data-brand-sub") || get("brandSub"),
      brandMark: get("data-brand-mark") || get("brandMark"),
      home: get("data-home") || get("home"),
    });
  }

  function hrefFor(path, cfg) {
    // Absolute on project sites so chrome always points at the hub collective.
    if (cfg.mode === "project") {
      if (path.startsWith("http") || path.startsWith("#")) return path;
      return HUB + path;
    }
    return path;
  }

  function isCurrent(page, name) {
    return page === name;
  }

  function currentAttr(page, name) {
    return isCurrent(page, name) ? ' aria-current="page"' : "";
  }

  function sep() {
    return '<span class="foot-sep" aria-hidden="true">·</span>';
  }

  /**
   * Collective primary nav: who / constitution / plan / donate
   * plus hub conveniences projects + github (not product-specific nav).
   */
  function renderNav(cfg) {
    // Only the hub index may use an in-page #projects hash.
    // Project sites (and any non-index hub page) must point at the hub grid.
    const proj =
      cfg.mode === "hub" && (cfg.page === "index" || cfg.page === "home")
        ? "#projects"
        : HUB + "/#projects";

    const who = hrefFor("/who", cfg);
    const constitution = hrefFor("/constitution", cfg);
    const plan = hrefFor("/plan", cfg);
    const donate = hrefFor("/donate", cfg);

    return (
      '<nav class="nav" aria-label="Primary">' +
      `<a href="${esc(proj)}">projects</a>` +
      `<a href="${esc(who)}"${currentAttr(cfg.page, "who")}>who</a>` +
      `<a href="${esc(constitution)}"${currentAttr(cfg.page, "constitution")}>constitution</a>` +
      `<a href="${esc(plan)}"${currentAttr(cfg.page, "plan")}>plan</a>` +
      `<a class="nav-donate" data-f00-donate href="${esc(donate)}"${currentAttr(cfg.page, "donate")}>donate</a>` +
      `<a href="${esc(ORG_GITHUB)}" rel="noopener">github</a>` +
      "</nav>"
    );
  }

  function renderHeader(raw) {
    const cfg = normalizeConfig(raw);
    const home = cfg.home || HUB + "/";
    return (
      '<header class="top" data-f00-chrome-mounted="header">' +
      '<div class="top-inner">' +
      `<a class="brand" href="${esc(home)}" aria-label="f00 home">` +
      `<span class="brand-mark">${esc(cfg.brandMark)}</span>` +
      `<span class="brand-sub">${esc(cfg.brandSub)}</span>` +
      "</a>" +
      renderNav(cfg) +
      "</div>" +
      "</header>"
    );
  }

  function defaultFootLeft(cfg) {
    if (cfg.left != null) return cfg.left;
    if (cfg.mode === "project") {
      const label = cfg.project || cfg.page || "project";
      return `<a href="${esc(HUB + "/")}">← f00</a> · ${esc(label)}`;
    }
    if (cfg.page === "index" || cfg.page === "home" || !cfg.page) {
      // f00 ships no copyright claims on the hub or products
      return "f00 · collective · for love · MIT";
    }
    const label = PAGE_LABELS[cfg.page] != null ? PAGE_LABELS[cfg.page] : cfg.page;
    return `<a href="${esc(HUB + "/")}">← f00</a> · ${esc(label)}`;
  }

  function renderStars(cfg) {
    if (cfg.mode === "project" && cfg.repo) {
      const gh = "https://github.com/" + cfg.repo;
      return (
        `<a class="foot-stars" href="${esc(gh)}" rel="noopener" data-f00-stars data-repo="${esc(cfg.repo)}">★ …</a>`
      );
    }
    return (
      `<a class="foot-stars" href="${esc(ORG_GITHUB)}" rel="noopener" data-f00-stars-total>★ …</a>`
    );
  }

  function renderContact() {
    return (
      '<p class="foot-contact">' +
      `<a href="${CONTACT_MAILTO}">${esc(CONTACT_EMAIL)}</a>` +
      sep() +
      `<a href="${CONTACT_TEL}">${esc(CONTACT_PHONE_DISPLAY)}</a>` +
      "</p>"
    );
  }

  function renderFootDocs(cfg) {
    const who = hrefFor("/who", cfg);
    const constitution = hrefFor("/constitution", cfg);
    const plan = hrefFor("/plan", cfg);
    return (
      '<nav class="foot-center" aria-label="Collective docs">' +
      `<a href="${esc(who)}"${currentAttr(cfg.page, "who")}>who</a>` +
      sep() +
      `<a href="${esc(constitution)}"${currentAttr(cfg.page, "constitution")}>constitution</a>` +
      sep() +
      `<a href="${esc(plan)}"${currentAttr(cfg.page, "plan")}>plan</a>` +
      "</nav>"
    );
  }

  function renderFooter(raw) {
    const cfg = normalizeConfig(raw);
    // Explicit left is plain text (escaped). Otherwise structured default with hub link.
    const footLeftInner =
      raw &&
      typeof raw === "object" &&
      Object.prototype.hasOwnProperty.call(raw, "left") &&
      raw.left != null
        ? esc(String(raw.left))
        : defaultFootLeft({ ...cfg, left: null });

    return (
      '<footer class="foot" data-f00-chrome-mounted="footer">' +
      `<span class="foot-left">${footLeftInner}</span>` +
      renderFootDocs(cfg) +
      renderStars(cfg) +
      renderContact() +
      "</footer>"
    );
  }

  function mountEl(el) {
    if (!el || !el.getAttribute) return;
    const which = (el.getAttribute("data-f00-chrome") || "").toLowerCase();
    if (!which || which === "0" || which === "false") return;
    const cfg = configFromAttrs(el);
    let html = "";
    if (which === "header") html = renderHeader(cfg);
    else if (which === "footer") html = renderFooter(cfg);
    else if (which === "both") html = renderHeader(cfg) + renderFooter(cfg);
    else return;
    el.outerHTML = html;
  }

  function mountAll(doc) {
    const root = doc || (typeof document !== "undefined" ? document : null);
    if (!root || !root.querySelectorAll) return;
    const nodes = root.querySelectorAll("[data-f00-chrome]");
    // snapshot — outerHTML replaces nodes
    Array.prototype.slice.call(nodes).forEach(mountEl);
  }

  /**
   * Infer hub vs project chrome from hostname + path (pure; pass a location-like object).
   * @param {{ hostname?: string, pathname?: string }} loc
   */
  function inferConfigFromLocation(loc) {
    const host = String((loc && loc.hostname) || "")
      .toLowerCase()
      .replace(/\.$/, "");
    const path = String((loc && loc.pathname) || "/");
    const hubHosts = { "f00.sh": 1, "www.f00.sh": 1 };

    if (hubHosts[host] || host === "") {
      const seg = path.replace(/\/+$/, "").split("/").filter(Boolean)[0] || "index";
      const page = PAGE_LABELS[seg] !== undefined || seg === "index" ? seg : "index";
      // donate page key is donate; ledger etc.
      const pageKey =
        seg === "" || seg === "index.html" ? "index" : seg.replace(/\.html$/, "");
      return normalizeConfig({ page: pageKey || "index", mode: "hub" });
    }

    const mapped = DOMAIN_PROJECTS[host];
    if (mapped) {
      return normalizeConfig({
        mode: "project",
        project: mapped.project,
        repo: mapped.repo,
        page: "index",
      });
    }

    // *.f00.sh unknown project — still project mode, label = subdomain
    if (host.endsWith(".f00.sh")) {
      const sub = host.slice(0, -".f00.sh".length);
      return normalizeConfig({
        mode: "project",
        project: sub,
        repo: sub ? "f00-sh/" + sub : "",
        page: "index",
      });
    }

    return normalizeConfig({ mode: "hub", page: "index" });
  }

  /**
   * Pure HTML helper: rewrite first footer to shared chrome footer (tests + offline).
   * Strips copyright-style left text by full replacement.
   */
  function ensureContactHtml(html) {
    if (typeof html !== "string") return html;
    if (/data-f00-chrome-mounted=["']footer["']/.test(html)) return html;
    const cfg = { mode: "project", project: "project", repo: "" };
    const foot = renderFooter(cfg);
    if (/<footer[\s>]/i.test(html)) {
      return html.replace(/<footer\b[^>]*>[\s\S]*?<\/footer>/i, foot);
    }
    return html + foot;
  }

  function isInsideContent(el) {
    if (!el || !el.closest) return false;
    return !!el.closest(
      "main, article, [role='main'], .mantra-body, .hero-copy, .panel, .step-panel"
    );
  }

  function isSectionHeader(el) {
    if (!el || !el.classList) return false;
    return (
      el.classList.contains("section-intro") ||
      el.classList.contains("panel-head") ||
      el.classList.contains("mantra-head") ||
      el.classList.contains("section-head")
    );
  }

  /** Site chrome headers only — never section titles inside main. */
  function findSiteHeaders(root) {
    const all = Array.prototype.slice.call(
      root.querySelectorAll("header, .site-header")
    );
    return all.filter(function (el) {
      if (el.getAttribute && el.getAttribute("data-f00-chrome-mounted") === "header") {
        return false;
      }
      if (isSectionHeader(el)) return false;
      if (isInsideContent(el)) return false;
      if (el.classList && el.classList.contains("site-header")) return true;
      if (el.classList && el.classList.contains("top")) return true;
      if (el.tagName === "HEADER") return true;
      return false;
    });
  }

  function findSiteFooters(root) {
    const all = Array.prototype.slice.call(
      root.querySelectorAll("footer, .site-footer")
    );
    return all.filter(function (el) {
      if (el.getAttribute && el.getAttribute("data-f00-chrome-mounted") === "footer") {
        return false;
      }
      // site footers always win — even when authors nest them under <main>
      if (el.closest && el.closest("article .card, article.card")) return false;
      // drop nested footer inside footer
      return !all.some(function (other) {
        return other !== el && other.contains(el);
      });
    });
  }

  /**
   * Force shared collective header + footer on every f00 page.
   * Replaces legacy site-header / multi-column footers / copyright bars.
   */
  function replaceSiteChrome(doc, loc) {
    const root = doc;
    if (!root || !root.body) return;
    const cfg = inferConfigFromLocation(
      loc ||
        (typeof location !== "undefined"
          ? location
          : { hostname: "", pathname: "/" })
    );
    const headerHtml = renderHeader(cfg);
    const footerHtml = renderFooter(cfg);

    const headers = findSiteHeaders(root);
    if (headers.length) {
      headers[0].outerHTML = headerHtml;
      for (let i = 1; i < headers.length; i++) {
        // re-query may be stale after first replace — only remove if still connected
        if (headers[i].parentNode) headers[i].parentNode.removeChild(headers[i]);
      }
    } else if (!root.querySelector("[data-f00-chrome-mounted='header']")) {
      const skip = root.querySelector("a.skip, a.skip-link");
      if (skip && skip.parentNode) {
        skip.insertAdjacentHTML("afterend", headerHtml);
      } else {
        root.body.insertAdjacentHTML("afterbegin", headerHtml);
      }
    }

    const foots = findSiteFooters(root);
    if (foots.length) {
      foots[0].outerHTML = footerHtml;
      for (let i = 1; i < foots.length; i++) {
        if (foots[i].parentNode) foots[i].parentNode.removeChild(foots[i]);
      }
    } else if (!root.querySelector("[data-f00-chrome-mounted='footer']")) {
      root.body.insertAdjacentHTML("beforeend", footerHtml);
    }

    // purge any leftover copyright-only strips outside shared chrome
    Array.prototype.slice
      .call(root.querySelectorAll(".footer-bottom, .site-footer, footer"))
      .forEach(function (el) {
        if (el.getAttribute && el.getAttribute("data-f00-chrome-mounted")) return;
        if (isInsideContent(el)) return;
        const t = (el.textContent || "").replace(/\s+/g, " ");
        if (/©|copyright/i.test(t) && el.parentNode) {
          el.parentNode.removeChild(el);
        }
      });
  }

  function chromeIsHealthy(doc) {
    const h = doc.querySelector("[data-f00-chrome-mounted='header']");
    const f = doc.querySelector("[data-f00-chrome-mounted='footer']");
    if (!h || !f) return false;
    if (!f.querySelector(".foot-contact")) return false;
    if (/©|copyright/i.test(f.textContent || "")) return false;
    // collective nav present
    const nav = h.querySelector("nav.nav");
    if (!nav) return false;
    const hrefs = Array.prototype.map
      .call(nav.querySelectorAll("a"), function (a) {
        return a.getAttribute("href") || "";
      })
      .join(" ");
    return /who/.test(hrefs) && /donate/.test(hrefs);
  }

  function dedupeMounted(doc) {
    if (!doc || !doc.querySelectorAll) return;
    ["header", "footer"].forEach(function (kind) {
      const nodes = doc.querySelectorAll(
        "[data-f00-chrome-mounted='" + kind + "']"
      );
      for (let i = 1; i < nodes.length; i++) {
        if (nodes[i].parentNode) nodes[i].parentNode.removeChild(nodes[i]);
      }
    });
  }

  function bootstrap(doc) {
    const root = doc || (typeof document !== "undefined" ? document : null);
    if (!root) return;
    if (root.querySelector && root.querySelector("[data-f00-chrome]")) {
      mountAll(root);
    }
    dedupeMounted(root);
    if (!chromeIsHealthy(root)) {
      replaceSiteChrome(root);
    }
    dedupeMounted(root);
  }

  let _mo;
  function watchSpa(doc) {
    if (!doc || !doc.documentElement || typeof MutationObserver === "undefined") return;
    if (_mo) return;
    let t = null;
    _mo = new MutationObserver(function () {
      if (t) clearTimeout(t);
      t = setTimeout(function () {
        if (!chromeIsHealthy(doc)) bootstrap(doc);
      }, 80);
    });
    _mo.observe(doc.documentElement, { childList: true, subtree: true });
  }

  /**
   * Pure HTML pin: ensure one chrome script tag (Worker + tests).
   * @param {string} html
   * @param {{ href?: string }} [opts]
   */
  function pinChromeScript(html, opts) {
    if (typeof html !== "string") return html;
    if (html.includes(CHROME_MARKER)) return html;
    const href = (opts && opts.href) || CHROME_HREF;
    const tag =
      `<script src="${href}" data-f00-chrome-script defer></` + `script>`;
    if (/<\/body>/i.test(html)) {
      return html.replace(/<\/body>/i, `${tag}\n</body>`);
    }
    if (/<\/head>/i.test(html)) {
      return html.replace(/<\/head>/i, `  ${tag}\n</head>`);
    }
    return html + tag;
  }

  return {
    HUB,
    CONTACT_EMAIL,
    CONTACT_MAILTO,
    CONTACT_PHONE_DISPLAY,
    CONTACT_TEL,
    CHROME_HREF,
    CHROME_MARKER,
    DOMAIN_PROJECTS,
    normalizeConfig,
    configFromAttrs,
    inferConfigFromLocation,
    renderHeader,
    renderFooter,
    renderNav,
    renderContact,
    ensureContactHtml,
    replaceSiteChrome,
    chromeIsHealthy,
    bootstrap,
    watchSpa,
    mountAll,
    mountEl,
    pinChromeScript,
  };
});
