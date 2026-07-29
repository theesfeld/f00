/**
 * Injects the ONE shared org stylesheet into HTML on *.f00.sh if missing.
 */
const THEME_LINK =
  '<link rel="stylesheet" href="https://f00.sh/theme/f00-theme.css" data-f00-theme="1" />';

export default {
  async fetch(request) {
    const url = new URL(request.url);
    if (url.pathname.startsWith("/theme/f00-theme")) {
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
