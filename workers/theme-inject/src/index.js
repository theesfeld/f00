/**
 * Inject shared f00-theme.css into HTML on *.f00.sh if missing.
 * Never touch assets/theme files.
 */
const THEME_LINK =
  '<link rel="stylesheet" href="https://f00.sh/theme/f00-theme.css" data-f00-theme="1" />';

const SKIP_PREFIXES = [
  "/theme/",
  "/assets/",
  "/styles",
  "/catalog.json",
  "/favicon",
];

export default {
  async fetch(request) {
    const url = new URL(request.url);
    const path = url.pathname;

    // Never intercept static theme/assets
    if (SKIP_PREFIXES.some((p) => path.startsWith(p) || path.includes(p))) {
      return fetch(request);
    }
    // only HTML navigations
    const accept = request.headers.get("accept") || "";
    if (!accept.includes("text/html") && request.method === "GET") {
      return fetch(request);
    }

    const response = await fetch(request);
    const ct = response.headers.get("content-type") || "";
    if (!ct.includes("text/html")) {
      return response;
    }

    const headers = new Headers(response.headers);
    headers.delete("content-length");

    let html = await response.text();
    if (
      html.includes("f00-theme.css") ||
      html.includes("data-f00-theme") ||
      html.includes("/theme/f00-theme")
    ) {
      return new Response(html, {
        status: response.status,
        statusText: response.statusText,
        headers,
      });
    }

    if (/<\/head>/i.test(html)) {
      html = html.replace(/<\/head>/i, `  ${THEME_LINK}\n</head>`);
    } else if (/<body/i.test(html)) {
      html = html.replace(/<body/i, `${THEME_LINK}\n<body`);
    }

    return new Response(html, {
      status: response.status,
      statusText: response.statusText,
      headers,
    });
  },
};
