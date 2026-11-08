import { test, expect, type Locator, type Page } from '@playwright/test';
import { COLUMN_WIDTHS, TABLE_ADDRESSES, live } from './support';

// Every Markdown table is rendered in two forms inside one wrapper: the
// table, and a "record viewer" (the names of the columns stay still on the
// left, a box on the right shows one record at a time). The page shows the
// table while its columns get a readable width and the viewer below that;
// tables never scroll sideways. Checked on every page of the built site that
// has tables, found by walking the built site (see support.ts), at the widths
// of the text column (COLUMN_WIDTHS), whatever form the table has there:
//   - exactly one of the two forms is displayed
//   - the table is not wider than its wrapper
//   - in the viewer, for each record: every row of the box is as tall as, and
//     level with, the matching row of the label column; the number row reads
//     "k / n"
//   - the keyboard reaches the box and moves it one record (once per page, at
//     320x568, where the viewers are shown: the keyboard does not behave
//     differently with the width once the viewer is shown)
//   - the page does not scroll sideways
// A live run has no folder to walk and skips them. It checks what the reader
// sees, never how the page is built (the class names below only find the
// parts of the viewer).

const SINGLE_TABLE = {
  // The six-column table of the hybris PK article is the one whose form the
  // card annotate-markdown-for-wide-content fixes at two widths.
  address: './articles/how-to-generate-hybris-pk-values-without-hybris.html',
  columns: 6,
};

test.beforeAll(() => {
  if (!live && TABLE_ADDRESSES.length === 0) {
    throw new Error('no page with a table found in target/site: run scripts/build-site.sh from the repository root before this test.');
  }
});

async function gotoReady(page: Page, address: string) {
  await page.goto(address);
  await page.evaluate(() => document.fonts.ready);
}

async function wrappersOf(page: Page): Promise<Locator[]> {
  const all = page.locator('.table-with-viewer');
  return Array.from({ length: await all.count() }, (_, i) => all.nth(i));
}

// Which forms of a wrapper are displayed.
async function formsShown(wrapper: Locator) {
  return wrapper.evaluate((el) => ({
    table: getComputedStyle(el.querySelector('table')!).display !== 'none',
    viewer: el.querySelector('.record-viewer') !== null && getComputedStyle(el.querySelector('.record-viewer')!).display !== 'none',
  }));
}

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

for (const address of TABLE_ADDRESSES) {
  for (const { name, width, height } of COLUMN_WIDTHS) {
    const where = `of ${address} at ${width}x${height} (${name})`;

    test(`exactly one form of each table ${where} is shown, and the page does not scroll sideways`, async ({ page }) => {
      test.skip(live, 'checked locally only: the pages come from the built folder');
      await page.setViewportSize({ width, height });
      await gotoReady(page, address);

      const wrappers = await wrappersOf(page);
      expect(wrappers.length, 'the page has tables').toBeGreaterThan(0);
      for (const wrapper of wrappers) {
        const { table, viewer } = await formsShown(wrapper);
        expect(table !== viewer, `exactly one form is displayed (table: ${table}, viewer: ${viewer})`).toBe(true);
        if (table) {
          const { wrapperWidth, tableWidth } = await wrapper.evaluate((el) => ({
            wrapperWidth: el.getBoundingClientRect().width,
            tableWidth: el.querySelector('table')!.getBoundingClientRect().width,
          }));
          expect(tableWidth, 'the table is not wider than its wrapper').toBeLessThanOrEqual(wrapperWidth + 0.5);
        }
      }
      const { scrollWidth, clientWidth } = await page.evaluate(() => ({
        scrollWidth: document.documentElement.scrollWidth,
        clientWidth: document.documentElement.clientWidth,
      }));
      expect(scrollWidth, 'the page does not scroll sideways').toBeLessThanOrEqual(clientWidth);
    });

    test(`every record of the viewers ${where} lines up with the labels and is numbered`, async ({ page }) => {
      test.skip(live, 'checked locally only: the pages come from the built folder');
      await page.setViewportSize({ width, height });
      await gotoReady(page, address);

      for (const wrapper of await wrappersOf(page)) {
        if (!(await formsShown(wrapper)).viewer) continue;
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
      }
    });
  }

  const { name, width, height } = COLUMN_WIDTHS[0];
  test(`the keyboard reaches the viewers of ${address} at ${width}x${height} (${name}) and moves one record`, async ({ page }) => {
    test.skip(live, 'checked locally only: the pages come from the built folder');
    await page.setViewportSize({ width, height });
    await gotoReady(page, address);

    for (const wrapper of await wrappersOf(page)) {
      if (!(await formsShown(wrapper)).viewer) continue;
      const box = wrapper.getByRole('region');
      const records = await box.getByRole('group').count();

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
      if (records < 2) continue;
      await page.keyboard.press('ArrowDown');
      await expect
        .poll(() =>
          box.evaluate((el) => (el.querySelectorAll('[role="group"]')[1] as HTMLElement).offsetTop - el.scrollTop),
        )
        .toBeLessThan(1);
      await page.keyboard.press('PageDown');
      await expect
        .poll(() =>
          box.evaluate((el) => {
            const pane = (el.querySelectorAll('[role="group"]')[Math.min(2, el.querySelectorAll('[role="group"]').length - 1)] as HTMLElement);
            return Math.abs(pane.offsetTop - el.scrollTop);
          }),
        )
        .toBeLessThan(1);
    }
  });
}

// The card fixes the form of the hybris PK article's six-column table at two
// widths: a table on a tablet, a viewer one pixel below.
test(`the six-column table of ${SINGLE_TABLE.address} is a table at 768x1024`, async ({ page }) => {
  test.skip(live, 'checked locally only: the pages come from the built folder');
  await page.setViewportSize({ width: 768, height: 1024 });
  await gotoReady(page, SINGLE_TABLE.address);
  const wrapper = page.locator('.table-with-viewer', { has: page.locator(`tr:first-child > :nth-child(${SINGLE_TABLE.columns})`) }).last();
  expect(await formsShown(wrapper)).toEqual({ table: true, viewer: false });
});

test(`the six-column table of ${SINGLE_TABLE.address} is a viewer at 767x800`, async ({ page }) => {
  test.skip(live, 'checked locally only: the pages come from the built folder');
  await page.setViewportSize({ width: 767, height: 800 });
  await gotoReady(page, SINGLE_TABLE.address);
  const wrapper = page.locator('.table-with-viewer', { has: page.locator(`tr:first-child > :nth-child(${SINGLE_TABLE.columns})`) }).last();
  expect(await formsShown(wrapper)).toEqual({ table: false, viewer: true });
});
