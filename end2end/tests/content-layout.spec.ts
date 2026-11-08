import { test, expect, type Page } from '@playwright/test';
import { COLUMN_WIDTHS, retryWhenLive, TEXT_PAGES, VIEWPORTS } from './support';

// Enforces that the content (the text in <main>: the curriculum, the licence) is readable at the
// widths of the text column (COLUMN_WIDTHS in support.ts). It checks what the reader sees (sizes, positions, colours), never
// how the page is built.
//
// Rules at those widths:
//   1. no horizontal scroll
//   2. section headings (h2) are larger than body text
//   3. a page with roles has at least one role heading (h3)
//   4. at the end of the page the content ends at least 32px above the sticky footer
//   5. consecutive headings, paragraphs and list items do not overlap
//   6. section (h2) and role (h3) headings stand out from the body text by colour (amber); no line
//      above them. This is a test of its own, tagged @colours so that the dark theme runs it too, at
//      one width only: colours do not depend on the width. (This rule used to check the amber line
//      above h2 and the inverse video of h3: the headings were changed to amber text, the intent
//      "sections and roles stay apart from the text" holds)
//   7. a numbered list (ol) shows its numbers and is indented and spaced like the bulleted
//      lists of the same column (checked on the After Save Event article)
//   8. the markers of lists take the amber accent while the item text keeps its colour: bullets in
//      the tone of lines and frames, numbers in the tone of normal-size text (ADR-003)
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
  h3Count: number;
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
      h3Count: h3s.length,
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
  if (hasRoles) rule(s.h3Count > 0, 'the content must have at least one h3');
  rule(
    s.footerTop - s.lastBottom >= MIN_GAP_ABOVE_FOOTER - 0.5,
    `the end of the content (${s.lastBottom}) must be at least ${MIN_GAP_ABOVE_FOOTER}px above the footer (${s.footerTop}), gap is ${s.footerTop - s.lastBottom}px`,
  );
  for (const o of s.overlaps) out.push(`${where} ${o}`);
  return out;
}

// ---- Tests ---------------------------------------------------------------

for (const { name: pageName, address, hasRoles } of TEXT_PAGES) {
for (const { name, width, height } of COLUMN_WIDTHS) {
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

// Rule 6, at one width: colours do not depend on it. Tagged so that the dark theme runs it too.
for (const { name: pageName, address } of TEXT_PAGES) {
  const { name, width, height } = VIEWPORTS.find((v) => v.name === 'Laptop')!;
  test(`heading colours of the ${pageName} at ${width}x${height} (${name})`, { tag: '@colours' }, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);
      const seen = await page.evaluate(() => {
        const main = document.querySelector('main')!;
        const p = main.querySelector('p');
        const colours = (selector: string) =>
          Array.from(main.querySelectorAll(selector)).map((el) => getComputedStyle(el).color);
        return { pColor: p ? getComputedStyle(p).color : '', h2: colours('h2'), h3: colours('h3') };
      });
      const found: string[] = [];
      seen.h2.forEach((c, i) => {
        if (c === seen.pColor) found.push(`h2 number ${i + 1} must differ in colour from the body text (${seen.pColor}), has ${c}`);
      });
      seen.h3.forEach((c, i) => {
        if (c === seen.pColor) found.push(`h3 number ${i + 1} must differ in colour from the body text (${seen.pColor}), has ${c}`);
      });
      expect(found.length, `heading colours broken:\n${found.join('\n')}`).toBe(0);
    });
  });
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

// The markers (bullets and numbers) of a list are amber, the text of the items is not.
// Bullets are graphics (accent-line); numbers are text (accent-code, the contrast of normal text).
test('list markers take the amber accent and the item text keeps its colour', { tag: '@colours' }, async ({ page }) => {
  await retryWhenLive(async () => {
    await page.goto('./articles/after-save-event.html');
    await page.evaluate(() => document.fonts.ready);
    const seen = await page.evaluate(() => {
      const main = document.querySelector('main')!;
      // The colour a design token resolves to, as the browser computes it.
      const resolve = (token: string) => {
        const probe = document.createElement('span');
        probe.style.color = `var(${token})`;
        main.appendChild(probe);
        const colour = getComputedStyle(probe).color;
        probe.remove();
        return colour;
      };
      const look = (li: Element | null) =>
        li ? { marker: getComputedStyle(li, '::marker').color, text: getComputedStyle(li).color } : null;
      return {
        ol: look(main.querySelector('ol > li')),
        ul: look(main.querySelector('ul > li')),
        accentCode: resolve('--color-accent-code'),
        accentLine: resolve('--color-accent-line'),
      };
    });
    expect(seen.ol, 'the article must have a numbered list').not.toBeNull();
    expect(seen.ul, 'the article must have a bulleted list').not.toBeNull();
    expect(seen.ol!.marker, 'numbers use the tone of normal text').toBe(seen.accentCode);
    expect(seen.ul!.marker, 'bullets use the tone of lines and frames').toBe(seen.accentLine);
    expect(seen.ol!.text, 'the item text keeps its colour').not.toBe(seen.ol!.marker);
    expect(seen.ul!.text, 'the item text keeps its colour').not.toBe(seen.ul!.marker);
  });
});

