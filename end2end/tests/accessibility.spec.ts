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

// The contrast rules, the only ones the drawn logo is exempt from (see below).
const CONTRAST_RULES = MUST_HAVE_RUN;

test.beforeAll(() => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/index.html'))) {
    throw new Error(
      'target/site/index.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }
});

// The pages built from the shared frame. The "page not found" is opened as
// its own file: only the host serves it for a missing address, so a live run
// leaves it out (the live check in not-found.spec.ts covers it). The article
// pages are not published until the site navigation exists, so a live run
// leaves them out too.
const PAGES = [
  { name: 'home page', address: './', onlyLocal: false },
  { name: 'curriculum page', address: './curriculum.html', onlyLocal: false },
  { name: 'licence page', address: './licence.html', onlyLocal: false },
  { name: 'about page', address: './about.html', onlyLocal: false },
  { name: 'After Save Event article page', address: './articles/after-save-event.html', onlyLocal: true },
  { name: 'hybris PK article page', address: './articles/how-to-generate-hybris-pk-values-without-hybris.html', onlyLocal: true },
  { name: 'page not found', address: './404.html', onlyLocal: true },
];

for (const { name: pageName, address, onlyLocal } of PAGES) {
  for (const { name, width, height } of BANDS) {
    test(`accessibility of the ${pageName} at ${width}x${height} (${name})`, async ({ page }) => {
      test.skip(onlyLocal && live, 'checked locally only, see PAGES');
      await page.setViewportSize({ width, height });

      const check = async () => {
        await page.goto(address);
        await page.evaluate(() => document.fonts.ready);

        const results = await new AxeBuilder({ page }).withTags(TAGS).analyze();

        // WCAG 1.4.3 and 1.4.6 exempt text that is part of a logo or brand
        // name from contrast. The drawn logo (aria-hidden, 16px from md) takes
        // the accent-heading shade, which on the light page reaches 4.61:1,
        // enough for large text only; axe cannot know it is a logo. So its
        // nodes are dropped from the two contrast rules, and from those rules
        // only: every other rule still looks at it, and the contrast rules
        // still look at everything else, the 32px text logo below md included
        // (AxeBuilder.exclude() would drop all rules). A node is the logo when
        // its element, found again in the page by axe's selector, is inside
        // the element marked data-testid="ascii-logo"; a violation goes only
        // when no other node remains.
        const violations = [];
        for (const v of results.violations) {
          if (!CONTRAST_RULES.includes(v.id)) {
            violations.push(v);
            continue;
          }
          const kept = [];
          for (const n of v.nodes) {
            const isLogo =
              n.target.length === 1 &&
              typeof n.target[0] === 'string' &&
              (await page.evaluate(
                (selector) =>
                  document.querySelector(selector)?.closest('[data-testid="ascii-logo"]') != null,
                n.target[0],
              ));
            if (!isLogo) kept.push(n);
          }
          if (kept.length > 0) violations.push({ ...v, nodes: kept });
        }

        const report = violations
          .map((v) => {
            const targets = v.nodes.map((n) => `    ${n.target.join(' ')}`).join('\n');
            return `- ${v.id} (${v.impact ?? 'no impact given'}): ${v.help}\n${targets}`;
          })
          .join('\n');
        expect(violations, `accessibility violations:\n${report}`).toEqual([]);

        const passed = results.passes.map((r) => r.id);
        for (const id of MUST_HAVE_RUN) {
          expect(passed, `axe did not evaluate the rule ${id}`).toContain(id);
        }
      };

      await retryWhenLive(check);
    });
  }
}

// axe finds rule violations; it does not notice a change in what a screen
// reader is given, such as a logo that loses its name, a heading that changes
// level or parts read in another order. So the accessibility tree is also
// compared with the expected one, in two parts. The banner and the footer are
// compared strictly: "/children: equal" makes an extra, missing or reordered
// element fail, where the default would accept extra ones. ("deep-equal" is
// not used: it would also demand the ASCII art inside the wide logo.) The
// content of main is the curriculum, whose text changes at every edit, so
// listing the whole tree would force a change to this test each time; only
// the structure of its headings is compared (see the test below): the
// level-2 headings, in order, and no level-1 heading. How the page sounds (the voice,
// how "Lvk@73r" is spelled out) is not checked here: see the card
// docs/backlog/run-screen-reader-check-on-demand.md.
//
// The logo, in both forms, is one link to the home page named "Lvk@73r, home
// page"; the ASCII art is hidden from the tree, so the screen reader hears
// the name, not the art.
const treeByLogo = (heading: string, footerLink: string) => ({
  text: `
    - /children: equal
    - banner:
      - /children: equal
      - link "Lvk@73r, home page"
      - heading "${heading}" [level=1]
    - main
    - contentinfo:
      - /children: equal
      - paragraph:
        - text: © 2026 Luca Lorenzon ·
        - link "${footerLink}"
  `,
  art: `
    - /children: equal
    - banner:
      - /children: equal
      - link "Lvk@73r, home page"
      - heading "${heading}" [level=1]
    - main
    - contentinfo:
      - /children: equal
      - paragraph:
        - text: © 2026 Luca Lorenzon ·
        - link "${footerLink}"
  `,
});

// The level-2 headings of each page's text, in order. toHaveText with an
// array of strings checks the count and the order of the matching elements.
const TREE_PAGES = [
  {
    name: 'home page',
    address: './',
    trees: treeByLogo('Curriculum Vitae', 'This CV may be copied, not changed'),
    sections: [
      'Summary',
      'Experience',
      'Technical expertise and continuous learning',
      'Core competencies',
      'Education',
      'Certifications and training',
      'Languages',
    ],
  },
  {
    name: 'curriculum page',
    address: './curriculum.html',
    trees: treeByLogo('Curriculum Vitae', 'This CV may be copied, not changed'),
    sections: [
      'Summary',
      'Experience',
      'Technical expertise and continuous learning',
      'Core competencies',
      'Education',
      'Certifications and training',
      'Languages',
    ],
  },
  {
    name: 'about page',
    address: './about.html',
    trees: treeByLogo('About', 'Reuse with attribution'),
    sections: ['My views', 'Reusing what you read'],
  },
  {
    name: 'licence page',
    address: './licence.html',
    trees: treeByLogo('Licence', 'Reuse with attribution'),
    sections: [
      'In short',
      'Articles',
      'Code samples',
      'Curriculum',
      'Material by others',
      "The site's code",
    ],
  },
];

const LOGO_BY_BAND: Record<string, 'text' | 'art'> = {
  'iPhone 15': 'text',
  'iPad Mini': 'art',
  Laptop: 'art',
};

for (const { name: pageName, address, trees, sections } of TREE_PAGES) {
  for (const { name, width, height } of BANDS) {
    test(`accessibility tree of the ${pageName} at ${width}x${height} (${name})`, async ({
      page,
    }) => {
      await page.setViewportSize({ width, height });

      const check = async () => {
        await page.goto(address);
        await page.evaluate(() => document.fonts.ready);
        await expect(page.locator('body')).toMatchAriaSnapshot(trees[LOGO_BY_BAND[name]]);

        const main = page.getByRole('main');
        await expect(main.getByRole('heading', { level: 1 })).toHaveCount(0);
        await expect(main.getByRole('heading', { level: 2 })).toHaveText(sections);
      };

      await retryWhenLive(check);
    });
  }
}
