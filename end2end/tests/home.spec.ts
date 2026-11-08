import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test, type Locator } from '@playwright/test';
import { live, retryWhenLive } from './support';

test.beforeAll(() => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/index.html'))) {
    throw new Error(
      'target/site/index.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }
});

test('home page shows header, body and footer, and loads stylesheet, fonts and favicon', async ({
  page,
}) => {
  const check = async () => {
    const waitFor = (suffix: string) => {
      const response = page.waitForResponse(
        (r) => new URL(r.url()).pathname.endsWith(suffix),
        { timeout: 10_000 },
      );
      response.catch(() => {}); // a failed attempt must not leave an unhandled rejection
      return response;
    };
    const stylesheet = waitFor('/style.css');
    const font = waitFor('.woff');
    const textFont = waitFor('.woff2');
    await page.goto('./');
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Home Page', {
      timeout: 5_000,
    });
    await expect(page.getByRole('main').getByRole('heading', { level: 2 })).toHaveCount(2);
    await expect(page.getByText('© 2026 Luca Lorenzon')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Reuse with attribution' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Lvk@73r, home page' })).toHaveCount(1);
    expect((await stylesheet).status()).toBe(200);
    expect((await font).status()).toBe(200);
    expect((await textFont).status()).toBe(200);
    const favicon = await page.request.get(new URL('favicon.ico', page.url()).href);
    expect(favicon.status()).toBe(200);
  };

  await retryWhenLive(check);
});

// The list shows the newest article first; each title is a link to its article.
const LISTED_ARTICLES = [
  { title: 'After Save Event', url: /articles\/after-save-event\.html$/ },
  {
    title: 'How to generate hybris PK values without hybris',
    url: /articles\/how-to-generate-hybris-pk-values-without-hybris\.html$/,
  },
];

test('the home page lists the articles, newest first, with description and date', async ({
  page,
}) => {
  const check = async () => {
    await page.goto('./');
    const entries = page.getByRole('main').getByRole('listitem');
    await expect(entries).toHaveCount(2);
    // No bullets and no indent: the column's own rule for lists must not win.
    const list = page.getByRole('main').getByRole('list');
    await expect(list).toHaveCSS('list-style-type', 'none');
    await expect(list).toHaveCSS('padding-left', '0px');
    await expect(entries.nth(0)).toContainText('After Save Event');
    await expect(entries.nth(0)).toContainText('SAP hybris change events system');
    await expect(entries.nth(0).locator('time')).toHaveText('30 April 2023');
    await expect(entries.nth(0).locator('time')).toHaveAttribute('datetime', '2023-04-30');
    await expect(entries.nth(1)).toContainText('How to generate hybris PK values without hybris');
    await expect(entries.nth(1).locator('time')).toHaveText('25 April 2023');
    await expect(entries.nth(1).locator('time')).toHaveAttribute('datetime', '2023-04-25');
  };

  await retryWhenLive(check);
});

for (const { title, url } of LISTED_ARTICLES) {
  test(`from the home page, the title "${title}" opens the article`, async ({ page }) => {
    const check = async () => {
      await page.goto('./');
      await page.getByRole('main').getByRole('link', { name: title }).click();
      await expect(page.getByRole('heading', { level: 1 })).toHaveText(title, { timeout: 5_000 });
      await expect(page).toHaveURL(url);
    };

    await retryWhenLive(check);
  });
}

test('the footer link of the home page leads to the about page', async ({ page }) => {
  const check = async () => {
    await page.goto('./');
    await page.getByRole('link', { name: 'Reuse with attribution' }).click();
    await expect(page).toHaveTitle('About - Luca Lorenzon', { timeout: 5_000 });
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('About');
    await expect(page.getByRole('heading', { level: 2, name: 'My views' })).toBeVisible();
  };

  await retryWhenLive(check);
});

test('from the about page, the text links lead to the licence page and to the curriculum', async ({
  page,
}) => {
  const check = async () => {
    await page.goto('./about.html');
    await page.getByRole('main').getByRole('link', { name: 'licence page' }).click();
    await expect(page).toHaveTitle('Licence - Luca Lorenzon', { timeout: 5_000 });
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Licence');

    await page.goto('./about.html');
    await page.getByRole('main').getByRole('link', { name: 'my curriculum' }).click();
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Curriculum Vitae', {
      timeout: 5_000,
    });
    await expect(page).toHaveURL(/curriculum\.html$/);
  };

  await retryWhenLive(check);
});

// The logo has two forms (text below 768px, ASCII art from 768px); the link is
// the same, and only one form is shown at a time.
for (const { name, width, height } of [
  { name: 'iPhone 15', width: 393, height: 659 },
  { name: 'Laptop', width: 1280, height: 720 },
]) {
  test(`the logo of the licence page leads home, by click and by keyboard, at ${width}x${height} (${name})`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height });

    const check = async () => {
      await page.goto('./licence.html');
      const logo = page.getByRole('link', { name: 'Lvk@73r, home page' });
      await expect(logo).toHaveCount(1);

      // Keyboard: the logo is the first stop of the page and shows a focus ring.
      await page.keyboard.press('Tab');
      await expect(logo).toBeFocused();
      await expect(logo.locator('> div:visible')).toHaveCount(1);
      await expect(logo).toHaveCSS('outline-style', 'solid');
      await page.keyboard.press('Enter');
      await expect(page.getByRole('heading', { level: 1 })).toHaveText('Home Page', {
        timeout: 5_000,
      });

      // Pointer.
      await page.goto('./licence.html');
      await logo.click();
      await expect(page.getByRole('heading', { level: 1 })).toHaveText('Home Page');
    };

    await retryWhenLive(check);
  });
}

