import { test, expect } from '@playwright/test';
import { COLUMN_WIDTHS, MD, retryWhenLive, TEXT_PAGE } from './support';

// The text column (the <main> of every page, utility `long-text-column` in style/input.css)
// as the reader sees it, checked once, on the style guide's text page (TEXT_PAGE), at the widths
// of the column (COLUMN_WIDTHS in support.ts). It guards the column's behaviour, not each page
// that uses it:
//   - below md the text is 18px, from md it is 20px (the two sizes are design values);
//   - the column fills the screen up to its maximum, 60 characters wide (60ch of its own font:
//     in Literata that is 600px at 18px and 720px at 20px, written here as a comment only,
//     the test measures it; the 584px and 704px of support.ts are the same widths without the
//     column's 8px of padding on each side);
//   - the column is centred: the space left and right of it is equal;
//   - the maximum grows with the text size, so from md it is wider than at 18px.
// Why: docs/board/cards/make-the-text-column-a-component.md

const TEXT_SIZE_BELOW_MD = 18;
const TEXT_SIZE_FROM_MD = 20;

for (const { name, width, height } of COLUMN_WIDTHS) {
  test(`text column of the style guide's text page at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    await retryWhenLive(async () => {
      await page.goto(TEXT_PAGE);
      await page.evaluate(() => document.fonts.ready);

      const measured = await page.evaluate((textSize18) => {
        const main = document.querySelector('main')!;
        // Two probes, removed after measuring: 60ch in the column's own font, and 60ch in the same
        // font at 18px, to see whether the maximum grows with the text size.
        const probe = (fontSize: string) => {
          const el = document.createElement('div');
          el.style.cssText = `width: 60ch; height: 0; ${fontSize ? `font-size: ${fontSize};` : ''}`;
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
          maxWidth: probe(''),
          maxWidthAt18: probe(`${textSize18}px`),
        };
      }, TEXT_SIZE_BELOW_MD);

      const expectedSize = width >= MD ? TEXT_SIZE_FROM_MD : TEXT_SIZE_BELOW_MD;
      const where = `at ${width}px: ${JSON.stringify(measured)}`;
      expect(measured.fontSize, `the text is ${expectedSize}px ${where}`).toBe(expectedSize);

      const expectedWidth = Math.min(measured.screen, measured.maxWidth);
      expect(
        Math.abs(measured.columnWidth - expectedWidth),
        `the column fills the screen up to 60 characters ${where}`,
      ).toBeLessThanOrEqual(1);
      expect(
        Math.abs(measured.left - measured.right),
        `the column is centred ${where}`,
      ).toBeLessThanOrEqual(1);

      if (width >= MD) {
        expect(measured.maxWidth, `the maximum grows with the text size ${where}`).toBeGreaterThan(measured.maxWidthAt18 + 1);
      } else {
        expect(Math.abs(measured.maxWidth - measured.maxWidthAt18), `the maximum at 18px ${where}`).toBeLessThanOrEqual(1);
      }
    });
  });
}
