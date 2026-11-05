import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test } from '@playwright/test';
import { live, retryWhenLive } from './support';

// The page background has the colour of the project's colour scheme only when
// style.css is applied; without the stylesheet the browser leaves it
// transparent.
const styledBackground = (colorScheme: string | null | undefined) =>
  colorScheme === 'dark' ? 'rgb(0, 0, 0)' : 'rgb(251, 250, 247)';

test.beforeAll(() => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/404.html'))) {
    throw new Error(
      'target/site/404.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }
});

test('the "page not found" page has its title, heading and stylesheet, and leads home', async ({
  page,
}, testInfo) => {
  test.skip(live, 'live: the page is checked through a missing address, see below');

  await page.goto('./404.html');
  await expect(page).toHaveTitle('Page not found - Luca Lorenzon');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Page not found');
  await expect(page.locator('body')).toHaveCSS('background-color', styledBackground(testInfo.project.use.colorScheme));

  await page.getByRole('link', { name: 'Go to the home page' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Curriculum Vitae');
});

test('the footer link of the "page not found" leads to the about page', async ({ page }) => {
  test.skip(live, 'live: checked through a missing address, see below');

  await page.goto('./404.html');
  await page.getByRole('link', { name: 'Reuse with attribution' }).click();
  await expect(page).toHaveTitle('About - Luca Lorenzon');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('About');
});

test('a missing address, however deep, is answered with the "page not found" and its stylesheet', async ({
  page,
}, testInfo) => {
  test.skip(!live, 'only the host decides what a missing address shows: checked on the live site');

  const check = async () => {
    const response = await page.goto('a/b/c/missing-page');
    expect(response?.status()).toBe(404);
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Page not found', {
      timeout: 5_000,
    });
    await expect(page.locator('body')).toHaveCSS('background-color', styledBackground(testInfo.project.use.colorScheme), {
      timeout: 5_000,
    });

    // The footer link is written from the site's path, so it works from here.
    await page.getByRole('link', { name: 'Reuse with attribution' }).click();
    await expect(page).toHaveTitle('About - Luca Lorenzon', { timeout: 5_000 });
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('About');
  };

  await retryWhenLive(check);
});
