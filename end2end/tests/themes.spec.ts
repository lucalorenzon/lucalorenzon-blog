import { test, expect } from '@playwright/test';
import { ADDRESSES, BANDS, live } from './support';

// Why this test exists: the dark project runs only the tests tagged @colours
// (see playwright.config.ts), and everything else runs in the light theme
// alone. That is valid only while the light theme changes colours and nothing
// else. This test fails if a theme starts to change the size or the position
// of anything (a different font weight, a padding, a hidden element), because
// then a colour-free test passing in light would say nothing about dark.
//
// For every page of the built site and at each width band, the page is opened
// in two fresh browser contexts, one preferring dark and one preferring light,
// and the box of every element is compared. It is one test, not one per page,
// so the number of tests does not grow with the articles. A live run has no
// folder to walk and skips it.

type Box = { tag: string; text: string; left: number; top: number; width: number; height: number };
type Layout = { boxes: Box[]; scrollWidth: number; scrollHeight: number };

test('the dark and the light theme differ only in colours', async ({ browser }) => {
  test.skip(live, 'checked locally only: the pages come from the built folder');
  test.setTimeout(120_000);
  expect(ADDRESSES.length, 'target/site not found: run scripts/build-site.sh before this test').toBeGreaterThan(0);

  const layoutOf = async (colorScheme: 'dark' | 'light', width: number, height: number, address: string) => {
    const context = await browser.newContext({
      colorScheme,
      viewport: { width, height },
      baseURL: test.info().project.use.baseURL,
    });
    try {
      const page = await context.newPage();
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);
      return await page.evaluate((): Layout => {
        const round = (n: number) => Math.round(n * 100) / 100;
        const boxes = Array.from(document.querySelectorAll('body, body *')).map((el) => {
          const r = el.getBoundingClientRect();
          return {
            tag: el.tagName.toLowerCase(),
            text: (el.textContent ?? '').trim().replace(/\s+/g, ' ').slice(0, 30),
            left: round(r.left),
            top: round(r.top),
            width: round(r.width),
            height: round(r.height),
          };
        });
        return {
          boxes,
          scrollWidth: document.documentElement.scrollWidth,
          scrollHeight: document.documentElement.scrollHeight,
        };
      });
    } finally {
      await context.close();
    }
  };

  const differences: string[] = [];
  for (const address of ADDRESSES) {
    for (const { width, height } of BANDS) {
      const where = `${address} at ${width}x${height}`;
      const dark = await layoutOf('dark', width, height, address);
      const light = await layoutOf('light', width, height, address);
      const box = (b: Box | undefined) =>
        b ? `${b.left},${b.top} ${b.width}x${b.height}` : 'missing';

      if (dark.scrollWidth !== light.scrollWidth || dark.scrollHeight !== light.scrollHeight) {
        differences.push(
          `${where}: page size dark ${dark.scrollWidth}x${dark.scrollHeight}, light ${light.scrollWidth}x${light.scrollHeight}`,
        );
      }
      const count = Math.max(dark.boxes.length, light.boxes.length);
      let shown = 0;
      for (let i = 0; i < count && shown < 3; i++) {
        const d = dark.boxes[i];
        const l = light.boxes[i];
        if (d && l && d.tag === l.tag && box(d) === box(l)) continue;
        differences.push(
          `${where}: <${(d ?? l).tag}> "${(d ?? l).text}" dark ${box(d)}, light ${box(l)}`,
        );
        shown++;
      }
    }
  }
  expect(differences, `the themes differ in more than colours:\n${differences.join('\n')}`).toEqual([]);
});
