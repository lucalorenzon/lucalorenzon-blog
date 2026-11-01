import { test, expect, type Page } from '@playwright/test';
import { retryWhenLive, VIEWPORTS } from './support';

// Enforces that the content (the curriculum in <main>) is readable on every
// device. It checks what the reader sees (sizes, positions, colours), never
// how the page is built.
//
// Rules on every viewport:
//   1. no horizontal scroll
//   2. no line wider than 80 characters of the font (8px each)
//   3. section headings (h2) are larger than body text
//   4. role headings (h3) stand out from the page background
//   5. at the end of the page the content ends at least 32px above the sticky footer
//   6. consecutive headings, paragraphs and list items do not overlap
//   7. the first section has no line above it (the header has its own); the others do

const CHAR_WIDTH = 8;
const MAX_CHARACTERS = 80;
const MIN_GAP_ABOVE_FOOTER = 32; // the space used before sections

type Snapshot = {
  innerWidth: number;
  scrollWidth: number;
  mainContentWidth: number;
  h2Count: number;
  h2MinFontSize: number;
  pFontSize: number;
  h2TopBorders: number[];
  h3Count: number;
  pageBackground: string;
  h3Backgrounds: string[];
  lastBottom: number;
  footerTop: number;
  overlaps: string[];
};

// ---- Measuring (runs in the browser) -------------------------------------

async function snapshot(page: Page): Promise<Snapshot> {
  // Scroll to the very end first: the footer is sticky, so only there can we
  // see what it covers.
  await page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));

  return page.evaluate(() => {
    const main = document.querySelector('main')!;
    const mainStyle = getComputedStyle(main);
    const contentWidth =
      main.getBoundingClientRect().width -
      parseFloat(mainStyle.paddingLeft) -
      parseFloat(mainStyle.paddingRight);

    const h2s = Array.from(main.querySelectorAll('h2'));
    const h3s = Array.from(main.querySelectorAll('h3'));
    const p = main.querySelector('p');

    // The last element in main that takes room: its bottom must clear the footer.
    const all = Array.from(main.querySelectorAll('*')).filter((el) => el.getBoundingClientRect().height > 0);
    const lastBottom = Math.max(...all.map((el) => el.getBoundingClientRect().bottom));
    const footer = document.querySelector('footer')!;

    // Cheap overlap check: each block element against the next one in reading order.
    const blocks = Array.from(main.querySelectorAll('h2, h3, h4, p, li'));
    const overlaps: string[] = [];
    for (let i = 0; i + 1 < blocks.length; i++) {
      const a = blocks[i].getBoundingClientRect();
      const b = blocks[i + 1].getBoundingClientRect();
      // A list item may contain the next one (nested list): only flag partial overlaps
      if (blocks[i].contains(blocks[i + 1])) continue;
      if (a.bottom > b.top + 0.5) {
        overlaps.push(`"${blocks[i].textContent?.slice(0, 30)}" overlaps "${blocks[i + 1].textContent?.slice(0, 30)}"`);
      }
    }

    return {
      innerWidth: window.innerWidth,
      scrollWidth: document.documentElement.scrollWidth,
      mainContentWidth: contentWidth,
      h2Count: h2s.length,
      h2MinFontSize: Math.min(...h2s.map((el) => parseFloat(getComputedStyle(el).fontSize))),
      pFontSize: p ? parseFloat(getComputedStyle(p).fontSize) : NaN,
      h2TopBorders: h2s.map((el) => parseFloat(getComputedStyle(el).borderTopWidth)),
      h3Count: h3s.length,
      pageBackground: getComputedStyle(document.body).backgroundColor,
      h3Backgrounds: h3s.map((el) => getComputedStyle(el).backgroundColor),
      lastBottom,
      footerTop: footer.getBoundingClientRect().top,
      overlaps,
    };
  });
}

// ---- Rules (run in Node) -------------------------------------------------

// Returns one message per broken rule; empty means the content is as agreed.
function violations(s: Snapshot): string[] {
  const where = `[${s.innerWidth}px]`;
  const out: string[] = [];
  const rule = (ok: boolean, message: string) => {
    if (!ok) out.push(`${where} ${message}`);
  };

  rule(s.scrollWidth <= s.innerWidth, `no horizontal scroll: scrollWidth ${s.scrollWidth} > innerWidth ${s.innerWidth}`);
  rule(
    s.mainContentWidth <= MAX_CHARACTERS * CHAR_WIDTH,
    `content must be at most ${MAX_CHARACTERS} characters wide (${MAX_CHARACTERS * CHAR_WIDTH}px), is ${s.mainContentWidth}px`,
  );
  rule(s.h2Count > 0, 'the content must have at least one h2');
  rule(
    s.h2MinFontSize > s.pFontSize,
    `h2 must be larger than body text: smallest h2 ${s.h2MinFontSize}px, paragraph ${s.pFontSize}px`,
  );
  rule(
    s.h2TopBorders[0] === 0,
    `the first h2 must have no top border (the header has its own line), has ${s.h2TopBorders[0]}px`,
  );
  s.h2TopBorders.slice(1).forEach((w, i) =>
    rule(w > 0, `h2 number ${i + 2} must have a top border, has ${w}px`),
  );
  rule(s.h3Count > 0, 'the content must have at least one h3');
  s.h3Backgrounds.forEach((bg, i) =>
    rule(bg !== s.pageBackground, `h3 number ${i + 1} must differ from the page background (${s.pageBackground}), has ${bg}`),
  );
  rule(
    s.footerTop - s.lastBottom >= MIN_GAP_ABOVE_FOOTER - 0.5,
    `the end of the content (${s.lastBottom}) must be at least ${MIN_GAP_ABOVE_FOOTER}px above the footer (${s.footerTop}), gap is ${s.footerTop - s.lastBottom}px`,
  );
  for (const o of s.overlaps) out.push(`${where} ${o}`);
  return out;
}

// ---- Tests ---------------------------------------------------------------

for (const { name, width, height } of VIEWPORTS) {
  test(`content layout at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto('./');
      await page.evaluate(() => document.fonts.ready);
      const found = violations(await snapshot(page));
      expect(found.length, `content rules broken:\n${found.join('\n')}`).toBe(0);
    });
  });
}
