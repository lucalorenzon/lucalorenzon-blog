import { test, expect, type Page } from '@playwright/test';
import {
  COLUMN_WIDTHS,
  expectWorstCaseStillWorst,
  NET_PAGES,
  READER_SPACING,
  retryWhenLive,
  VIEWPORTS,
  WORST_CASE,
} from './support';

// Enforces the visual presentation of the text. It checks what the reader
// sees (rendered sizes, positions, lines), never how the page is built.
//
// Two kinds of test, as ADR-004 says (docs/adr/ADR-004-test-components-once-and-pages-with-a-net.md):
//
// The NET runs on every page of the sitemap (NET_PAGES in support.ts), because
// the rule depends on the text, which any page may change:
//   - no rendered line is longer than 80 characters (WCAG 1.4.8), at the
//     widest column, 1280x720 (text 20 px, column 704 px, from 768 px up).
//     Counted on the lines as the browser breaks them, not estimated from the
//     width: the font is proportional, so no style rule can bound it.
// (The sideways scroll, without and with the reader's spacing, is the other
// net check: see content-layout.spec.ts.)
//
// The COMPONENT checks run once, on the worst case (WORST_CASE in support.ts,
// the After Save Event article), at the widths of the text column
// (COLUMN_WIDTHS: it is all they depend on). Each test first checks the guard
// (expectWorstCaseStillWorst): the article still has every kind of element the
// checks look at.
//
// WCAG 1.4.8 (visual presentation, level AAA):
//   1. the line height of every paragraph and list item is at least 1.5
//      times its font size
//   2. the empty space between two consecutive paragraphs is at least 1.5
//      times the line height
//   3. no text in the content is justified
//
// WCAG 1.4.12 (text spacing, level AA; the header depends on the screen
// width and the title, so it is checked in header-layout.spec.ts at every
// width): when the reader raises line height to 1.5, letter spacing to
// 0.12em, word spacing to 0.16em and the space after paragraphs to 2em (as a
// bookmarklet or an extension would do), nothing is lost:
//   4. consecutive blocks of the content do not overlap
//   5. no element outside the header cuts off its content
//   6. at the end of the page the content ends at least 32px above the
//      sticky footer
//   (no horizontal scroll with the spacing: net, content-layout.spec.ts)

const MIN_LINE_HEIGHT_RATIO = 1.5;
const MIN_PARAGRAPH_GAP_RATIO = 1.5; // of the line height
const MAX_CHARACTERS = 80;
const MIN_GAP_ABOVE_FOOTER = 32;

type LongestLine = { characters: number; text: string };

// ---- Measuring (runs in the browser) -------------------------------------

async function gotoReady(page: Page, address: string) {
  await page.goto(address);
  await page.evaluate(() => document.fonts.ready);
}

async function lineHeightsTooSmall(page: Page): Promise<string[]> {
  return page.evaluate((minRatio) => {
    const out: string[] = [];
    for (const el of Array.from(document.querySelectorAll('main p, main li'))) {
      const style = getComputedStyle(el);
      const fontSize = parseFloat(style.fontSize);
      const lineHeight = parseFloat(style.lineHeight); // NaN for "normal"
      if (!(lineHeight >= minRatio * fontSize - 0.01)) {
        out.push(
          `"${el.textContent?.slice(0, 30)}" has line height ${style.lineHeight} on font size ${fontSize}px, needs at least ${minRatio * fontSize}px`,
        );
      }
    }
    return out;
  }, MIN_LINE_HEIGHT_RATIO);
}

async function paragraphGaps(page: Page) {
  return page.evaluate(() => {
    const pairs: { text: string; gap: number; lineHeight: number }[] = [];
    for (const p of Array.from(document.querySelectorAll('main p'))) {
      const next = p.nextElementSibling;
      if (!next || next.tagName !== 'P') continue;
      pairs.push({
        text: `"${p.textContent?.slice(0, 30)}" and "${next.textContent?.slice(0, 30)}"`,
        gap: next.getBoundingClientRect().top - p.getBoundingClientRect().bottom,
        lineHeight: parseFloat(getComputedStyle(p).lineHeight),
      });
    }
    return pairs;
  });
}

