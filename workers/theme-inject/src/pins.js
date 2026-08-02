/**
 * Pure HTML pin helpers for f00-theme-inject (unit-testable, no fetch).
 */
export const THEME_HREF = "https://f00.sh/theme/f00-theme.css";
export const ENTROPY_HREF = "https://f00.sh/theme/f00-entropy.js?v=26";
export const CHROME_HREF = "https://f00.sh/theme/f00-chrome.js?v=5";
export const CHROME_MARKER = "data-f00-chrome-script";

const THEME_LINK = `<link rel="stylesheet" href="${THEME_HREF}" data-f00-theme="1" />`;
const ENTROPY_SCRIPT =
  `<script src="${ENTROPY_HREF}" data-f00-entropy-script defer></` + `script>`;
const CHROME_SCRIPT =
  `<script src="${CHROME_HREF}" data-f00-chrome-script defer></` + `script>`;

export function pinTheme(html) {
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/f00-theme(?:-\d+)?\.css(?:\?[^"'\s>]*)?/g,
    THEME_HREF
  );
  html = html.replace(
    /(?:https:\/\/f00\.sh)?\/theme\/f00-theme(?:-\d+)?\.css(?:\?[^"'\s>]*)?/g,
    THEME_HREF
  );
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/(?:pack\/|textures\/)?hb-shell[^"'\s>]*\.css/g,
    THEME_HREF
  );
  html = html.replace(
    /href=["'][^"']*hb-shell[^"']*\.css["']/gi,
    `href="${THEME_HREF}" data-f00-theme="1"`
  );

  if (!/data-f00-theme\s*=\s*["']?1["']?/.test(html) && !html.includes(THEME_HREF)) {
    if (/<\/head>/i.test(html)) {
      html = html.replace(/<\/head>/i, `  ${THEME_LINK}\n</head>`);
    } else if (/<body/i.test(html)) {
      html = html.replace(/<body/i, `${THEME_LINK}\n<body`);
    } else {
      html = THEME_LINK + "\n" + html;
    }
  } else if (html.includes(THEME_HREF) && !/data-f00-theme/.test(html)) {
    html = html.replace(
      new RegExp(
        `(<link[^>]+href=["']${THEME_HREF.replace(/\./g, "\\.")}["'][^>]*)(/?>)`,
        "i"
      ),
      `$1 data-f00-theme="1"$2`
    );
  }
  return html;
}

export function pinEntropy(html) {
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/f00-entropy\.js(?:\?v=[^"'\s>]*)?/g,
    ENTROPY_HREF
  );
  if (!html.includes("data-f00-entropy-script")) {
    if (/<\/body>/i.test(html)) {
      html = html.replace(/<\/body>/i, `${ENTROPY_SCRIPT}\n</body>`);
    } else if (/<\/head>/i.test(html)) {
      html = html.replace(/<\/head>/i, `  ${ENTROPY_SCRIPT}\n</head>`);
    } else {
      html += ENTROPY_SCRIPT;
    }
  }
  return html;
}

/** Ensure one chrome script; never stack duplicates. */
export function pinChrome(html) {
  if (typeof html !== "string") return html;
  // rewrite any older chrome URL to live SSOT
  html = html.replace(
    /https:\/\/f00\.sh\/theme\/f00-chrome\.js(?:\?v=[^"'\s>]*)?/g,
    CHROME_HREF
  );
  if (html.includes(CHROME_MARKER)) return html;
  if (/<\/body>/i.test(html)) {
    return html.replace(/<\/body>/i, `${CHROME_SCRIPT}\n</body>`);
  }
  if (/<\/head>/i.test(html)) {
    return html.replace(/<\/head>/i, `  ${CHROME_SCRIPT}\n</head>`);
  }
  return html + CHROME_SCRIPT;
}
