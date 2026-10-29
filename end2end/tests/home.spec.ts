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

test('home page shows header, body and footer, and loads stylesheet, font and favicon', async ({
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
    await page.goto('./');
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Home Page', {
      timeout: 5_000,
    });
    await expect(page.getByText('Sito in costruzione')).toBeVisible();
    await expect(page.getByText('© Luca Lorenzon 2026')).toBeVisible();
    await expect(page.getByRole('img', { name: 'Lvk@73r' })).toBeVisible();
    expect((await stylesheet).status()).toBe(200);
    expect((await font).status()).toBe(200);
    const favicon = await page.request.get(new URL('favicon.ico', page.url()).href);
    expect(favicon.status()).toBe(200);
  };

  await retryWhenLive(check);
});
