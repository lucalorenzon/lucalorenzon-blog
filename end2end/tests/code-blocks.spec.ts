import { test, expect, type Page } from '@playwright/test';
import { COLUMN_WIDTHS } from './support';

// Code blocks (Markdown code rendered as a component):
//   - the "wrap" toggle is visible if and only if the code scrolls sideways
//   - checking it wraps the lines, so nothing scrolls any more
//   - the toggle is reached with Tab and switched with Space
//   - a wrapped line continues 4 columns deeper than its own indent
//   - copying the code gives the article's code, without the toggle's text
// It checks what the reader sees, never how the page is built.

const ARTICLES = [
  { name: 'After Save Event', address: './articles/after-save-event.html' },
  { name: 'hybris PK', address: './articles/how-to-generate-hybris-pk-values-without-hybris.html' },
];

async function open(page: Page, address: string) {
  await page.goto(address);
  await page.evaluate(() => document.fonts.ready);
}

for (const { name: articleName, address } of ARTICLES) {
  for (const { name, width, height } of COLUMN_WIDTHS) {
    test(`code blocks of ${articleName} at ${width}x${height} (${name}): the toggle shows exactly when the code scrolls`, async ({ page }) => {
      await page.setViewportSize({ width, height });
      await open(page, address);

      const blocks = page.locator('.code-block');
      const count = await blocks.count();
      expect(count, 'the article has code blocks').toBeGreaterThan(0);

      for (let i = 0; i < count; i++) {
        const block = blocks.nth(i);
        const scroller = block.locator('.code-scroller');
        const toggle = block.locator('label');
        const overflows = () => scroller.evaluate((el) => el.scrollWidth > el.clientWidth);
        const label = `block ${i + 1}`;

        expect(await toggle.isVisible(), `${label}: toggle visible iff the code scrolls sideways`).toBe(await overflows());

        if (await toggle.isVisible()) {
          await block.locator('input').check();
          expect(await overflows(), `${label}: wrapped code does not scroll sideways`).toBe(false);
        }
      }

      expect(
        await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth),
        'the page does not scroll sideways',
      ).toBe(false);
    });
  }

  test(`toggle of ${articleName}: Tab reaches it and Space switches it`, async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 568 });
    await open(page, address);

    const input = page.locator('.code-block:has(label:visible) input').first();
    await input.scrollIntoViewIfNeeded();
    // Tab from the start of the page until the checkbox has focus.
    await page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());
    let reached = false;
    for (let i = 0; i < 200 && !reached; i++) {
      await page.keyboard.press('Tab');
      reached = await input.evaluate((el) => el === document.activeElement);
    }
    expect(reached, 'Tab reaches the toggle').toBe(true);
    expect(await input.isChecked()).toBe(false);
    await page.keyboard.press('Space');
    expect(await input.isChecked()).toBe(true);
    await page.keyboard.press('Space');
    expect(await input.isChecked()).toBe(false);
  });

  test(`wrapped lines of ${articleName} continue 4 columns deeper than their indent`, async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 568 });
    await open(page, address);

    const problems = await page.evaluate(() => {
      const out: string[] = [];
      for (const block of Array.from(document.querySelectorAll('.code-block:has(label)'))) {
        const input = block.querySelector('input') as HTMLInputElement;
        if (getComputedStyle(block.querySelector('label')!).display === 'none') continue;
        input.checked = true;
        const advance = (() => {
          const probe = document.createElement('span');
          probe.style.cssText = 'position:absolute;visibility:hidden;white-space:pre;font:inherit';
          probe.textContent = '0'.repeat(100);
          block.querySelector('code')!.appendChild(probe);
          const w = probe.getBoundingClientRect().width / 100;
          probe.remove();
          return w;
        })();
        for (const line of Array.from(block.querySelectorAll('.code-line'))) {
          const text = line.textContent!.replace(/\n$/, '');
          const indent = text.length - text.trimStart().length;
          const range = document.createRange();
          range.selectNodeContents(line);
          const rects = Array.from(range.getClientRects()).filter((r) => r.width > 0);
          // One x per rendered row: the leftmost rect of each distinct top.
          const rows = new Map<number, number>();
          for (const r of rects) {
            const top = Math.round(r.top);
            rows.set(top, Math.min(rows.get(top) ?? Infinity, r.left));
          }
          const origin = line.getBoundingClientRect().left;
          const xs = Array.from(rows.entries()).sort((a, b) => a[0] - b[0]).map(([, x]) => (x - origin) / advance);
          xs.slice(1).forEach((x, i) => {
            if (Math.abs(x - (indent + 4)) > 0.1) {
              out.push(`segment ${i + 2} of "${text.slice(0, 30)}" starts at column ${x.toFixed(2)}, expected ${indent + 4}`);
            }
          });
          if (xs.length > 0 && Math.abs(xs[0]) > 0.1) out.push(`first segment of "${text.slice(0, 30)}" starts at column ${xs[0].toFixed(2)}`);
        }
        input.checked = false;
      }
      return out;
    });
    expect(problems).toEqual([]);
  });

  test(`copying the code of ${articleName} gives the article's code`, async ({ page }) => {
    await page.setViewportSize({ width: 320, height: 568 });
    await open(page, address);

    const blocks = page.locator('.code-block');
    const count = await blocks.count();
    for (const wrapped of [false, true]) {
      for (let i = 0; i < count; i++) {
        const block = blocks.nth(i);
        if (wrapped && (await block.locator('label').isVisible())) await block.locator('input').check();
        const { selected, source } = await block.evaluate((el) => {
          const code = el.querySelector('code')!;
          const range = document.createRange();
          range.selectNodeContents(code);
          const selection = window.getSelection()!;
          selection.removeAllRanges();
          selection.addRange(range);
          return { selected: selection.toString(), source: Array.from(code.querySelectorAll('.code-line')).map((l) => l.textContent).join('') };
        });
        // What the browser copies: the code, with no toggle text, no extra blank
        // line. Chromium leaves out the last line break of the selection and WebKit
        // keeps it: only that one is allowed to differ.
        expect(selected.replace(/\n$/, ''), `block ${i + 1}${wrapped ? ' (wrapped)' : ''}`).toBe(source.replace(/\n$/, ''));
        expect(selected).not.toContain('wrap');
        expect(selected).not.toMatch(/\n\n\n/);
      }
    }
  });
}
