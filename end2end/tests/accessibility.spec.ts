import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import {
  axeReport,
  BANDS,
  CONTRAST_RULES,
  HEADING_RULES,
  live,
  NET_PAGES,
  retryWhenLive,
  runAxe,
  VIEWPORTS,
  type AxeResults,
} from './support';

// Fails on any accessibility violation that axe-core can find.
//
// Two groups of tests (ADR-004: components are tested once, pages with a net):
//   - the net: every page of the sitemap (NET_PAGES), at one width, the
//     laptop: the axe rules (WCAG 2.0, 2.1 and 2.2 at levels A and AA, plus
//     the three level-AAA rules axe can check, so every colour must reach a
//     contrast of 7:1, not just 4.5:1; see TAGS in support.ts), the structure
//     of the headings (three best-practice rules enabled by name), the colours
//     in both themes, the shape of the banner and footer, and that no
//     identifier is repeated;
//   - the frame: axe on the home page and on the "page not found" at the three
//     width bands.
// The elements that appear only at some widths (the record viewer of the
// tables, the code blocks' wrap toggle) are components, checked by their own
// tests with axe limited to the component: table-forms.spec.ts and
// code-blocks.spec.ts.
//
// Not checked here: most of level AAA, and everything a tool cannot see
// (for example whether alternative texts make sense, or how the page works
// with a screen reader). That is covered by hand, see the card
// docs/backlog/check-accessibility-by-hand.md.

// Rules that must have been evaluated by the tests: if they are missing from
// the results, axe did not look at what they check and an empty list of
// violations would mean nothing. The contrast rules must have passed (the page
// has visible text); the heading rules may also be "inapplicable" (for
// example empty-heading on a page whose headings are never empty is a pass,
// but a rule with nothing to look at is inapplicable).
const MUST_HAVE_RUN = CONTRAST_RULES;

test.beforeAll(() => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/index.html'))) {
    throw new Error(
      'target/site/index.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }
});

const LAPTOP = VIEWPORTS.find(({ name }) => name === 'Laptop')!;

// Two axe tests per page, so that the colours can be checked alone. The
// dark project runs only the tests tagged @colours (see playwright.config.ts),
// and the contrast of the dark theme is the one thing axe can tell about it
// that the light theme cannot. So the contrast rules are a test of their own,
// tagged, and every other rule stays in the untagged one that runs once.
type Run = (page: Page) => Promise<AxeResults>;

// Opens the page and runs axe; returns the violations, with the drawn logo
// dropped from the contrast rules, and the ids of the rules that passed.
async function look(page: Page, address: string, run: Run) {
  await page.goto(address);
  await page.evaluate(() => document.fonts.ready);

  const results = await run(page);

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

  return {
    violations,
    report: axeReport(violations),
    passed: results.passes.map((r) => r.id),
    inapplicable: results.inapplicable.map((r) => r.id),
  };
}

type Size = { width: number; height: number };
type Options = { extraRules?: string[]; onlyLocal?: boolean };

// The skip shared by the tests that open the "page not found".
const ONLY_LOCAL_REASON =
  'checked locally only: only the host serves the "page not found" for a missing address, not-found.spec.ts covers it live';

// What the accessibility test of a page at a width does: every selected rule
// except the two contrast rules. `extraRules` are enabled by name on top of
// the WCAG tags (see runAxe in support.ts). The test( calls are not here but
// in the net and in the frame below, so that each has its own declaration.
async function checkAccessibility(
  page: Page,
  address: string,
  { width, height }: Size,
  { extraRules = [], onlyLocal = false }: Options = {},
) {
  test.skip(onlyLocal && live, ONLY_LOCAL_REASON);
  await page.setViewportSize({ width, height });

  await retryWhenLive(async () => {
    const { violations, report, passed, inapplicable } = await look(page, address, (p) =>
      runAxe(p, 'rules', { extraRules }),
    );
    expect(violations, `accessibility violations:\n${report}`).toEqual([]);

    // Named rules are outside the tags: make sure axe did run them.
    for (const id of extraRules) {
      expect([...passed, ...inapplicable], `axe did not run the rule ${id}`).toContain(id);
    }
  });
}

// What the contrast test of a page at a width does: only the two contrast
// rules, in both themes. Selected by id, so color-contrast-enhanced (disabled
// by default in axe) runs too.
async function checkContrast(
  page: Page,
  address: string,
  { width, height }: Size,
  { onlyLocal = false }: Options = {},
) {
  test.skip(onlyLocal && live, ONLY_LOCAL_REASON);
  await page.setViewportSize({ width, height });

  await retryWhenLive(async () => {
    const { violations, report, passed } = await look(page, address, (p) => runAxe(p, 'contrast'));
    expect(violations, `accessibility violations:\n${report}`).toEqual([]);

    for (const id of MUST_HAVE_RUN) {
      expect(passed, `axe did not evaluate the rule ${id}`).toContain(id);
    }
  });
}