// The home page's boxes (card frame-home-page-articles-in-boxes), at the widths of the column (the list is inside it): no sideways
// scroll, the date label (on the bottom border line) clears the description and the next box, and
// the border line runs through the middle of the label, at rest and when active.
for (const { name, width, height } of COLUMN_WIDTHS) {
  test(`the home page's boxes and their date labels are laid out well at ${width}x${height} (${name})`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height });

    const check = async () => {
      await page.goto('./');
      const measure = () =>
        page.evaluate(() => {
          const rect = (el: Element) => el.getBoundingClientRect();
          const entries = Array.from(document.querySelectorAll('.article-list > li'));
          return {
            innerWidth: window.innerWidth,
            scrollWidth: document.documentElement.scrollWidth,
            entries: entries.map((li, i) => {
              const label = rect(li.querySelector('time')!);
              const description = rect(li.querySelector('p')!);
              const box = rect(li);
              const border = parseFloat(getComputedStyle(li).borderBottomWidth);
              const next = entries[i + 1] ? rect(entries[i + 1]) : null;
              return {
                borderWidth: border,
                labelAboveDescription: label.top - description.bottom,
                labelAboveNextBox: next ? next.top - label.bottom : null,
                lineCentreOffset: box.bottom - border / 2 - (label.top + label.height / 2),
              };
            }),
          };
        });

      for (const state of ['rest', 'active']) {
        if (state === 'active') await page.getByRole('main').getByRole('listitem').first().hover();
        else await page.mouse.move(0, 0);
        const m = await measure();
        expect(m.scrollWidth, `${state}: no sideways scroll`).toBeLessThanOrEqual(m.innerWidth);
        expect(m.entries.length).toBeGreaterThan(0);
        for (const [i, e] of m.entries.entries()) {
          expect(e.labelAboveDescription, `${state}, entry ${i}: label clears the description`).toBeGreaterThanOrEqual(0);
          if (e.labelAboveNextBox !== null)
            expect(e.labelAboveNextBox, `${state}, entry ${i}: label clears the next box`).toBeGreaterThanOrEqual(0);
          expect(Math.abs(e.lineCentreOffset), `${state}, entry ${i}: line through the label's middle`).toBeLessThanOrEqual(0.5);
        }
      }
    };

    await retryWhenLive(check);
  });
}

// The space between the home page's boxes is 2.25 times the text size and the first box has no
// margin above it (card frame-home-page-articles-in-boxes). The column's own rule for list items
// (0.75em above) must not win over it, whatever the order the stylesheet comes in.
for (const { name, width, height } of COLUMN_WIDTHS) {
  test(`the home page's boxes are 2.25em apart and the first has no margin above at ${width}x${height} (${name})`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height });

    const check = async () => {
      await page.goto('./');
      const m = await page.evaluate(() => {
        const entries = Array.from(document.querySelectorAll('.article-list > li'));
        const first = entries[0].getBoundingClientRect();
        const second = entries[1].getBoundingClientRect();
        const list = document.querySelector('.article-list')!.getBoundingClientRect();
        return {
          gap: second.top - first.bottom,
          fontSize: parseFloat(getComputedStyle(entries[0]).fontSize),
          firstTopFromList: first.top - list.top,
        };
      });
      expect(Math.abs(m.gap - 2.25 * m.fontSize)).toBeLessThanOrEqual(0.5);
      expect(Math.abs(m.firstTopFromList)).toBeLessThanOrEqual(0.5);
    };

    await retryWhenLive(check);
  });
}
