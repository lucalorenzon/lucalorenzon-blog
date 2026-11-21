import { test, expect, type Page } from '@playwright/test';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { retryWhenLive } from './support';

// The site has three fonts, named by role in the stylesheet (the @theme block the
// build writes from config/fonts.txt and config/fonts/): --font-text for the
// running text, --font-code for the code and --font-logo for the logo. Everything else refers to the roles, never to a
// font's name, so changing one value changes every place that plays the role.
// These tests check that: the stylesheet defines exactly the three roles, and
// each role reaches the elements that play it. Before measuring, the test replaces
// the three roles on the page with three made-up, different names, so it checks the
// link from the element to its role whatever fonts the configuration holds (two roles
// may share a font): the font the browser computes for the element must be the name
// of its role.
// Card: docs/board/cards/give-the-fonts-three-roles-in-the-stylesheet.md.

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
    cases: [['the text role\'s sample', '.sample-text-normal-400']],
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
    cases: [['the code role\'s sample', '.sample-code-normal-400']],
  },
  {
    role: '--font-logo',
    page: './style-guide/fonts.html',
    width: 1280,
    cases: [
      ['the logo role\'s sample', '.sample-logo-normal-400'],
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
        await page.addStyleTag({
          content: `:root { --font-text: "probe-text" !important; --font-code: "probe-code" !important; --font-logo: "probe-logo" !important; }`,
        });
        const expected = await fontOfRole(page, role);
        expect(expected, `${role} is made-up`).toContain(`probe-${role.replace('--font-', '')}`);
        expect(await fontOf(page, selector)).toBe(expected);
      });
    });
  }
}

// The role tests above compare family names, which stay the same when a font's
// file is missing. This one checks that every file the stylesheet declares is
// served and loads. The number of faces is read from the built stylesheet (one
// per @font-face rule, written by the build from config/fonts/), so a font added
// to the configuration needs no change here. The style guide's fonts page is
// used, but the fonts are loaded explicitly, so the test does not depend on what
// the page shows.
test('every font file the stylesheet declares loads', async ({ page }) => {
  await retryWhenLive(async () => {
    const response = await page.request.get('./style.css');
    expect(response.ok()).toBe(true);
    const declared = ((await response.text()).match(/@font-face\s*\{/g) ?? []).length;
    expect(declared, 'the stylesheet declares at least one font face').toBeGreaterThan(0);

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
    expect(faces, 'a font face is declared for each @font-face rule').toHaveLength(declared);
    expect(faces.filter((face) => face.status !== 'loaded'), 'font faces not loaded').toEqual([]);
  });
});

// Each sample of the fonts page stands for one file of a font in the configuration
// (config/fonts.txt and config/fonts/<folder>.txt). Its variant is named
// sample-<role>-<style>-<weight>, written by the build; this checks what the browser draws:
// the family of the sample's role and the style and weight of the file whose name is the
// sample's label. The expected values are read from the configuration, so a font or a file
// added there is checked with no change here.
// Card: docs/board/cards/build-the-fonts-page-from-the-configuration.md.
const CONFIG = join(__dirname, '..', '..', 'config');

function settings(path: string): [string, string][] {
  return readFileSync(path, 'utf8')
    .split('\n')
    .map((line) => line.trim())
    .filter((line) => line !== '' && !line.startsWith('#'))
    .map((line) => {
      const at = line.indexOf('=');
      return [line.slice(0, at).trim(), line.slice(at + 1).trim()];
    });
}

// The files of the font of each role: name (the label), style and weight.
function configuredFiles(): Map<string, { name: string; style: string; weight: string }[]> {
  const byRole = new Map<string, { name: string; style: string; weight: string }[]>();
  for (const [role, folder] of settings(join(CONFIG, 'fonts.txt'))) {
    const files = new Map<string, Record<string, string>>();
    for (const [key, value] of settings(join(CONFIG, 'fonts', `${folder}.txt`))) {
      const [label, field] = key.split('.');
      if (field === undefined) continue;
      files.set(label, { ...files.get(label), [field]: value });
    }
    byRole.set(
      role,
      [...files.values()].map((file) => ({ name: file.name, style: file.style, weight: file.weight })),
    );
  }
  return byRole;
}

test('each sample of the fonts page is drawn with the family, style and weight of its file', async ({
  page,
}) => {
  const configured = configuredFiles();
  await retryWhenLive(async () => {
    await page.goto('./style-guide/fonts.html');
    await page.evaluate(() => document.fonts.ready);
    const samples = await page.evaluate(() =>
      [...document.querySelectorAll('main p[class^="sample-"]')].map((p) => {
        const style = getComputedStyle(p);
        return {
          cls: p.className,
          label: p.nextElementSibling?.textContent ?? '',
          family: style.fontFamily,
          style: style.fontStyle,
          weight: style.fontWeight,
        };
      }),
    );
    expect(samples.length, 'the page has samples').toBeGreaterThan(0);
    for (const sample of samples) {
      const role = /^sample-(text|code|logo)-/.exec(sample.cls)?.[1];
      expect(role, `${sample.cls} is named from a role`).toBeDefined();
      const file = configured.get(role!)?.find((f) => f.name === sample.label);
      expect(file, `${sample.cls} stands for the file labelled ${sample.label}`).toBeDefined();
      expect(sample.family, `${sample.cls} family`).toBe(await fontOfRole(page, `--font-${role}` as Role));
      expect(sample.style, `${sample.cls} style`).toBe(file!.style);
      expect(sample.weight, `${sample.cls} weight`).toBe(file!.weight);
    }
  });
});
