import { test, expect } from '@playwright/test';
import { ADDRESSES, VIEWPORTS, live } from './support';

// A table cell whose text has no whitespace is a single value, not running
// text, so it stays on one line. Browsers treat an en dash as a place to
// break, so a range such as "63–44" used to be split in a narrow column
// while its neighbour had room to spare. Cells with whitespace wrap as usual
// and are not checked.
//
// The rule is checked on every page of the built site that has tables, at
// the phone widths, where columns are narrow. The pages are found by
// walking the built site, so a new page is covered without being listed;
// a live run has no folder to walk and skips the test.
//
// If no page had a table, this test would check nothing. This is known and
// left to a backlog card on making the browser tests check behaviour rather
// than articles.
// It checks what the reader sees, never how the page is built.

const PHONES = VIEWPORTS.filter(({ width }) => width < 768);

test.beforeAll(() => {
  if (!live && ADDRESSES.length === 0) {
    throw new Error('target/site not found: run scripts/build-site.sh from the repository root before this test.');
  }
});

for (const address of ADDRESSES) {
  for (const { name, width, height } of PHONES) {
    test(`single-value table cells of ${address} at ${width}x${height} (${name}) stay on one line`, async ({ page }) => {
      test.skip(live, 'checked locally only: the pages come from the built folder');
      await page.setViewportSize({ width, height });
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);

      const split = await page.evaluate(() =>
        [...document.querySelectorAll('th, td')].flatMap((cell) => {
          const text = (cell as HTMLElement).innerText.trim();
          if (text === '' || /\s/.test(text)) return [];
          const range = document.createRange();
          range.selectNodeContents(cell);
          const lines = new Set([...range.getClientRects()].map((r) => Math.round(r.top)));
          return lines.size > 1 ? [text] : [];
        }),
      );
      expect(split, 'single-value cells split over several lines').toEqual([]);
    });
  }
}
