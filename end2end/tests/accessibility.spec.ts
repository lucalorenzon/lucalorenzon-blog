import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import { live, retryWhenLive } from './support';

// Fails on any accessibility violation that axe-core can find.
//
// Checked: every axe rule tagged for WCAG 2.0, 2.1 and 2.2 at levels A and
// AA, plus the three level-AAA rules axe can check (enhanced contrast, links
// with the same text leading to the same purpose, no automatic page
// refresh). So every colour must reach a contrast of 7:1, not just 4.5:1.
//
// A second check, below, compares the accessibility tree (what a screen
// reader is given: roles, names, levels, order) with the expected one.
//
// Not checked here: most of level AAA, and everything a tool cannot see
// (for example whether alternative texts make sense, or how the page works
// with a screen reader). That is covered by hand, see the card
// docs/backlog/check-accessibility-by-hand.md.

// axe checks only what is shown, and the page shows different elements per
// width band, so there is one test per band, with the same names and sizes
// used in header-layout.spec.ts.
const BANDS = [
  { name: 'iPhone 15', width: 393, height: 659 }, // text logo
  { name: 'iPad Mini', width: 768, height: 1024 }, // ASCII-art logo, title below the line
  { name: 'Laptop', width: 1280, height: 720 }, // title beside the logo
];

// Rules axe has disabled by default (color-contrast-enhanced is one) still run
// when selected by tag: axe skips a rule only if it has none of the selected
// tags or carries an excluded tag (deprecated, experimental).
const TAGS = ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa', 'wcag2aaa'];

// Rules that must have been evaluated and passed: the page has visible text,
// so if they are missing from the passes, axe did not look at the colours at
// all and an empty list of violations would mean nothing.
const MUST_HAVE_RUN = ['color-contrast', 'color-contrast-enhanced'];

test.beforeAll(() => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/index.html'))) {
    throw new Error(
      'target/site/index.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }
});

for (const { name, width, height } of BANDS) {
  test(`accessibility at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    const check = async () => {
      await page.goto('./');
      await page.evaluate(() => document.fonts.ready);

      const results = await new AxeBuilder({ page }).withTags(TAGS).analyze();

      const report = results.violations
        .map((v) => {
          const targets = v.nodes.map((n) => `    ${n.target.join(' ')}`).join('\n');
          return `- ${v.id} (${v.impact ?? 'no impact given'}): ${v.help}\n${targets}`;
        })
        .join('\n');
      expect(results.violations, `accessibility violations:\n${report}`).toEqual([]);

      const passed = results.passes.map((r) => r.id);
      for (const id of MUST_HAVE_RUN) {
        expect(passed, `axe did not evaluate the rule ${id}`).toContain(id);
      }
    };

    await retryWhenLive(check);
  });
}

// axe finds rule violations; it does not notice a change in what a screen
// reader is given, such as a logo that loses its name, a heading that changes
// level or parts read in another order. So the accessibility tree is also
// compared with the expected one. "/children: equal" on every container makes
// the match strict: an extra, missing or reordered element fails, where the
// default would accept extra ones. ("deep-equal" is not used: it would also
// demand the ASCII art inside the wide logo.) How the page sounds (the voice,
// how "Lvk@73r" is spelled out) is not checked here: see the card
// docs/backlog/run-screen-reader-check-on-demand.md.
//
// The wide logo is an image whose accessible name is "Lvk@73r" and whose text
// content is the ASCII art, which is left out of the template on purpose: the
// name is what the screen reader reads.
const TREE_BY_LOGO = {
  text: `
    - /children: equal
    - banner:
      - /children: equal
      - text: Lvk@73r
      - heading "Home Page" [level=1]
    - main:
      - /children: equal
      - paragraph: Sito in costruzione
    - contentinfo:
      - /children: equal
      - paragraph: © Luca Lorenzon 2026
  `,
  art: `
    - /children: equal
    - banner:
      - /children: equal
      - img "Lvk@73r"
      - heading "Home Page" [level=1]
    - main:
      - /children: equal
      - paragraph: Sito in costruzione
    - contentinfo:
      - /children: equal
      - paragraph: © Luca Lorenzon 2026
  `,
};

const EXPECTED_TREE: Record<string, string> = {
  'iPhone 15': TREE_BY_LOGO.text,
  'iPad Mini': TREE_BY_LOGO.art,
  Laptop: TREE_BY_LOGO.art,
};

for (const { name, width, height } of BANDS) {
  test(`accessibility tree at ${width}x${height} (${name})`, async ({ page }) => {
    await page.setViewportSize({ width, height });

    const check = async () => {
      await page.goto('./');
      await page.evaluate(() => document.fonts.ready);
      await expect(page.locator('body')).toMatchAriaSnapshot(EXPECTED_TREE[name]);
    };

    await retryWhenLive(check);
  });
}
