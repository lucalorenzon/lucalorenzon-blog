import { test, expect, type Page } from '@playwright/test';
import { retryWhenLive } from './support';

// The site has three fonts, named by role in the stylesheet (style/input.css,
// block @theme): --font-text for the running text, --font-code for the code and
// --font-logo for the logo. Everything else refers to the roles, never to a
// font's name, so changing one value changes every place that plays the role.
// These tests check that: the stylesheet defines exactly the three roles, and
// each role reaches the elements that play it, comparing the font the browser
// computes for the element with the one it computes for the role's variable.
// Card: docs/done/2026-11-16_give-the-fonts-three-roles-in-the-stylesheet.md.

type Role = '--font-text' | '--font-code' | '--font-logo';

// The font the browser computes for an element that takes the role's variable,
// so quotes and spaces are written as the browser writes them.
async function fontOfRole(page: Page, role: Role): Promise<string> {
  return page.evaluate((name) => {
    const probe = document.createElement('span');
    probe.style.fontFamily = `var(${name})`;
    document.body.appendChild(probe);
    const family = getComputedStyle(probe).fontFamily;
    probe.remove();
    return family;
  }, role);
}

async function fontOf(page: Page, selector: string): Promise<string> {
  const element = page.locator(selector).first();
  await expect(element, `${selector} is on the page`).toBeVisible();
  return element.evaluate((el) => getComputedStyle(el).fontFamily);
}

test('the stylesheet defines the three font roles and no other font', async ({ page }) => {
  await retryWhenLive(async () => {
    const response = await page.request.get('./style.css');
    expect(response.ok()).toBe(true);
    const css = await response.text();

    // Tailwind's own fonts are gone, as variables and as classes.
    expect(css).not.toMatch(/--font-(sans|serif|mono)\b/);
    expect(css).not.toMatch(/\.font-(sans|serif|mono)\b/);

    // The font variables the theme defines: the weights are not fonts.
    const defined = new Set(
      [...css.matchAll(/(?<![\w-])--font-([a-z]+)\s*:/g)]
        .map((match) => match[1])
        .filter((name) => name !== 'weight'),
    );
    expect([...defined].sort()).toEqual(['code', 'logo', 'text']);
  });
});

const ROLES: { role: Role; page: string; width: number; cases: [string, string][] }[] = [
  {
    role: '--font-text',
    page: './style-guide/text.html',
    width: 1280,
    cases: [
      ['a paragraph', 'main p'],
      ['a list item', 'main ul li'],
      ['the page title', 'h1'],
    ],
  },
  {
    role: '--font-text',
    page: './style-guide/tables.html',
    width: 1280,
    cases: [['a table cell', 'main td']],
  },
  {
    role: '--font-text',
    page: './style-guide/fonts.html',
    width: 1280,
    cases: [['the Literata sample', '.sample-literata']],
  },
  {
    role: '--font-code',
    page: './style-guide/code.html',
    width: 1280,
    cases: [['a code block', 'pre code']],
  },
  {
    role: '--font-code',
    page: './style-guide/text.html',
    width: 1280,
    cases: [['code inside a line', 'main p code, main li code']],
  },
  {
    role: '--font-code',
    page: './style-guide/fonts.html',
    width: 1280,
    cases: [['the Hack sample', '.sample-hack']],
  },
  {
    role: '--font-logo',
    page: './style-guide/fonts.html',
    width: 1280,
    cases: [
      ['the pixel sample', '.sample-pixel'],
      ['the logo drawn in characters', '[data-testid="ascii-logo"]'],
    ],
  },
  {
    role: '--font-logo',
    page: './style-guide/fonts.html',
    width: 393,
    cases: [['the text logo', '.text-logo-large']],
  },
];

for (const { role, page: address, width, cases } of ROLES) {
  for (const [what, selector] of cases) {
    test(`${what} takes the font of ${role}, ${address.replace('./style-guide/', '')} at ${width}px`, async ({
      page,
    }) => {
      await page.setViewportSize({ width, height: 800 });
      await retryWhenLive(async () => {
        await page.goto(address);
        await page.evaluate(() => document.fonts.ready);
        const expected = await fontOfRole(page, role);
        expect(expected, `${role} is defined on the page`).not.toBe('');
        expect(await fontOf(page, selector)).toBe(expected);
      });
    });
  }
}

test('the three font roles are three different fonts', async ({ page }) => {
  await retryWhenLive(async () => {
    await page.goto('./style-guide/fonts.html');
    const fonts = await Promise.all(
      (['--font-text', '--font-code', '--font-logo'] as const).map((role) => fontOfRole(page, role)),
    );
    expect(new Set(fonts).size).toBe(3);
  });
});

// The role tests above compare family names, which stay the same when a font's
// file is missing. This one checks that every file the stylesheet declares is
// served and loads. The style guide's fonts page is used, but the fonts are
// loaded explicitly, so the test does not depend on what the page shows.
const DECLARED_FONT_FACES = 6; // Literata x4, Hack x1, IBM VGA 8x16 x1

test('every font file the stylesheet declares loads', async ({ page }) => {
  await retryWhenLive(async () => {
    await page.goto('./style-guide/fonts.html');
    const faces = await page.evaluate(async () => {
      const all = [...document.fonts];
      await Promise.allSettled(all.map((face) => face.load()));
      await document.fonts.ready;
      return all.map((face) => ({
        name: `${face.family} ${face.style} ${face.weight}`,
        status: face.status,
      }));
    });
    expect(faces, 'a font face is declared for each @font-face rule').toHaveLength(DECLARED_FONT_FACES);
    expect(faces.filter((face) => face.status !== 'loaded'), 'font faces not loaded').toEqual([]);
  });
});
