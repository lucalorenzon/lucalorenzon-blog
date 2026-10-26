import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test } from '@playwright/test';

const live = Boolean(process.env.BASE_URL?.trim());

test.beforeAll(() => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/index.html'))) {
    throw new Error(
      'target/site/index.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }
});

test('home page shows the heading and loads its stylesheet', async ({ page }) => {
  // GitHub Pages may take some seconds to serve a new deployment: when
  // checking the live site, retry the whole check for up to a minute.
  const check = async () => {
    const stylesheet = page.waitForResponse(
      (response) => new URL(response.url()).pathname.endsWith('/style.css'),
      { timeout: 10_000 },
    );
    stylesheet.catch(() => {}); // a failed attempt must not leave an unhandled rejection
    await page.goto('./');
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Sito in costruzione', {
      timeout: 5_000,
    });
    expect((await stylesheet).status()).toBe(200);
  };

  if (live) {
    test.setTimeout(90_000);
    await expect(check).toPass({ timeout: 60_000, intervals: [2_000, 5_000] });
  } else {
    await check();
  }
});