// The longest line as rendered. Every character gets its own Range; the
// characters are grouped by line from their position; spaces inside a line
// count, spaces at a wrap (and collapsed ones, which have no box) do not.
// A <br> or a break after a hyphen simply starts a new line.
async function longestLine(page: Page): Promise<LongestLine> {
  return page.evaluate(() => {
    const blockSelector = 'p, li, h2, h3, h4';
    let best = { characters: 0, text: '' };
    for (const block of Array.from(document.querySelectorAll(`main ${blockSelector}`))) {
      const lines: { top: number; bottom: number; text: string }[] = [];
      const walker = document.createTreeWalker(block, NodeFilter.SHOW_TEXT);
      for (let node = walker.nextNode(); node; node = walker.nextNode()) {
        // Text of a nested list item belongs to that item, not to this one.
        if (node.parentElement?.closest(blockSelector) !== block) continue;
        const data = node.textContent ?? '';
        for (let i = 0; i < data.length; i++) {
          const range = document.createRange();
          range.setStart(node, i);
          range.setEnd(node, i + 1);
          const rect = Array.from(range.getClientRects()).find((r) => r.width > 0 || r.height > 0);
          if (!rect) continue; // collapsed whitespace: nothing rendered
          const middle = (rect.top + rect.bottom) / 2;
          let line = lines[lines.length - 1];
          if (!line || middle > line.bottom || middle < line.top) {
            line = { top: rect.top, bottom: rect.bottom, text: '' };
            lines.push(line);
          }
          line.text += /\s/.test(data[i]) ? ' ' : data[i];
        }
      }
      for (const line of lines) {
        const text = line.text.trim();
        if (text.length > best.characters) best = { characters: text.length, text };
      }
    }
    return best;
  });
}

async function justifiedElements(page: Page): Promise<string[]> {
  return page.evaluate(() =>
    Array.from(document.querySelectorAll('main, main *'))
      .filter((el) => getComputedStyle(el).textAlign.startsWith('justify'))
      .map((el) => `<${el.tagName.toLowerCase()}> "${el.textContent?.slice(0, 30)}"`),
  );
}

// After the reader's spacing is applied.
async function spacingProblems(page: Page): Promise<string[]> {
  await page.addStyleTag({ content: READER_SPACING });
  await page.evaluate(() => document.fonts.ready);
  // Scroll to the very end: the footer is sticky, only there can we see what it covers.
  await page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));

  return page.evaluate(
    ({ minGap }) => {
      const out: string[] = [];
      const label = (el: Element) => `<${el.tagName.toLowerCase()}> "${el.textContent?.slice(0, 30)}"`;

      // Each block of the content against the next one in reading order.
      const main = document.querySelector('main')!;
      const blocks = Array.from(main.querySelectorAll('h2, h3, h4, p, li'));
      for (let i = 0; i + 1 < blocks.length; i++) {
        if (blocks[i].contains(blocks[i + 1])) continue; // nested list
        const a = blocks[i].getBoundingClientRect();
        const b = blocks[i + 1].getBoundingClientRect();
        if (a.bottom > b.top + 0.5) out.push(`${label(blocks[i])} overlaps ${label(blocks[i + 1])}`);
      }

      // Nothing cut off by its own box. The header is left to header-layout.spec.ts,
      // which checks it at every width with the longest titles the build accepts.
      const header = document.querySelector('header:not(main header)')!;
      for (const el of Array.from(document.querySelectorAll('body, body *'))) {
        if (header.contains(el)) continue;
        const rect = el.getBoundingClientRect();
        if (rect.width === 0 || rect.height === 0) continue;
        const style = getComputedStyle(el);
        // Code scrolls sideways on purpose: line breaks are meaningful in
        // code (WCAG technique G224), and the "wrap" toggle offers to wrap the
        // lines when they do not fit. Only horizontal scrolling of the code
        // scroller is allowed here. With the reader's letter spacing applied
        // here the lines are wider than the toggle's threshold (computed
        // without it), so the code scrolls although the toggle may be hidden:
        // nothing is lost, the code is still reachable by scrolling.
        //
        // Two elements of the record viewer (a table too narrow for its
        // columns, see tables.spec.ts) clip vertically on purpose. The box
        // shows one record at a time by design: the other records are
        // reachable by gesture or keyboard, the number row says how many
        // there are, and screen readers get every record as a group. The
        // number row is a one-line window over a strip of numbers, only one
        // of which is meant to show. Clipping sideways stays forbidden for
        // both.
        const clipsX = style.overflowX !== 'visible' && !el.matches('.code-scroller');
        const clipsY = style.overflowY !== 'visible' && !el.matches('.viewer-box, .viewer-count-window');
        if ((clipsX && el.scrollWidth > el.clientWidth + 1) || (clipsY && el.scrollHeight > el.clientHeight + 1)) {
          out.push(
            `${label(el)} cuts off its content (scroll ${el.scrollWidth}x${el.scrollHeight}, box ${el.clientWidth}x${el.clientHeight})`,
          );
        }
      }

      // The end of the content clears the sticky footer.
      const taking = Array.from(main.querySelectorAll('*')).filter((el) => el.getBoundingClientRect().height > 0);
      const lastBottom = Math.max(...taking.map((el) => el.getBoundingClientRect().bottom));
      const footerTop = document.querySelector('footer:not(main footer)')!.getBoundingClientRect().top;
      if (footerTop - lastBottom < minGap - 0.5) {
        out.push(
          `the end of the content (${lastBottom}) must be at least ${minGap}px above the footer (${footerTop}), gap is ${footerTop - lastBottom}px`,
        );
      }
      return out;
    },
    { minGap: MIN_GAP_ABOVE_FOOTER },
  );
}

