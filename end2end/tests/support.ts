import AxeBuilder from '@axe-core/playwright';
import { test, type Page } from '@playwright/test';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { relative, resolve } from 'node:path';

export const live = Boolean(process.env.BASE_URL?.trim());

// GitHub Pages may take some seconds to serve a new deployment: when
// checking the live site, retry the whole check for up to a minute.
// Locally the check runs once.
export async function retryWhenLive(check: () => Promise<void>): Promise<void> {
  if (live) {
    test.setTimeout(90_000);
    await test.expect(check).toPass({ timeout: 60_000, intervals: [2_000, 5_000] });
  } else {
    await check();
  }
}

// The site's breakpoints, in pixels, read from the file the build reads
// (config/breakpoints.txt), so that there is one figure for each, not copies that
// could drift apart. The tests also run on the live site, always from a checkout of
// the repository, so the file is on the disk there too. One line name=width; blank
// lines and lines starting with # are ignored (the build checks the rest).
export const BREAKPOINTS: Record<string, number> = Object.fromEntries(
  readFileSync(resolve(__dirname, '../../config/breakpoints.txt'), 'utf8')
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line !== '' && !line.startsWith('#'))
    .map((line) => {
      const [name, width] = line.split('=').map((part) => part.trim());
      return [name, Number.parseInt(width, 10)];
    }),
);

function breakpoint(name: string): number {
  const width = BREAKPOINTS[name];
  if (!Number.isInteger(width)) throw new Error(`config/breakpoints.txt names no breakpoint \`${name}\``);
  return width;
}

// Where the header changes its logo (md), and where the
// header becomes one row (xl). The width just below each is the other side of it.
export const MD = breakpoint('md');
export const XL = breakpoint('xl');

// The name of the entry standing for the width just below a breakpoint.
export const boundaryName = (breakpointWidth: number) => `boundary ${breakpointWidth - 1}`;
// The name of the entry standing for the breakpoint's own width, where no real
// device already sits.
export const atName = (breakpointWidth: number) => `at ${breakpointWidth}`;

// The real devices every layout test runs on, written by hand: a device does
// not change when a breakpoint moves.
const DEVICES = [
  { name: 'iPhone SE', width: 320, height: 568 },
  { name: 'iPhone 15', width: 393, height: 659 },
  { name: 'Pixel 7', width: 412, height: 839 },
  { name: 'iPhone 15 landscape', width: 659, height: 393 },
  { name: 'iPad Mini', width: 768, height: 1024 },
  { name: 'iPad 7th gen', width: 810, height: 1080 },
  { name: 'iPad Mini landscape', width: 1024, height: 768 },
  { name: 'iPad Pro 11 landscape', width: 1194, height: 834 },
  { name: 'Laptop', width: 1280, height: 720 },
  { name: 'Full HD', width: 1920, height: 1080 },
];

// The two sides of every breakpoint of the file, not devices: the width just
// below it ("boundary") and its own width ("at"), which follow the file. The
// "at" entry is left out where a real device already sits at that width, so
// the width is not run twice (with the file as it is today, the iPad Mini sits
// at md and the laptop at xl).
const SIDES = Object.values(BREAKPOINTS).flatMap((width) => [
  { name: boundaryName(width), width: width - 1, height: 800 },
  ...(DEVICES.some((device) => device.width === width) ? [] : [{ name: atName(width), width, height: 800 }]),
]);

// The devices and the sides of the breakpoints, by width.
export const VIEWPORTS = [...DEVICES, ...SIDES].sort((a, b) => a.width - b.width);

// The viewport at a width, whichever entry stands there (a device or a side).
export function viewportAt(width: number) {
  const found = VIEWPORTS.find((viewport) => viewport.width === width);
  if (!found) throw new Error(`no viewport at ${width}px`);
  return found;
}

