import { test, expect, type Locator, type Page } from '@playwright/test';
import { axeReport, COLUMN_PHONES, COLUMN_WIDTHS, CONTRAST_RULES, VIEWPORTS, runAxe } from './support';

// Every Markdown table is rendered in two forms inside one wrapper: the
// table, and a "record viewer" (the names of the columns stay still on the
// left, a box on the right shows one record at a time). The page shows the
// table while its columns get a readable width and the viewer below that;
// tables never scroll sideways. Checked on the style guide's tables page,
// whose cases are written for these tests (content/style-guide/tables.md);
// the articles are no longer test pages for the tables:
//   - at the widths of the text column (COLUMN_WIDTHS), for every case:
//     exactly one of the two forms is displayed, the table is not wider than
//     its wrapper, and the page does not scroll sideways
//   - in the case of rows of different heights, at the phone widths, where it
//     is a viewer (it is a table at the other widths of the column): for each record, every row of the box is as tall as, and
//     level with, the matching row of the label column; the number row reads
//     "k / n"
//   - at a phone's width, the keyboard reaches the six-column table's viewer and moves it
//     one record (ArrowDown), then one more (PageDown)
//   - axe finds no accessibility violation inside the tables, in both forms
//     (at 1024, where the page shows both, and at 1920, where it shows tables only; see the end of the file)
// It checks what the reader sees, never how the page is built (the class names
// below only find the parts of the viewer). A case is found by the title of
// its section: a table belongs to the section whose heading comes before it.

const PAGE = './style-guide/tables.html';

// What each case puts to the test. The threshold of each case, in em of its
// own text, is the number in the class `@max-[Tem]/table:viewer-shown` of its
// wrapper in the built page. The column grows with the page up to 1366 px,
// where it is 704 px wide (text 20 px, 35.2 em) and stays so, and the text
// grows with it, so a table is a viewer where its threshold is above the
// column's width in em. Measured on the built page, the six-column table and the
// rows of different heights are viewers at the three phones, and tables at 1366 and
// above; between, the form of the six-column table changes with the width
// and is not checked, only that exactly one form is shown.
type Case = { name: string; heading: string };
const SIMPLE: Case = { name: 'simple table', heading: 'A table that always fits' };
const SIX_COLUMNS: Case = { name: 'six columns', heading: 'Six columns' };
const HEIGHTS: Case = { name: 'rows of different heights', heading: 'Rows of different heights' };
const RANGE: Case = { name: 'range with an en dash', heading: 'A range with an en dash' };
const HYPHEN: Case = { name: 'word with a hyphen', heading: 'A word with a hyphen' };
const CASES: Case[] = [SIMPLE, SIX_COLUMNS, HEIGHTS, RANGE, HYPHEN];

async function gotoReady(page: Page) {
  await page.goto(PAGE);
  await page.evaluate(() => document.fonts.ready);
}

// The wrapper of a case: the first one after its section's heading, and before
// the next heading. A renamed heading or a missing table fails with the case's
// name.
async function wrapperOf(page: Page, { name, heading }: Case): Promise<Locator> {
  const quoted = JSON.stringify(heading);
  const wrapper = page.locator(
    `xpath=//h2[normalize-space()=${quoted}]/following-sibling::div[contains(@class,'table-with-viewer')][1][preceding-sibling::h2[1][normalize-space()=${quoted}]]`,
  );
  expect(await wrapper.count(), `case "${name}": no table under the heading "${heading}"`).toBe(1);
  return wrapper;
}

// Which forms of a wrapper are displayed.
async function formsShown(wrapper: Locator) {
  return wrapper.evaluate((el) => ({
    table: getComputedStyle(el.querySelector('table')!).display !== 'none',
    viewer: el.querySelector('.record-viewer') !== null && getComputedStyle(el.querySelector('.record-viewer')!).display !== 'none',
  }));
}

const TABLE = { table: true, viewer: false };
const VIEWER = { table: false, viewer: true };

