import { test, expect } from '@playwright/test';
import { COLUMN_WIDTHS, retryWhenLive, TEXT_PAGE, TEXT_RANGE, textSizeAt, VIEWPORTS } from './support';

// The text column (the <main> of every page, utility `long-text-column` in style/input.css)
// as the reader sees it, checked once, on the style guide's text page (TEXT_PAGE), at the widths
// of the column (COLUMN_WIDTHS in support.ts) and at Full HD. It guards the column's behaviour,
// not each page that uses it:
//   - the text grows with the page's width in a straight line, from 18px at 320px to 20px at
//     1366px, and stays at 18px below and at 20px above (TEXT_RANGE in support.ts); the tests'
//     browser hides the scrollbars, so the page's width is the window's;
//   - the column fills the screen up to its maximum, 60 characters wide (60ch of its own font:
//     in Literata that is 600px at 18px and 720px at 20px, written here as a comment only,
//     the test measures it);
//   - the column is centred: the space left and right of it is equal;
//   - the maximum grows with the text size, so at 1366px it is wider than at 843px;
//   - what the column passes on to the components inside it, the code block and the table, is its
//     own size in pixels: see the end of the file.
// Why: docs/board/cards/make-the-text-column-a-component.md and
// docs/board/cards/let-the-column-text-grow-within-an-injected-range.md

// How far the measured size may be from the straight line, in pixels.
const SIZE_TOLERANCE = 0.05;

// The widths of the column and, besides, one past the end of the range.
const FULL_HD = VIEWPORTS.find(({ name }) => name === 'Full HD')!;
const WIDTHS = [...COLUMN_WIDTHS, FULL_HD];

for (const { name, width, height } of WIDTHS) {
  test(`text column of the style guide's text page at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto(TEXT_PAGE);
      await page.evaluate(() => document.fonts.ready);

      const measured = await page.evaluate(() => {
        const main = document.querySelector('main')!;
        // A probe, removed after measuring: 60ch in the column's own font.
        const probe = () => {
          const el = document.createElement('div');
          el.style.cssText = 'width: 60ch; height: 0;';
          main.appendChild(el);
          const w = el.getBoundingClientRect().width;
          el.remove();
          return w;
        };
        const rect = main.getBoundingClientRect();
        const screen = document.documentElement.clientWidth;
        return {
          fontSize: parseFloat(getComputedStyle(main).fontSize),
          screen,
          left: rect.left,
          right: screen - rect.right,
          columnWidth: rect.width,
          maxWidth: probe(),
        };
      });

      const expectedSize = textSizeAt(width);
      const where = `at ${width}px: ${JSON.stringify(measured)}`;
      expect(
        Math.abs(measured.fontSize - expectedSize),
        `the text follows the range's line, ${expectedSize}px ${where}`,
      ).toBeLessThanOrEqual(SIZE_TOLERANCE);

      const expectedWidth = Math.min(measured.screen, measured.maxWidth);
      expect(
        Math.abs(measured.columnWidth - expectedWidth),
        `the column fills the screen up to 60 characters ${where}`,
      ).toBeLessThanOrEqual(1);
      expect(
        Math.abs(measured.left - measured.right),
        `the column is centred ${where}`,
      ).toBeLessThanOrEqual(1);
    });
  });
}

// The maximum grows with the text size: the column is wider where the text is larger, though the
// phones, where it fills the screen, cannot show it.
test('the column of the text page is wider at 1366px than at 843px, as its text is larger', async ({ page }) => {
  const maxWidthAt = async (width: number) => {
    await page.setViewportSize({ width, height: 800 });
    await page.goto(TEXT_PAGE);
    await page.evaluate(() => document.fonts.ready);
    return page.evaluate(() => document.querySelector('main')!.getBoundingClientRect().width);
  };
  const middle = await maxWidthAt(843);
  const end = await maxWidthAt(TEXT_RANGE.to);
  expect(end, `the column at ${TEXT_RANGE.to}px (${end}) is wider than at 843px (${middle})`).toBeGreaterThan(middle + 1);
});

// What the column passes on to what is inside it. The code block and the table are containers
// themselves, and use the column's size (the custom property --long-text-size) to space and to
// measure themselves. The property is registered, so they receive it already computed in pixels;
// not registered, each would compute the expression against its own width and get a size of its
// own (or the expression itself, as text). The check is run where the text is neither 18px nor
// 20px, so a size that falls back to a bound is noticed too. The code and the tables pages hold
// both components.
for (const { page: address, inside, component } of [
  { page: './style-guide/code.html', inside: '.code-block .code-scroller', component: 'code block' },
  { page: './style-guide/tables.html', inside: '.table-with-viewer table', component: 'table' },
]) {
  test(`the size the text column passes on to the ${component} is its own, in pixels, at 843x800`, async ({ page }) => {
    await page.setViewportSize({ width: 843, height: 800 });
    await retryWhenLive(async () => {
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);
      const sizes = await page.evaluate((selector) => {
        const column = parseFloat(getComputedStyle(document.querySelector('main')!).fontSize);
        const passed = getComputedStyle(document.querySelector(selector)!).getPropertyValue('--long-text-size').trim();
        return { column, passed };
      }, inside);
      expect(sizes.passed, `the ${component} receives a length in pixels: ${JSON.stringify(sizes)}`).toMatch(/^[\d.]+px$/);
      expect(sizes.column, `the column's text at 843px is between 18 and 20: ${JSON.stringify(sizes)}`).toBeGreaterThan(TEXT_RANGE.min + 0.5);
      expect(
        Math.abs(parseFloat(sizes.passed) - sizes.column),
        `the ${component} receives the column's size: ${JSON.stringify(sizes)}`,
      ).toBeLessThanOrEqual(0.01);
    });
  });
}