// The text column's range: its text grows in a straight line from `min` px at a page
// width of `from` px to `max` px at `to` px, and stays at `min` below and at `max` above
// (`long-text-column` in style/input.css). Written by hand: these numbers repeat the
// stylesheet until docs/board/cards/read-the-column-text-range-from-the-configuration.md
// moves the range to config/, where the build and the tests read it.
export const TEXT_RANGE = { from: 320, to: 1366, min: 18, max: 20 };

// The text size the range gives at a page width: the straight line, clamped.
export function textSizeAt(width: number): number {
  return sizeOnRange(TEXT_RANGE, width);
}

// The size a range gives at a page width: the straight line from `min` px at `from` px
// to `max` px at `to` px, clamped outside them.
export function sizeOnRange(range: { from: number; to: number; min: number; max: number }, width: number): number {
  const { from, to, min, max } = range;
  return Math.min(max, Math.max(min, min + ((max - min) * (width - from)) / (to - from)));
}

// The page frame's own ranges, over the column's band of widths (`title-size` and
// `footer-text-size` in style/input.css). Written by hand, as TEXT_RANGE is.
export const TITLE_RANGE = { from: 320, to: 1366, min: 24, max: 32 };
export const FOOTER_RANGE = { from: 320, to: 1366, min: 16, max: 18 };

// The widths for tests of what is inside the text column. The text column grows
// smoothly with the page, up to 1366 px, so there is no longer one width per
// column size: the tests take a few widths that show what changes. 320, 843
// and 1366 show the text at 18, 19 and 20 px (843 is about the middle of the
// range); the three phones, 320, 393 and 412 px, are where the column fills the
// screen and its lines are shortest; 1366 is also where the column is at its
// widest in characters and in pixels, as the code block's case at the limit
// needs. The widths in between stay covered by the layout and readability tests,
// which run on every device. Why: docs/board/cards/check-at-every-width-only-what-depends-on-it.md
// and docs/board/cards/let-the-column-text-grow-within-an-injected-range.md
// The three phones are devices, by name; the last two are widths of the range, not
// devices, so they are not among VIEWPORTS or DEVICES.
const COLUMN_WIDTH_PHONES = ['iPhone SE', 'iPhone 15', 'Pixel 7'];
// The phones among them, where the column fills the screen and is narrowest.
export const COLUMN_PHONES = VIEWPORTS.filter(({ name }) => COLUMN_WIDTH_PHONES.includes(name));
export const COLUMN_WIDTHS = [
  ...COLUMN_PHONES,
  { name: 'text 19 px', width: 843, height: 800 },
  { name: 'range end', width: TEXT_RANGE.to, height: 800 },
];

// The pages show different elements per width band (and axe checks only what
// is shown), so some tests run once per band, with the same names and sizes
// used in header-layout.spec.ts. The first band is a phone, a device; the others
// start at md and at xl, taken by width from the file, so each is checked at the
// start of its band even when a breakpoint moves.
export const BANDS = [
  viewportAt(393), // text logo
  viewportAt(MD), // ASCII-art logo, title below the line
  viewportAt(XL), // title beside the logo
];

// The page the component checks of the text (readability.spec.ts and
// content-layout.spec.ts) run on: per ADR-004 a component is checked once, on
// the cases written to stress it most. They are on the style guide's text
// page (content/style-guide/text.md).
export const TEXT_PAGE = './style-guide/text.html';