// The net: every page of the sitemap, at one width.
for (const { name: pageName, address } of NET_PAGES) {
  test(`accessibility of the ${pageName} at ${LAPTOP.width}x${LAPTOP.height} (${LAPTOP.name})`, async ({ page }) => {
    await checkAccessibility(page, address, LAPTOP, { extraRules: HEADING_RULES });
  });

  test(`contrast of the ${pageName} at ${LAPTOP.width}x${LAPTOP.height} (${LAPTOP.name})`, { tag: '@colours' }, async ({ page }) => {
    await checkContrast(page, address, LAPTOP);
  });

  // axe finds rule violations; it does not notice a change in what a screen
  // reader is given, such as a logo that loses its name, a heading that changes
  // level or parts read in another order. So the accessibility tree is also
  // checked, for its shape. The exact texts (the page heading, the footer's
  // licence phrase) are checked per page by Rust's unit tests (FooterLicence
  // in src/page_frame.rs), so the browser checks only the shape, the same on
  // every page: the banner holds, strictly and in order ("/children: equal"
  // makes an extra, missing or reordered element fail), the logo link and one
  // level-1 heading with any text; main exists; the footer holds the copyright
  // and one link with any text. The content of main is long text that changes
  // at every edit, so it is not listed: only that it holds no level-1 heading.
  // How the page sounds (the voice, how "Lvk@73r" is spelled out) is not
  // checked here: see the card docs/backlog/run-screen-reader-check-on-demand.md.
  //
  // The logo, in both forms, is one link to the home page named "Lvk@73r, home
  // page"; the ASCII art is hidden from the tree, so the screen reader hears
  // the name, not the art.
  test(`shape of the ${pageName} at ${LAPTOP.width}x${LAPTOP.height} (${LAPTOP.name})`, async ({ page }) => {
    await page.setViewportSize({ width: LAPTOP.width, height: LAPTOP.height });

    await retryWhenLive(async () => {
      await page.goto(address);
      await page.evaluate(() => document.fonts.ready);
      await expect(page.locator('body')).toMatchAriaSnapshot(`
        - banner:
          - /children: equal
          - link "Lvk@73r, home page"
          - heading /.+/ [level=1]
        - main
        - contentinfo:
          - /children: equal
          - paragraph:
            - text: /© 2026 Luca Lorenzon ·/
            - link /.+/
      `);
      await expect(page.getByRole('main').getByRole('heading', { level: 1 })).toHaveCount(0);
      await expect(page.getByRole('heading', { level: 1 })).toHaveCount(1);
    });
  });

  // axe's general rule duplicate-id was retired with WCAG 2.2 (criterion 4.1.1
  // was removed); duplicate-id-aria, for the identifiers that attributes such
  // as aria-labelledby point to, still runs. This check guards the first
  // component that introduces ids: the pages have none today.
  test(`no identifier is used twice in the ${pageName}`, async ({ page }) => {
    await page.setViewportSize({ width: LAPTOP.width, height: LAPTOP.height });

    await retryWhenLive(async () => {
      await page.goto(address);
      const repeated = await page.evaluate(() => {
        const seen = new Map<string, number>();
        for (const el of document.querySelectorAll('[id]')) {
          seen.set(el.id, (seen.get(el.id) ?? 0) + 1);
        }
        return [...seen].filter(([, count]) => count > 1).map(([id, count]) => `${id} (${count} times)`);
      });
      expect(repeated, 'identifiers used by more than one element').toEqual([]);
    });
  });
}

const FRAME_PAGES = [
  { label: 'home page', address: './', onlyLocal: false },
  { label: 'page not found', address: './404.html', onlyLocal: true },
];

// The frame, at the three width bands. The header changes form with the band
// (text logo, drawn logo, title below or beside the logo); the frame is the
// same on every page, so one content page plus the "page not found" suffice.
// The "page not found" is opened as its own file: only the host serves it for
// a missing address, so a live run leaves it out.
for (const band of BANDS) {
  const where = `at ${band.width}x${band.height} (${band.name})`;
  for (const { label, address, onlyLocal } of FRAME_PAGES) {
    test(`accessibility of the frame, ${label} ${where}`, async ({ page }) => {
      await checkAccessibility(page, address, band, { extraRules: HEADING_RULES, onlyLocal });
    });

    test(`contrast of the frame, ${label} ${where}`, { tag: '@colours' }, async ({ page }) => {
      await checkContrast(page, address, band, { onlyLocal });
    });
  }
}