// ---- Tests ---------------------------------------------------------------

// Net: every page of the sitemap, at the widest column.
for (const { name: pageName, address } of NET_PAGES) {
  const { name, width, height } = VIEWPORTS.find((v) => v.name === 'Laptop')!;
  test(`no line over ${MAX_CHARACTERS} characters on the ${pageName} at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await gotoReady(page, address);
      const longest = await longestLine(page);
      console.log(`[${width}px] longest line: ${longest.characters} characters: "${longest.text}"`);
      expect(
        longest.characters,
        `a line is ${longest.characters} characters long, at most ${MAX_CHARACTERS} allowed: "${longest.text}"`,
      ).toBeLessThanOrEqual(MAX_CHARACTERS);
    });
  });
}

// Components, once, on the worst case.
for (const { name, width, height } of COLUMN_WIDTHS) {
  test(`visual presentation (WCAG 1.4.8) of the worst case at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await gotoReady(page, WORST_CASE);
      await expectWorstCaseStillWorst(page);

      const tooSmall = await lineHeightsTooSmall(page);
      expect(tooSmall, `line height must be at least ${MIN_LINE_HEIGHT_RATIO} times the font size:\n${tooSmall.join('\n')}`).toEqual([]);

      const pairs = await paragraphGaps(page);
      const tight = pairs.filter((p) => !(p.gap >= MIN_PARAGRAPH_GAP_RATIO * p.lineHeight - 0.5));
      expect(
        tight,
        `space between paragraphs must be at least ${MIN_PARAGRAPH_GAP_RATIO} times the line height:\n${tight.map((p) => `${p.text}: ${p.gap}px on line height ${p.lineHeight}px`).join('\n')}`,
      ).toEqual([]);

      const justified = await justifiedElements(page);
      expect(justified, `text must not be justified:\n${justified.join('\n')}`).toEqual([]);
    });
  });
}

for (const { name, width, height } of COLUMN_WIDTHS) {
  test(`text spacing (WCAG 1.4.12) of the worst case at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await gotoReady(page, WORST_CASE);
      await expectWorstCaseStillWorst(page);
      const found = await spacingProblems(page);
      expect(found, `with the reader's text spacing:\n${found.join('\n')}`).toEqual([]);
    });
  });
}
