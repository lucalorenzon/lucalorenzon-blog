import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test } from '@playwright/test';
import { live, retryWhenLive, TEXT_PAGES } from './support';

// Locally the link is built from SITE_ADDRESS (the build's default when unset);
// on the live site the page's own address is the canonical one.
const siteAddress = process.env.SITE_ADDRESS?.trim() || 'https://www.lucalorenzon.com/';

for (const textPage of TEXT_PAGES) {
  test(`the ${textPage.name} has one canonical link with its full address`, async ({
    page,
    baseURL,
  }) => {
    await retryWhenLive(async () => {
      await page.goto(textPage.address);
      // './' is the home page: its address is the site's address itself.
      const siteRelative = textPage.address.replace(/^\.\//, '');
      const expected = live
        ? new URL(textPage.address, baseURL).href
        : siteAddress + siteRelative;
      const canonical = page.locator('link[rel="canonical"]');
      await expect(canonical).toHaveCount(1);
      await expect(canonical).toHaveAttribute('href', expected);
    });
  });
}

test('the "page not found" asks not to be indexed and has no canonical link', async ({ page }) => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/404.html'))) {
    throw new Error(
      'target/site/404.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }

  await retryWhenLive(async () => {
    // Live, only the host decides what a missing address shows: ask it for one.
    await page.goto(live ? 'a/b/c/missing-page' : './404.html');
    const robots = page.locator('meta[name="robots"]');
    await expect(robots).toHaveCount(1);
    await expect(robots).toHaveAttribute('content', 'noindex');
    await expect(page.locator('link[rel="canonical"]')).toHaveCount(0);
  });
});
