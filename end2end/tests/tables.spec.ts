import { test, expect } from '@playwright/test';
import { COLUMN_PHONES } from './support';

// A table cell whose text has no whitespace is a single value, not running
// text, so it stays on one line. Browsers treat an en dash, and a hyphen
// followed by a letter, as a place to break, so a range such as "63–44" or a
// word such as "read-only" used to be split in a narrow column while its
// neighbour had room to spare. Cells with whitespace wrap as usual and are not
// checked.
//
// The rule is checked on the style guide's tables page, whose two cases are
// written for it (content/style-guide/tables.md): a column of ranges with an
// en dash and a column of words with a hyphen, each narrow and beside a column
// with room to spare, so that without the rule the browser breaks them (at 320
// the ranges and the words, measured). It runs at the phone widths of the text
// column (COLUMN_PHONES: 320, 393 and 412), where columns are narrow. A case is found
// by the title of its section: a table belongs to the section whose heading
// comes before it.
// It checks what the reader sees, never how the page is built.

const PHONES = COLUMN_PHONES;
const CASES = [
  { name: 'range with an en dash', heading: 'A range with an en dash' },
  { name: 'word with a hyphen', heading: 'A word with a hyphen' },
];

for (const { name, width, height } of PHONES) {
  test(`single-value table cells of the style guide's tables page at ${width}x${height} (${name}) stay on one line`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await page.goto('./style-guide/tables.html');
    await page.evaluate(() => document.fonts.ready);

    for (const c of CASES) {
      const quoted = JSON.stringify(c.heading);
      const table = page.locator(
        `xpath=//h2[normalize-space()=${quoted}]/following-sibling::div[contains(@class,'table-with-viewer')][1][preceding-sibling::h2[1][normalize-space()=${quoted}]]//table`,
      );
      expect(await table.count(), `case "${c.name}": no table under the heading "${c.heading}"`).toBe(1);
      expect(await table.isVisible(), `case "${c.name}": the table form is shown`).toBe(true);

      const { single, split } = await table.evaluate((el) => {
        const single = [...el.querySelectorAll('th, td')].filter((cell) => {
          const text = (cell as HTMLElement).innerText.trim();
          return text !== '' && !/\s/.test(text);
        });
        const split = single.flatMap((cell) => {
          const range = document.createRange();
          range.selectNodeContents(cell);
          const lines = new Set([...range.getClientRects()].map((r) => Math.round(r.top)));
          return lines.size > 1 ? [(cell as HTMLElement).innerText.trim()] : [];
        });
        return { single: single.length, split };
      });
      expect(single, `case "${c.name}": the table has single-value cells`).toBeGreaterThan(0);
      expect(split, `case "${c.name}": single-value cells split over several lines`).toEqual([]);
    }
  });
}
