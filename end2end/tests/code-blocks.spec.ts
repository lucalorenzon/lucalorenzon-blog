import { test, expect, type Locator, type Page } from '@playwright/test';
import { axeReport, COLUMN_WIDTHS, CONTRAST_RULES, VIEWPORTS, runAxe } from './support';

// Code blocks (Markdown code rendered as a component), on the style guide's
// code page, whose cases are written for these tests
// (content/style-guide/code.md):
//   - each case shows or hides the "wrap" toggle as written for it, at every
//     column width: the long line and the indented cases scroll sideways and
//     show it, the short block and the block with a blank line fit and do not;
//     checking it wraps the lines, so nothing scrolls any more
//   - the toggle is reached with Tab and switched with Space
//   - a wrapped line continues 4 columns deeper than its own indent, spaces
//     or tabs (the tab case must really wrap at 320)
//   - copying the code gives the block's code, without the toggle's text,
//     whatever the case: tabs and the blank line included
//   - axe finds no accessibility violation inside the code blocks, and the
//     contrast holds (at two widths, see the end of the file)
// It checks what the reader sees, never how the page is built. A case is found
// by the title of its section: a block belongs to the section whose heading
// comes before it.

const PAGE = './style-guide/code.html';

// What each case puts to the test, and what it must show at each column
// width. Measured on the built page: the text column is 704 px wide from 768
// px up, and the block's content 686 px at most; the long line is about 1700
// px (1890 px at the larger text), the nested and the tab cases 720 to 920 px,
// the short block and the blank-line block fit even at 320 (286 px).
type Case = { name: string; heading: string; scrollsAt: (width: number) => boolean };
const CASES: Case[] = [
  { name: 'long line', heading: 'A line longer than the column', scrollsAt: () => true },
  { name: 'short block', heading: 'A block that always fits', scrollsAt: () => false },
  { name: 'nested indentation', heading: 'Nested indentation', scrollsAt: () => true },
  { name: 'tabs', heading: 'Indentation made of tabs', scrollsAt: () => true },
  { name: 'blank line', heading: 'A blank line inside a block', scrollsAt: () => false },
];

async function open(page: Page, address: string) {
  await page.goto(address);
  await page.evaluate(() => document.fonts.ready);
}

// The block of a case: the first one after its section's heading, and before
// the next heading. A renamed heading or a missing block fails with the case's
// name.
async function blockOf(page: Page, { name, heading }: Case): Promise<Locator> {
  const quoted = JSON.stringify(heading);
  const block = page.locator(
    `xpath=//h2[normalize-space()=${quoted}]/following-sibling::div[contains(@class,'code-block')][1][preceding-sibling::h2[1][normalize-space()=${quoted}]]`,
  );
  expect(await block.count(), `case "${name}": no code block under the heading "${heading}"`).toBe(1);
  return block;
}

for (const { name, width, height } of COLUMN_WIDTHS) {
  test(`code cases at ${width}x${height} (${name}): each one scrolls or fits as written`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await open(page, PAGE);

    for (const c of CASES) {
      const block = await blockOf(page, c);
      const scroller = block.locator('.code-scroller');
      const toggle = block.locator('label');
      const overflows = () => scroller.evaluate((el) => el.scrollWidth > el.clientWidth);
      const scrolls = c.scrollsAt(width);

      expect(await overflows(), `${c.name}: ${scrolls ? 'scrolls sideways' : 'fits the column'}`).toBe(scrolls);
      expect(await toggle.isVisible(), `${c.name}: toggle ${scrolls ? 'visible' : 'hidden'}`).toBe(scrolls);

      if (scrolls) {
        await block.locator('input').check();
        expect(await overflows(), `${c.name}: wrapped code does not scroll sideways`).toBe(false);
      }
    }

    expect(
      await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth),
      'the page does not scroll sideways',
    ).toBe(false);
  });
}

test('toggle of the code cases: Tab reaches it and Space switches it', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 568 });
  await open(page, PAGE);

  const input = (await blockOf(page, CASES[0])).locator('input');
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