// Scrolls the box to the k-th record (from 0) and waits for it to settle.
async function showRecord(box: Locator, k: number) {
  await box.evaluate((el, k) => {
    const pane = el.querySelectorAll('[role="group"]')[k] as HTMLElement;
    el.scrollTo({ top: pane.offsetTop, behavior: 'instant' });
  }, k);
  await expect
    .poll(
      () =>
        box.evaluate((el, k) => {
          const pane = el.querySelectorAll('[role="group"]')[k] as HTMLElement;
          return Math.abs(el.scrollTop - pane.offsetTop) < 1;
        }, k),
      { message: `the box settles on record ${k + 1}` },
    )
    .toBe(true);
}

for (const { name, width, height } of COLUMN_WIDTHS) {
  const where = `at ${width}x${height} (${name})`;

  test(`exactly one form of each table ${where} is shown, and the page does not scroll sideways`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await gotoReady(page);

    for (const c of CASES) {
      const wrapper = await wrapperOf(page, c);
      const { table, viewer } = await formsShown(wrapper);
      expect(table !== viewer, `${c.name}: exactly one form is displayed (table: ${table}, viewer: ${viewer})`).toBe(true);
      if (table) {
        const { wrapperWidth, tableWidth } = await wrapper.evaluate((el) => ({
          wrapperWidth: el.getBoundingClientRect().width,
          tableWidth: el.querySelector('table')!.getBoundingClientRect().width,
        }));
        expect(tableWidth, `${c.name}: the table is not wider than its wrapper`).toBeLessThanOrEqual(wrapperWidth + 0.5);
      }
    }
    const { scrollWidth, clientWidth } = await page.evaluate(() => ({
      scrollWidth: document.documentElement.scrollWidth,
      clientWidth: document.documentElement.clientWidth,
    }));
    expect(scrollWidth, 'the page does not scroll sideways').toBeLessThanOrEqual(clientWidth);
  });
}

// The rows of different heights are a viewer at every phone width (its
// threshold is 33.643 em, above the phones' columns, at most 412 px at 18.2 px
// text): there the rows of a record must line up with their labels.
for (const { name, width, height } of COLUMN_PHONES) {
  test(`every record of the rows of different heights at ${width}x${height} (${name}) lines up with the labels and is numbered`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await gotoReady(page);

    const wrapper = await wrapperOf(page, HEIGHTS);
    expect(await formsShown(wrapper), 'rows of different heights: a record viewer').toEqual(VIEWER);
    const box = wrapper.getByRole('region');
    const records = await box.getByRole('group').count();
    expect(records, 'the viewer has records').toBeGreaterThan(0);

    for (let k = 0; k < records; k++) {
      await showRecord(box, k);

      // Each block's rows are measured from its own first row, so the
      // blocks need not start at the same height.
      const mismatches = await wrapper.evaluate(
        (el, k) => {
          const rowsOf = (rows: Element[]) => {
            const first = rows[0].getBoundingClientRect().top;
            return rows.map((row) => {
              const rect = row.getBoundingClientRect();
              return { top: rect.top - first, height: rect.height };
            });
          };
          const labels = rowsOf([...el.querySelectorAll('.viewer-rows > .viewer-row')]);
          const pane = el.querySelectorAll('[role="group"]')[k];
          const shown = rowsOf([...pane.querySelectorAll(':scope > .viewer-row')]);
          const out: string[] = [];
          if (labels.length !== shown.length) out.push(`${labels.length} label rows, ${shown.length} rows in the record`);
          labels.forEach((label, i) => {
            const row = shown[i];
            if (!row) return;
            if (Math.abs(label.top - row.top) > 0.5 || Math.abs(label.height - row.height) > 0.5) {
              out.push(`row ${i + 1}: label at ${label.top} high ${label.height}, value at ${row.top} high ${row.height}`);
            }
          });
          return out;
        },
        k,
      );
      expect(mismatches, `record ${k + 1}: rows of the labels and of the value`).toEqual([]);

      // The number row shows one line of its strip: the one inside the window.
      await expect
        .poll(
          () =>
            wrapper.evaluate((el) => {
              const window = el.querySelector('.viewer-count-window')!.getBoundingClientRect();
              const spans = [...el.querySelectorAll('.viewer-count-strip > *')];
              return spans
                .filter((span) => {
                  const rect = span.getBoundingClientRect();
                  const middle = (rect.top + rect.bottom) / 2;
                  return middle > window.top && middle < window.bottom;
                })
                .map((span) => span.textContent);
            }),
          { message: `the number row reads ${k + 1} / ${records}` },
        )
        .toEqual([`${k + 1} / ${records}`]);
    }
  });
}

