import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test } from '@playwright/test';
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