test('wrapped lines of the code cases continue 4 columns deeper than their indent', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 568 });
  await open(page, PAGE);

  for (const c of CASES) {
    const block = await blockOf(page, c);
    if (!(await block.locator('label').isVisible())) continue;
    const { problems, wrapped } = await block.evaluate((el) => {
      const out: string[] = [];
      let wrappedLines = 0;
      const input = el.querySelector('input') as HTMLInputElement;
      input.checked = true;
      const advance = (() => {
        const probe = document.createElement('span');
        probe.style.cssText = 'position:absolute;visibility:hidden;white-space:pre;font:inherit';
        probe.textContent = '0'.repeat(100);
        el.querySelector('code')!.appendChild(probe);
        const w = probe.getBoundingClientRect().width / 100;
        probe.remove();
        return w;
      })();
      for (const line of Array.from(el.querySelectorAll('.code-line'))) {
        const text = line.textContent!.replace(/\n$/, '');
        // The indent in columns: a tab goes to the next multiple of 4.
        let indent = 0;
        for (const ch of text) {
          if (ch === ' ') indent += 1;
          else if (ch === '\t') indent = (Math.floor(indent / 4) + 1) * 4;
          else break;
        }
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
        if (xs.length > 1) wrappedLines++;
        xs.slice(1).forEach((x, i) => {
          if (Math.abs(x - (indent + 4)) > 0.1) {
            out.push(`segment ${i + 2} of "${text.slice(0, 30)}" starts at column ${x.toFixed(2)}, expected ${indent + 4}`);
          }
        });
        if (xs.length > 0 && Math.abs(xs[0]) > 0.1) out.push(`first segment of "${text.slice(0, 30)}" starts at column ${xs[0].toFixed(2)}`);
      }
      input.checked = false;
      return { problems: out, wrapped: wrappedLines };
    });
    expect(problems, c.name).toEqual([]);
    // The check above is worth something only if the case really wraps.
    if (c.name === 'nested indentation' || c.name === 'tabs') {
      expect(wrapped, `${c.name}: has an indented line that wraps at 320`).toBeGreaterThan(0);
    }
  }
});

test('copying the code of the cases gives the code as written', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 568 });
  await open(page, PAGE);

  for (const wrapped of [false, true]) {
    for (const c of CASES) {
      const block = await blockOf(page, c);
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
      expect(selected.replace(/\n$/, ''), `${c.name}${wrapped ? ' (wrapped)' : ''}`).toBe(source.replace(/\n$/, ''));
      expect(selected).not.toContain('wrap');
      expect(selected).not.toMatch(/\n\n\n/);
    }
  }
});

// Accessibility of the components, with axe limited to the component.
// Per ADR-004 a component is checked once, on the case that stresses it most:
// the style guide's code page shows, at both widths, blocks with the wrap
// toggle and blocks without it. The contrast rules are a test of their own,
// tagged @colours, to run in both themes.
const AXE_WIDTHS = VIEWPORTS.filter(({ name }) => ['iPhone SE', 'iPad Mini'].includes(name));

for (const { name, width, height } of AXE_WIDTHS) {
  const where = `of the style guide's code page at ${width}x${height} (${name})`;

  test(`accessibility of the code blocks ${where}`, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await open(page, PAGE);
    const { violations } = await runAxe(page, 'rules', { scope: '.code-block' });
    expect(violations, `accessibility violations:\n${axeReport(violations)}`).toEqual([]);
  });

  test(`contrast of the code blocks ${where}`, { tag: '@colours' }, async ({ page }) => {
    await page.setViewportSize({ width, height });
    await open(page, PAGE);
    const { violations, passes } = await runAxe(page, 'contrast', { scope: '.code-block' });
    expect(violations, `accessibility violations:\n${axeReport(violations)}`).toEqual([]);
    for (const id of CONTRAST_RULES) {
      expect(passes.map((r) => r.id), `axe did not evaluate the rule ${id}`).toContain(id);
    }
  });
}