// At the width of an iPhone 15, where the six-column table is a viewer.
const KEYBOARD_PHONE = COLUMN_PHONES.find(({ name }) => name === 'iPhone 15')!;

test(`the keyboard reaches the viewer of the six-column table at ${KEYBOARD_PHONE.width}x${KEYBOARD_PHONE.height} and moves one record, then one more`, async ({ page }) => {
  await page.setViewportSize({ width: KEYBOARD_PHONE.width, height: KEYBOARD_PHONE.height });
  await gotoReady(page);

  const wrapper = await wrapperOf(page, SIX_COLUMNS);
  expect(await formsShown(wrapper), 'six columns: a record viewer').toEqual(VIEWER);
  const box = wrapper.getByRole('region');

  // Tab from the top of the page until the box has focus.
  await page.evaluate(() => window.scrollTo(0, 0));
  await page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());
  let focused = false;
  for (let presses = 0; presses < 60 && !focused; presses++) {
    await page.keyboard.press('Tab');
    focused = await box.evaluate((el) => el === document.activeElement);
  }
  expect(focused, 'Tab reaches the box of the viewer').toBe(true);

  const top = () => box.evaluate((el) => el.scrollTop);
  expect(await top(), 'the box starts at the first record').toBeLessThan(1);
  // The distance of the k-th record from the top of the box.
  const offsetOf = (k: number) =>
    box.evaluate((el, k) => Math.abs((el.querySelectorAll('[role="group"]')[k] as HTMLElement).offsetTop - el.scrollTop), k);
  await page.keyboard.press('ArrowDown');
  await expect.poll(() => offsetOf(1), { message: 'ArrowDown shows the second record' }).toBeLessThan(1);
  await page.keyboard.press('PageDown');
  await expect.poll(() => offsetOf(2), { message: 'PageDown shows the third record' }).toBeLessThan(1);
});

// Accessibility of the components, with axe limited to the component.
// Per ADR-004 a component is checked once, on the case that stresses it most:
// the style guide's tables page shows both forms at the width of the iPad Mini in
// landscape, 1024 (the simple table and the two of single values are tables, the
// six-column table and the rows of different heights are viewers), and tables only at
// Full HD, 1920, past the end of the text's range. The contrast rules are a test of their
// own, tagged @colours, to run in both themes.
const AXE_WIDTHS = VIEWPORTS.filter(({ name }) => ['iPad Mini landscape', 'Full HD'].includes(name));

for (const { name, width, height } of AXE_WIDTHS) {
  const where = `of the style guide's tables page at ${width}x${height} (${name})`;

  test(`accessibility of the tables ${where}`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await gotoReady(page);
    const { violations } = await runAxe(page, 'rules', { scope: '.table-with-viewer' });
    expect(violations, `accessibility violations:\n${axeReport(violations)}`).toEqual([]);
  });

  test(`contrast of the tables ${where}`, { tag: '@colours' }, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await gotoReady(page);
    const { violations, passes } = await runAxe(page, 'contrast', { scope: '.table-with-viewer' });
    expect(violations, `accessibility violations:\n${axeReport(violations)}`).toEqual([]);
    for (const id of CONTRAST_RULES) {
      expect(passes.map((r) => r.id), `axe did not evaluate the rule ${id}`).toContain(id);
    }
  });
}