// The guard of the text checks: before measuring, every case of the text page
// takes at least two lines at the width under test, so that a check never
// passes with nothing to check (a heading, a paragraph or a list item that
// fits one line has nothing to overlap, nor to space).
//
// How the cases are found. The Markdown holds no HTML, so a case cannot carry
// an attribute. The page's text is the <div> inside <main> (the breadcrumb and
// the link back to the top are outside it), and it is built so that:
//   - the introduction and the first paragraph after each heading explain the
//     case to the reader (they are the author's, may be short, and are not
//     cases);
//   - every other piece of that text is a case: every heading (h2, h3, h4),
//     every list item, and every paragraph that is not the first after a
//     heading, that is the paragraphs of the section about paragraphs and the
//     paragraph that shows the elements inside a line.
// So a case added to the page is guarded without being listed, and a case
// shortened for the sake of the text fails here and not silently.
//
// Returns the number of lines of each case at the current width, by name.
export async function expectEveryCaseWraps(page: Page): Promise<{ name: string; lines: number }[]> {
  const cases = await page.evaluate(() => {
    const text = document.querySelector('main > div')!;
    const linesOf = (el: Element) => {
      // Lines as the browser breaks them: the boxes of the element's content,
      // grouped by their vertical position.
      const range = document.createRange();
      range.selectNodeContents(el);
      const rects = Array.from(range.getClientRects()).filter((r) => r.width > 0 && r.height > 0);
      rects.sort((a, b) => a.top - b.top);
      const lines: { top: number; bottom: number }[] = [];
      for (const r of rects) {
        const middle = (r.top + r.bottom) / 2;
        const line = lines.find((l) => middle > l.top && middle < l.bottom);
        if (line) {
          line.top = Math.min(line.top, r.top);
          line.bottom = Math.max(line.bottom, r.bottom);
        } else {
          lines.push({ top: r.top, bottom: r.bottom });
        }
      }
      return lines.length;
    };
    const found: { name: string; lines: number }[] = [];
    let afterHeading = false; // the next paragraph is the explanation of a case
    let introduction = true; // the paragraphs before the first heading explain the page
    for (const el of Array.from(text.querySelectorAll('h2, h3, h4, p, li'))) {
      const tag = el.tagName.toLowerCase();
      if (/^h[2-4]$/.test(tag)) {
        introduction = false;
        afterHeading = true;
      } else if (tag === 'p') {
        const explanation = introduction || afterHeading;
        afterHeading = false;
        if (explanation) continue;
      }
      found.push({ name: `<${tag}> "${el.textContent?.trim().slice(0, 40)}"`, lines: linesOf(el) });
    }
    return found;
  });
  const width = page.viewportSize()!.width;
  const short = cases.filter((c) => c.lines < 2);
  if (cases.length === 0 || short.length > 0) {
    throw new Error(
      cases.length === 0
        ? `${TEXT_PAGE} holds no case: the text checks would pass with nothing to check.`
        : `at ${width}px, these cases of ${TEXT_PAGE} take one line, at least two are needed so that the text checks have something to check:\n` +
            short.map((c) => `  ${c.name}`).join('\n'),
    );
  }
  return cases;
}

// The built site, and its pages, found by walking the folder: a new page is
// covered by the tests that use these without being listed. A live run has no
// folder to walk, so the lists are empty and those tests skip.
export const SITE = resolve(__dirname, '../../target/site');

export function pagesOf(folder: string): string[] {
  if (!existsSync(folder)) return [];
  return readdirSync(folder, { withFileTypes: true }).flatMap((entry) => {
    const path = resolve(folder, entry.name);
    if (entry.isDirectory()) return pagesOf(path);
    return entry.name.endsWith('.html') ? [path] : [];
  });
}

export const ADDRESSES = pagesOf(SITE).map((path) => './' + relative(SITE, path));

// The paths, on the server under test, of the pages a sitemap lists.
export function pathsOfSitemap(xml: string): string[] {
  const addresses = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((match) => match[1].trim());
  // The <loc> addresses always point to the site's production address, whatever
  // server is under test. Locally the pages are served from 127.0.0.1, and
  // Lighthouse's canonical rule does not compare domains, so each address is
  // mapped to the server under test by keeping its path and resolving it,
  // relative, against the test's base URL.
  return addresses.map((address) => {
    const url = new URL(address);
    return './' + url.pathname.replace(/^\/+/, '') + url.search;
  });
}

