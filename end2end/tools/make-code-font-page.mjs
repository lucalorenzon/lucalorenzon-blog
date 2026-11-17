// Writes target/code-font-page.html, one standalone page that measures how
// wide a browser draws the site's code font, so the result can be pasted back.
//
// On demand and local only: it is not a test and never runs in the pipeline.
//
// Why: the wrap toggle's threshold (src/markdown.rs) assumes the code font's
// advance is CODE_ADVANCE_NUMERATOR / CODE_ADVANCE_DENOMINATOR of an em.
// Chromium on Linux rounds the advance to whole pixels unless the code has
// `text-rendering: geometricPrecision`. The page lets someone measure a
// browser on another system, such as Safari on macOS, that the agent cannot
// reach; see docs/backlog/measure-the-code-font-on-macos.md.
//
// Run from end2end/ (plain Node, no dependency, no browser needed):
//
//   node tools/make-code-font-page.mjs
//
// Then open target/code-font-page.html in the browser to measure and paste
// back the text of the box at the bottom of the page.
//
// How: the font is the one of the `code` role in config/fonts.txt, described
// in config/fonts/<folder>.txt; its `regular` file is embedded as a base64
// data: URL, so the page needs nothing else. The assumed advance is read from
// src/markdown.rs, never copied. The page measures a line of 100 "a" at every
// size from 10 to 30 px in tenths, with and without geometricPrecision.
import { mkdirSync, readFileSync, writeFileSync, existsSync } from 'node:fs';
import { dirname, extname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

function fail(message) {
  console.error(`make-code-font-page: ${message}`);
  process.exit(1);
}

// key=value lines; blank lines and lines starting with # are ignored.
function readPairs(path) {
  if (!existsSync(path)) fail(`${path} does not exist`);
  const pairs = new Map();
  for (const line of readFileSync(path, 'utf8').split('\n')) {
    const text = line.trim();
    if (text === '' || text.startsWith('#')) continue;
    const at = text.indexOf('=');
    if (at < 0) continue;
    pairs.set(text.slice(0, at).trim(), text.slice(at + 1).trim());
  }
  return pairs;
}

const roles = readPairs(join(root, 'config/fonts.txt'));
const folder = roles.get('code');
if (!folder) fail('config/fonts.txt has no `code=<folder>` line');

const fontConfig = join(root, 'config/fonts', `${folder}.txt`);
const font = readPairs(fontConfig);
const family = font.get('family');
if (!family) fail(`${fontConfig} has no \`family=\` line`);
const file = font.get('regular.file');
if (!file) fail(`${fontConfig} has no \`regular.file=\` line: no regular style`);
const style = font.get('regular.style') ?? 'normal';
const weight = font.get('regular.weight') ?? '400';

const fontPath = join(root, 'assets/fonts', folder, file);
if (!existsSync(fontPath)) fail(`the font file ${fontPath} does not exist`);
const formats = { '.woff2': ['font/woff2', 'woff2'], '.woff': ['font/woff', 'woff'], '.ttf': ['font/ttf', 'truetype'], '.otf': ['font/otf', 'opentype'] };
const format = formats[extname(file).toLowerCase()];
if (!format) fail(`unknown font file type: ${file}`);
const base64 = readFileSync(fontPath).toString('base64');

const markdown = readFileSync(join(root, 'src/markdown.rs'), 'utf8');
const numerator = markdown.match(/const\s+CODE_ADVANCE_NUMERATOR\s*:\s*\w+\s*=\s*(\d+)\s*;/);
const denominator = markdown.match(/const\s+CODE_ADVANCE_DENOMINATOR\s*:\s*\w+\s*=\s*(\d+)\s*;/);
if (!numerator || !denominator) {
  fail('CODE_ADVANCE_NUMERATOR / CODE_ADVANCE_DENOMINATOR not found in src/markdown.rs');
}

const familyJson = JSON.stringify(family);
const familyHtml = family.replace(/&/g, '&amp;').replace(/</g, '&lt;');
const familyCss = family.replace(/\\/g, '\\\\').replace(/"/g, '\\"');

const html = `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>${familyHtml} advance test</title>
<style>
@font-face{font-family:"${familyCss}";src:url(data:${format[0]};base64,${base64}) format("${format[1]}");font-weight:${weight};font-style:${style};font-display:block}
body{font-family:system-ui,sans-serif;margin:16px;background:#fff;color:#111}
.probe{font-family:"${familyCss}",monospace;white-space:pre;position:absolute;left:0;top:-1000px}
textarea{width:100%;height:12em;font-family:monospace;font-size:12px}
table{border-collapse:collapse}td,th{border:1px solid #999;padding:2px 6px;text-align:right}
</style></head><body>
<h1>${familyHtml} advance test</h1>
<p id="status">Measuring…</p>
<table id="summary"></table>
<p>Copy all the text below and paste it back:</p>
<textarea id="out" readonly></textarea>
<script>
(async () => {
  const family = ${familyJson};
  await document.fonts.load('16px "' + family + '"');
  await document.fonts.ready;
  const assumed = ${numerator[1]} / ${denominator[1]};
  function measure(extra) {
    const rows = [];
    for (let t = 100; t <= 300; t++) {
      const size = t / 10;
      const s = document.createElement('span');
      s.className = 'probe';
      s.style.fontSize = size + 'px';
      if (extra) s.style.textRendering = extra;
      s.textContent = 'a'.repeat(100);
      document.body.appendChild(s);
      const drawn = s.getBoundingClientRect().width / 100;
      s.remove();
      rows.push([size, Math.round(drawn * 10000) / 10000, Math.round(assumed * size * 10000) / 10000]);
    }
    return rows;
  }
  const result = { ua: navigator.userAgent, dpr: window.devicePixelRatio, family, fontLoaded: document.fonts.check('16px "' + family + '"'), plain: measure(''), geometric: measure('geometricPrecision') };
  const stats = (rows) => {
    const n = rows.filter(r => r[1] < r[2] - 0.01).length, w = rows.filter(r => r[1] > r[2] + 0.01).length;
    const whole = rows.filter(r => Math.abs(r[1] - Math.round(r[1])) < 1e-6).length;
    const max = Math.max(...rows.map(r => Math.abs(r[1] - r[2])));
    return [n, w, whole, max.toFixed(4)];
  };
  const tbl = document.getElementById('summary');
  tbl.innerHTML = '<tr><th>Rule</th><th>Narrower</th><th>Wider</th><th>Whole pixels</th><th>Max difference (px)</th></tr>' +
    [['none', stats(result.plain)], ['geometricPrecision', stats(result.geometric)]].map(([k, s]) => '<tr><th>' + k + '</th><td>' + s.join('</td><td>') + '</td></tr>').join('');
  document.getElementById('status').textContent = (result.fontLoaded ? family + ' loaded. ' : 'WARNING: ' + family + ' NOT loaded. ') + '201 sizes from 10 to 30 px measured, ' + navigator.userAgent;
  document.getElementById('out').value = JSON.stringify(result);
})();
</script></body></html>
`;

const out = join(root, 'target/code-font-page.html');
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, html);
console.log(`Wrote ${out} (font ${family}, advance ${numerator[1]}/${denominator[1]} em)`);