// Each article is framed in a box (card frame-home-page-articles-in-boxes): the whole box leads to
// the article, the border thickens when the box is active, the date is a label on the bottom border.
const FIRST_ARTICLE = LISTED_ARTICLES[0];

// The point at the centre of an element. The description is covered by the title link's stretched
// area, so Playwright's own click and hover (which wait for the element to receive the event)
// would wait forever: the mouse is moved to the point instead, as a reader's would be.
async function centreOf(locator: Locator): Promise<{ x: number; y: number }> {
  const box = (await locator.boundingBox())!;
  return { x: box.x + box.width / 2, y: box.y + box.height / 2 };
}

test('in each entry the date comes last, after the description', async ({ page }) => {
  const check = async () => {
    await page.goto('./');
    const order = await page.evaluate(() =>
      Array.from(document.querySelectorAll('.article-list > li')).map((li) => ({
        last: li.lastElementChild?.tagName,
        tags: Array.from(li.children).map((c) => c.tagName),
      })),
    );
    expect(order).toHaveLength(2);
    for (const { last, tags } of order) {
      expect(last).toBe('TIME');
      expect(tags.indexOf('TIME')).toBeGreaterThan(tags.indexOf('P'));
    }
  };

  await retryWhenLive(check);
});

test('a click on the description of an entry opens the article', async ({ page }) => {
  const check = async () => {
    await page.goto('./');
    const { x, y } = await centreOf(page.getByRole('main').getByRole('listitem').first().locator('p'));
    await page.mouse.click(x, y);
    await expect(page.getByRole('heading', { level: 1 })).toHaveText(FIRST_ARTICLE.title, {
      timeout: 5_000,
    });
    await expect(page).toHaveURL(FIRST_ARTICLE.url);
  };

  await retryWhenLive(check);
});

test('a click on the lower half of the date label opens the article', async ({ page }) => {
  const check = async () => {
    await page.goto('./');
    const label = page.getByRole('main').getByRole('listitem').first().locator('time');
    const box = (await label.boundingBox())!;
    await page.mouse.click(box.x + box.width / 2, box.y + box.height - 2);
    await expect(page.getByRole('heading', { level: 1 })).toHaveText(FIRST_ARTICLE.title, {
      timeout: 5_000,
    });
    await expect(page).toHaveURL(FIRST_ARTICLE.url);
  };

  await retryWhenLive(check);
});

test('at rest the box has a 2px border in the tone of lines and 8px rounded corners', { tag: '@colours' }, async ({
  page,
}) => {
  const check = async () => {
    await page.goto('./');
    const entry = page.getByRole('main').getByRole('listitem').first();
    await page.mouse.move(0, 0);
    const accent = await page.evaluate(() => {
      const probe = document.createElement('span');
      probe.style.color = 'var(--color-accent-line)';
      document.body.append(probe);
      const colour = getComputedStyle(probe).color;
      probe.remove();
      return colour;
    });
    for (const side of ['top', 'right', 'bottom', 'left']) {
      await expect(entry).toHaveCSS(`border-${side}-width`, '2px');
      await expect(entry).toHaveCSS(`border-${side}-style`, 'solid');
      await expect(entry).toHaveCSS(`border-${side}-color`, accent);
    }
    for (const corner of ['top-left', 'top-right', 'bottom-left', 'bottom-right']) {
      await expect(entry).toHaveCSS(`border-${corner}-radius`, '8px');
    }
  };

  await retryWhenLive(check);
});

// The border goes from 2px to 4px under the mouse and with the keyboard focus; the title does not move.
test('hover and keyboard focus raise the border to 4px without moving the title', async ({
  page,
}) => {
  const check = async () => {
    await page.goto('./');
    const entry = page.getByRole('main').getByRole('listitem').first();
    const title = entry.getByRole('heading', { level: 2 });
    await page.mouse.move(0, 0);
    await expect(entry).toHaveCSS('border-top-width', '2px');
    const rest = (await title.boundingBox())!;

    const { x, y } = await centreOf(entry.locator('p'));
    await page.mouse.move(x, y);
    for (const side of ['top', 'right', 'bottom', 'left']) {
      await expect(entry).toHaveCSS(`border-${side}-width`, '4px');
    }
    expect(await title.boundingBox()).toEqual(rest);

    await page.mouse.move(0, 0);
    await expect(entry).toHaveCSS('border-top-width', '2px');
    // The skip link or logo may come first: Tab until the link of the first entry has the focus.
    const link = entry.getByRole('link', { name: FIRST_ARTICLE.title });
    for (let i = 0; i < 10 && !(await link.evaluate((a) => a === document.activeElement)); i++) {
      await page.keyboard.press('Tab');
    }
    await expect(link).toBeFocused();
    for (const side of ['top', 'right', 'bottom', 'left']) {
      await expect(entry).toHaveCSS(`border-${side}-width`, '4px');
    }
    await expect(link).toHaveCSS('outline-style', 'none');
    expect(await title.boundingBox()).toEqual(rest);
  };

  await retryWhenLive(check);
});
