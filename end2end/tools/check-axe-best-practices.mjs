// Runs all of axe-core's "best-practice" rules on every built page, at three
// widths and in both colour themes, and prints how the site fares.
//
// On demand and local only: it is not a test and never runs in the pipeline.
// Axe's best practices are not WCAG criteria and may change with axe's
// versions; the tests in end2end/tests/ enable only three of them by name.
//
// Run from end2end/, after scripts/build-site.sh (it imports Playwright and
// axe from node_modules, so it adds no dependency of its own):
//
//   node tools/check-axe-best-practices.mjs
//
// Exit code: 1 if any rule has a violation or an incomplete result, else 0.
//
// How: every .html file under target/site is served over HTTP (404.html uses
// absolute paths for its assets), one browser context per width and theme,
// and each page is analysed with the tag "best-practice".
import { chromium } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { spawn } from 'node:child_process';
import { existsSync, readdirSync } from 'node:fs';
import { createServer } from 'node:net';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const site = resolve(root, 'target/site');

// The three bands of end2end/tests/support.ts (BANDS): keep them in step.
const widths = [
  { width: 393, height: 659 },
  { width: 768, height: 1024 },
  { width: 1280, height: 720 },
];
const themes = ['light', 'dark'];

if (!existsSync(join(site, 'index.html'))) {
  console.error(`${site}/index.html is missing: run scripts/build-site.sh first.`);
  process.exit(1);
}

function htmlFiles(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) return htmlFiles(path);
    return entry.name.endsWith('.html') ? [relative(site, path)] : [];
  });
}
const pages = htmlFiles(site).sort();

function freePort() {
  return new Promise((resolvePort, reject) => {
    const probe = createServer();
    probe.once('error', reject);
    probe.listen(0, '127.0.0.1', () => {
      const { port } = probe.address();
      probe.close(() => resolvePort(port));
    });
  });
}

const port = await freePort();
const server = spawn(
  'python3',
  ['-m', 'http.server', String(port), '--bind', '127.0.0.1', '--directory', site],
  { stdio: 'ignore' },
);
const baseURL = `http://127.0.0.1:${port}/`;

let browser;
let exitCode = 0;
try {
  let up = false;
  for (let i = 0; i < 100 && !up; i++) {
    try {
      up = (await fetch(baseURL)).ok;
    } catch {
      await new Promise((r) => setTimeout(r, 100));
    }
  }
  if (!up) throw new Error(`the local server did not answer at ${baseURL}`);

  browser = await chromium.launch();
  const rules = new Map(); // id -> counts
  const findings = [];
  let runs = 0;
  let axeVersion = '';

  for (const colorScheme of themes) {
    for (const viewport of widths) {
      const context = await browser.newContext({ viewport, colorScheme });
      const page = await context.newPage();
      for (const file of pages) {
        await page.goto(baseURL + file);
        await page.evaluate(() => document.fonts.ready);
        const result = await new AxeBuilder({ page }).withTags(['best-practice']).analyze();
        axeVersion = result.testEngine.version;
        runs++;
        for (const kind of ['passes', 'inapplicable', 'violations', 'incomplete']) {
          for (const rule of result[kind]) {
            const counts = rules.get(rule.id) ?? { passes: 0, inapplicable: 0, violations: 0, incomplete: 0 };
            counts[kind]++;
            rules.set(rule.id, counts);
            if (kind === 'violations' || kind === 'incomplete') {
              findings.push({
                kind,
                id: rule.id,
                file,
                width: viewport.width,
                theme: colorScheme,
                targets: rule.nodes.map((node) => node.target.join(' ')),
              });
            }
          }
        }
      }
      await context.close();
    }
  }

  console.log(`axe-core ${axeVersion}, best-practice rules: ${runs} runs (${pages.length} pages x ${widths.length} widths x ${themes.length} themes)\n`);
  const idWidth = Math.max(...[...rules.keys()].map((id) => id.length), 4);
  const head = (text) => text.padStart(16);
  console.log(`${'rule'.padEnd(idWidth)}${head('passed')}${head('not applicable')}${head('violations')}${head('incomplete')}`);
  for (const id of [...rules.keys()].sort()) {
    const c = rules.get(id);
    console.log(`${id.padEnd(idWidth)}${head(String(c.passes))}${head(String(c.inapplicable))}${head(String(c.violations))}${head(String(c.incomplete))}`);
  }

  if (findings.length > 0) {
    console.log('');
    for (const f of findings) {
      console.log(`${f.kind} ${f.id}: ${f.file}, ${f.width}px, ${f.theme}, ${f.targets.length} node(s): ${f.targets.join(' | ')}`);
    }
    const violations = findings.filter((f) => f.kind === 'violations').length;
    console.log(`\n${rules.size} rules, ${runs} runs: ${violations} violation(s), ${findings.length - violations} incomplete result(s)`);
    exitCode = 1;
  } else {
    console.log(`\n${rules.size} rules, ${runs} runs: no violations, nothing incomplete`);
  }
} catch (error) {
  console.error(error);
  exitCode = 1;
} finally {
  if (browser) await browser.close();
  server.kill();
}
process.exit(exitCode);