// The pages the "net" tests (written once, run on every page) check. The list
// is the build's sitemap in both moments: locally and in the build the site is
// in target/site; in the live check after a publication the pipeline puts only
// the build's sitemap.xml there (see deploy.yml), so the tests that walk the
// folder (ADDRESSES) still find no pages and skip. It is read
// synchronously, because Playwright builds the list of tests before running
// any. A missing or empty sitemap throws: a net check with nothing to check
// must not pass.
function netPages(): { name: string; address: string }[] {
  const file = resolve(SITE, 'sitemap.xml');
  if (!existsSync(file)) {
    throw new Error(
      `${file} not found: run scripts/build-site.sh from the repository root before the tests` +
        ' (for the live run, the pipeline copies the build\'s sitemap.xml there).',
    );
  }
  const paths = pathsOfSitemap(readFileSync(file, 'utf8'));
  if (paths.length === 0) {
    throw new Error(`${file} lists no <loc>: the net tests would have no page to check.`);
  }
  return paths.map((address) => ({ name: address === './' ? 'home page' : address.replace(/^\.\//, ''), address }));
}

export const NET_PAGES = netPages();

// What a reader's own style sheet may set, from the text of the criterion.
export const READER_SPACING = `
  * {
    line-height: 1.5 !important;
    letter-spacing: 0.12em !important;
    word-spacing: 0.16em !important;
  }
  p { margin-bottom: 2em !important; }
`;

// axe, shared by the net (accessibility.spec.ts) and by the component tests.
//
// TAGS: every axe rule tagged for WCAG 2.0, 2.1 and 2.2 at levels A and AA,
// plus the three level-AAA rules axe can check (enhanced contrast, links with
// the same text leading to the same purpose, no automatic page refresh).
// Rules axe has disabled by default (color-contrast-enhanced is one) still run
// when selected by tag or by id: axe skips a rule only if it has none of the
// selected tags and is not named, or carries an excluded tag (deprecated,
// experimental).
export const TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa', 'wcag2aaa'];

// Rules that must have been evaluated and passed by a contrast test: the page
// has visible text, so if they are missing from the passes, axe did not look
// at the colours at all and an empty list of violations would mean nothing.
export const CONTRAST_RULES = ['color-contrast', 'color-contrast-enhanced'];

// The structure of the headings. They belong to axe's "best practice" group,
// not to a WCAG criterion, so the selection by tag leaves them out; they are
// enabled by name. (The whole group is not selected: a new version of axe
// could change its rules and stop the build for a rule nobody chose.)
export const HEADING_RULES = ['page-has-heading-one', 'heading-order', 'empty-heading'];

// The type of axe's results, taken from the package the project declares
// rather than from axe-core, which only comes with it.
export type AxeResults = Awaited<ReturnType<AxeBuilder['analyze']>>;

// Runs axe on the open page, optionally limited to the elements matching
// `scope`. AxeBuilder.withTags() and withRules() both set `runOnly` and
// exclude each other, and disableRules() replaces the `rules` option, so the
// options are written whole: `runOnly` selects the tags, and `rules` switches
// single rules on or off by name, which axe honours over the tags.
//   - 'rules': TAGS without the contrast rules, plus `extraRules` by name
//   - 'contrast': only the two contrast rules, selected by id
export function runAxe(
  page: Page,
  kind: 'rules' | 'contrast',
  { scope, extraRules = [] }: { scope?: string; extraRules?: string[] } = {},
): Promise<AxeResults> {
  const builder = new AxeBuilder({ page });
  if (scope) builder.include(scope);
  if (kind === 'contrast') return builder.withRules(CONTRAST_RULES).analyze();
  const rules: Record<string, { enabled: boolean }> = {};
  for (const id of CONTRAST_RULES) rules[id] = { enabled: false };
  for (const id of extraRules) rules[id] = { enabled: true };
  return builder.options({ runOnly: { type: 'tag', values: TAGS }, rules }).analyze();
}

export function axeReport(violations: AxeResults['violations']): string {
  return violations
    .map((v) => {
      const targets = v.nodes.map((n) => `    ${n.target.join(' ')}`).join('\n');
      return `- ${v.id} (${v.impact ?? 'no impact given'}): ${v.help}\n${targets}`;
    })
    .join('\n');
}
