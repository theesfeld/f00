/**
 * Drive shipped f00 chrome render + Worker pinChrome (not re-implementations).
 * Run: node --test tests/chrome-ssot.test.mjs
 */
import assert from "node:assert/strict";
import { test } from "node:test";
import { createRequire } from "node:module";
import { readFileSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = join(__dirname, "..");
const require = createRequire(import.meta.url);

const chrome = require(join(root, "site/theme/f00-chrome.js"));
const { pinChrome, pinDomain, CHROME_HREF, CHROME_MARKER } = await import(
  join(root, "workers/theme-inject/src/pins.js")
);

test("hub page=who footer/header: contact + collective nav", () => {
  const header = chrome.renderHeader({ page: "who", mode: "hub" });
  const footer = chrome.renderFooter({ page: "who", mode: "hub" });

  assert.match(footer, /mailto:tj@f00\.sh/);
  assert.match(footer, /tel:\+19733829681/);
  assert.match(footer, /tj@f00\.sh/);
  assert.match(footer, /USA \(973\) 382-9681/);

  for (const path of ["/who", "/constitution", "/plan", "/donate"]) {
    assert.match(header, new RegExp(`href="${path}"`));
  }
  assert.match(header, /nav-donate/);
  assert.match(header, /data-f00-donate/);
  assert.match(header, /aria-current="page"[^>]*>who<|>who[^>]*aria-current="page"/);
  assert.match(footer, /← f00/);
  assert.match(footer, /· who/);
  assert.match(footer, /data-f00-stars-total/);
  // no product-only nav invented
  assert.doesNotMatch(header, /install|download|docs#|benchmark/i);
});

test("project-shaped config: repo stars + ← f00, no product nav", () => {
  const header = chrome.renderHeader({
    page: "home",
    mode: "project",
    project: "f00tils",
    repo: "f00-sh/f00tils",
  });
  const footer = chrome.renderFooter({
    page: "home",
    mode: "project",
    project: "f00tils",
    repo: "f00-sh/f00tils",
  });

  assert.match(footer, /mailto:tj@f00\.sh/);
  assert.match(footer, /tel:\+19733829681/);
  assert.match(footer, /data-f00-stars/);
  assert.match(footer, /data-repo="f00-sh\/f00tils"/);
  assert.match(footer, /github\.com\/f00-sh\/f00tils/);
  assert.match(footer, /← f00/);
  assert.match(footer, /f00tils/);

  // collective links absolute to hub
  assert.match(header, /https:\/\/f00\.sh\/who/);
  assert.match(header, /https:\/\/f00\.sh\/constitution/);
  assert.match(header, /https:\/\/f00\.sh\/plan/);
  assert.match(header, /https:\/\/f00\.sh\/donate/);
  // projects must not be a relative hash on project sites (default page=home/index)
  assert.match(header, /href="https:\/\/f00\.sh\/#projects"/);
  assert.doesNotMatch(header, /href="#projects"/);

  // brand points home; project in mark
  assert.match(header, /brand-sub">\/f00tils</);
  assert.doesNotMatch(header, /class="nav"[^>]*>[\s\S]*install/i);
  assert.doesNotMatch(header, /multicall|coreutils only/i);
});

test("project mode default page (omit data-page) still uses hub projects URL", () => {
  // Documented project placeholders may omit data-page → normalize → page=index
  const header = chrome.renderHeader({
    mode: "project",
    project: "clun",
    repo: "f00-sh/clun",
  });
  assert.match(header, /href="https:\/\/f00\.sh\/#projects"/);
  assert.doesNotMatch(header, /href="#projects"/);
  assert.match(header, /https:\/\/f00\.sh\/who/);
});

test("inferConfigFromLocation maps subdomains to project chrome", () => {
  const core = chrome.inferConfigFromLocation({
    hostname: "coreutils.f00.sh",
    pathname: "/",
  });
  assert.equal(core.mode, "project");
  assert.equal(core.project, "f00tils");
  assert.equal(core.repo, "f00-sh/f00tils");

  const hub = chrome.inferConfigFromLocation({
    hostname: "f00.sh",
    pathname: "/who",
  });
  assert.equal(hub.mode, "hub");
  assert.equal(hub.page, "who");

  const header = chrome.renderHeader(core);
  assert.match(header, /href="https:\/\/f00\.sh\/#projects"/);
  assert.doesNotMatch(header, /href="#projects"/);
});

test("ensureContactHtml replaces legacy copyright footers with shared chrome", () => {
  const legacy =
    '<footer class="site-footer"><span>© 2026 TJ Theesfeld</span></footer>';
  const once = chrome.ensureContactHtml(legacy);
  assert.match(once, /foot-contact/);
  assert.match(once, /mailto:tj@f00\.sh/);
  assert.match(once, /tel:\+19733829681/);
  assert.doesNotMatch(once, /©|copyright/i);
  assert.match(once, /data-f00-chrome-mounted="footer"/);
  const twice = chrome.ensureContactHtml(once);
  assert.equal(twice, once);
});

test("hub and project footers never claim copyright", () => {
  const hub = chrome.renderFooter({ page: "index", mode: "hub" });
  const proj = chrome.renderFooter({
    mode: "project",
    project: "clun",
    repo: "f00-sh/clun",
  });
  assert.doesNotMatch(hub, /©|copyright/i);
  assert.doesNotMatch(proj, /©|copyright/i);
  assert.match(hub, /collective · for love · MIT/);
  assert.match(proj, /← f00/);
});

test("pinDomain stamps hub vs project data-f00-domain without FOUC", () => {
  const bare = "<!doctype html><html lang=\"en\"><head></head><body></body></html>";
  const hub = pinDomain(bare, "f00.sh");
  assert.match(hub, /data-f00-domain="hub"/);
  const proj = pinDomain(bare, "coreutils.f00.sh");
  assert.match(proj, /data-f00-domain="f00tils"/);
  const clun = pinDomain(bare, "clun.f00.sh");
  assert.match(clun, /data-f00-domain="clun"/);
  // idempotent
  assert.equal(pinDomain(hub, "f00.sh"), hub);
});

test("pinChrome injects once; no duplicate when marker present", () => {
  const bare = "<!doctype html><html><head></head><body><p>hi</p></body></html>";
  const once = pinChrome(bare);
  const count = (s) =>
    (s.match(/data-f00-chrome-script/g) || []).length;

  assert.equal(count(once), 1);
  assert.match(once, new RegExp(CHROME_HREF.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  assert.match(once, /<\/script>\s*<\/body>/i);

  const twice = pinChrome(once);
  assert.equal(count(twice), 1, "must not stack chrome scripts");
  assert.equal(twice, once);

  const already = `<html><body><script src="${CHROME_HREF}" ${CHROME_MARKER} defer></script></body></html>`;
  const pinned = pinChrome(already);
  assert.equal(count(pinned), 1);
  assert.equal(pinned, already);
});

test("hub HTML mounts via data-f00-chrome placeholders, not six inlined foot-contact blocks", () => {
  const hubPages = [
    "index.html",
    "who.html",
    "constitution.html",
    "plan.html",
    "donate.html",
    "ledger.html",
  ];
  const site = join(root, "site");
  const report = [];

  for (const name of hubPages) {
    const html = readFileSync(join(site, name), "utf8");
    const mounts = (html.match(/data-f00-chrome="(header|footer)"/g) || []).length;
    const footContactBlocks = (html.match(/class="foot-contact"/g) || []).length;
    const fullFooters = (html.match(/<footer class="foot">/g) || []).length;
    const chromeScript = html.includes("data-f00-chrome-script");

    report.push({
      name,
      mounts,
      footContactBlocks,
      fullFooters,
      chromeScript,
    });

    assert.ok(chromeScript, `${name} must load chrome script`);
    assert.ok(mounts >= 2, `${name} needs header+footer mount markers`);
    assert.equal(
      footContactBlocks,
      0,
      `${name} must not own inlined foot-contact SSOT (got ${footContactBlocks})`
    );
    assert.equal(
      fullFooters,
      0,
      `${name} must not inline full footer chrome (got ${fullFooters})`
    );
  }

  // body-copy emails may remain (e.g. who submission CTA)
  const who = readFileSync(join(site, "who.html"), "utf8");
  assert.match(who, /mailto:tj@f00\.sh/);

  // durable report for verifier
  const lines = report
    .map(
      (r) =>
        `${r.name}: mounts=${r.mounts} foot-contact=${r.footContactBlocks} footer-tags=${r.fullFooters} chrome-script=${r.chromeScript}`
    )
    .join("\n");
  assert.ok(lines.includes("index.html"));
  // attach for harness consumers via console
  console.log("hub-chrome-sources\n" + lines);
});
