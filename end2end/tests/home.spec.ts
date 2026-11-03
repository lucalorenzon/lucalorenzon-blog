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
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Curriculum Vitae', {
      timeout: 5_000,
    });
    await expect(page.getByRole('heading', { level: 2, name: 'Summary' })).toBeVisible();
    await expect(page.getByText('© 2026 Luca Lorenzon')).toBeVisible();
    await expect(page.getByRole('link', { name: 'This CV may be copied, not changed' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Lvk@73r, home page' })).toHaveCount(1);
    expect((await stylesheet).status()).toBe(200);
    expect((await font).status()).toBe(200);
    expect((await textFont).status()).toBe(200);
    const favicon = await page.request.get(new URL('favicon.ico', page.url()).href);
    expect(favicon.status()).toBe(200);
  };

  await retryWhenLive(check);
});

test('the footer link of the home page leads to the about page', async ({ page }) => {
  const check = async () => {
    await page.goto('./');
    await page.getByRole('link', { name: 'This CV may be copied, not changed' }).click();
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
      await expect(page.getByRole('heading', { level: 1 })).toHaveText('Curriculum Vitae', {
        timeout: 5_000,
      });

      // Pointer.
      await page.goto('./licence.html');
      await logo.click();
      await expect(page.getByRole('heading', { level: 1 })).toHaveText('Curriculum Vitae');
    };

    await retryWhenLive(check);
  });
}
