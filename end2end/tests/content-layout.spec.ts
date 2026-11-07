import { test, expect, type Page } from '@playwright/test';
import { retryWhenLive, TEXT_PAGES, VIEWPORTS } from './support';

// Enforces that the content (the text in <main>: the curriculum, the licence) is readable on every
// device. It checks what the reader sees (sizes, positions, colours), never
// how the page is built.
//
// Rules on every viewport:
//   1. no horizontal scroll
//   2. section headings (h2) are larger than body text
//   3. role headings (h3) stand out from the body text by colour (amber, no inverse video)
//   4. at the end of the page the content ends at least 32px above the sticky footer
//   5. consecutive headings, paragraphs and list items do not overlap
//   6. section headings (h2) stand out from the body text by colour (amber); no line above them
//      (rules 3 and 6 used to check the amber line above h2 and the inverse video of h3: the
//      headings were changed to amber text, the intent "sections and roles stay apart from the text" holds)
//   7. a numbered list (ol) shows its numbers and is indented and spaced like the bulleted
//      lists of the same column (checked on the After Save Event article)
//
// The width of the lines (at most 80 characters, counted on the rendered
// lines) is checked in readability.spec.ts.

const MIN_GAP_ABOVE_FOOTER = 32; // the space used before sections

type Snapshot = {
  innerWidth: number;
  scrollWidth: number;
  h2Count: number;
  h2MinFontSize: number;
  pFontSize: number;
  pColor: string;
  h2Colors: string[];
  h3Count: number;
  h3Colors: string[];
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

    const h2s = Array.from(main.querySelectorAll('h2'));
    const h3s = Array.from(main.querySelectorAll('h3'));
    const p = main.querySelector('p');

    // The last element in main that takes room: its bottom must clear the footer.
    const all = Array.from(main.querySelectorAll('*')).filter((el) => el.getBoundingClientRect().height > 0);
    const lastBottom = Math.max(...all.map((el) => el.getBoundingClientRect().bottom));
    const footer = document.querySelector('footer:not(main footer)')!;

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
      h2Count: h2s.length,
      h2MinFontSize: Math.min(...h2s.map((el) => parseFloat(getComputedStyle(el).fontSize))),
      pFontSize: p ? parseFloat(getComputedStyle(p).fontSize) : NaN,
      pColor: p ? getComputedStyle(p).color : '',
      h2Colors: h2s.map((el) => getComputedStyle(el).color),
      h3Count: h3s.length,
      h3Colors: h3s.map((el) => getComputedStyle(el).color),
      lastBottom,
      footerTop: footer.getBoundingClientRect().top,
      overlaps,
    };
  });
}

// ---- Rules (run in Node) -------------------------------------------------

// Returns one message per broken rule; empty means the content is as agreed.
function violations(s: Snapshot, hasRoles: boolean): string[] {
  const where = `[${s.innerWidth}px]`;
  const out: string[] = [];
  const rule = (ok: boolean, message: string) => {
    if (!ok) out.push(`${where} ${message}`);
  };

  rule(s.scrollWidth <= s.innerWidth, `no horizontal scroll: scrollWidth ${s.scrollWidth} > innerWidth ${s.innerWidth}`);
  rule(s.h2Count > 0, 'the content must have at least one h2');
  rule(
    s.h2MinFontSize > s.pFontSize,
    `h2 must be larger than body text: smallest h2 ${s.h2MinFontSize}px, paragraph ${s.pFontSize}px`,
  );
  s.h2Colors.forEach((c, i) =>
    rule(c !== s.pColor, `h2 number ${i + 1} must differ in colour from the body text (${s.pColor}), has ${c}`),
  );
  if (hasRoles) rule(s.h3Count > 0, 'the content must have at least one h3');
  s.h3Colors.forEach((c, i) =>
    rule(c !== s.pColor, `h3 number ${i + 1} must differ in colour from the body text (${s.pColor}), has ${c}`),
  );
  rule(
    s.footerTop - s.lastBottom >= MIN_GAP_ABOVE_FOOTER - 0.5,
    `the end of the content (${s.lastBottom}) must be at least ${MIN_GAP_ABOVE_FOOTER}px above the footer (${s.footerTop}), gap is ${s.footerTop - s.lastBottom}px`,
  );
  for (const o of s.overlaps) out.push(`${where} ${o}`);
  return out;
}

// ---- Tests ---------------------------------------------------------------

for (const { name: pageName, address, hasRoles } of TEXT_PAGES) {
for (const { name, width, height } of VIEWPORTS) {
  test(`content layout of the ${pageName} at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);
      const found = violations(await snapshot(page), hasRoles);
      expect(found.length, `content rules broken:\n${found.join('\n')}`).toBe(0);
    });
  });
}
}

// A numbered list must show its numbers and sit in the column like a bulleted one:
// same indent on the left, same space above.
test('numbered list of the After Save Event page is numbered and indented like a bulleted list', async ({ page }) => {
  await retryWhenLive(async () => {
    await page.goto('./articles/after-save-event.html');
    await page.evaluate(() => document.fonts.ready);
    const lists = await page.evaluate(() => {
      const main = document.querySelector('main')!;
      const look = (el: Element) => {
        const style = getComputedStyle(el);
        return {
          listStyleType: style.listStyleType,
          paddingLeft: parseFloat(style.paddingLeft),
          marginTop: parseFloat(style.marginTop),
          fontSize: parseFloat(style.fontSize),
        };
      };
      const ol = main.querySelector('ol');
      const ul = main.querySelector('ul');
      return { ol: ol ? look(ol) : null, ul: ul ? look(ul) : null };
    });
    expect(lists.ol, 'the article must have a numbered list').not.toBeNull();
    expect(lists.ul, 'the article must have a bulleted list').not.toBeNull();
    const { ol, ul } = lists;
    expect(ol!.listStyleType, 'the numbered list must show numbers').toBe('decimal');
    expect(ol!.paddingLeft, 'indent: 1.5em, as the bulleted lists').toBeCloseTo(1.5 * ol!.fontSize, 1);
    expect(ol!.paddingLeft, 'indent equal to the bulleted list').toBeCloseTo(ul!.paddingLeft, 1);
    expect(ol!.marginTop, 'space above: 2.25em, as the bulleted lists').toBeCloseTo(2.25 * ol!.fontSize, 1);
    expect(ol!.marginTop, 'space above equal to the bulleted list').toBeCloseTo(ul!.marginTop, 1);
  });
});
