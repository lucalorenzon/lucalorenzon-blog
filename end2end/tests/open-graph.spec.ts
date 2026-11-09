import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { live, retryWhenLive, NET_PAGES } from './support';

// The content of the one meta element that matches; fails if there are none or several.
async function metaContent(page: Page, selector: string): Promise<string> {
  const tag = page.locator(selector);
  await expect(tag).toHaveCount(1);
  return (await tag.getAttribute('content')) ?? '';
}

const og = (property: string) => `meta[property="${property}"]`;

// What the pages say about themselves is checked as behaviour, not as texts:
// the texts belong to the code and change with it.
for (const netPage of NET_PAGES) {
  test(`the ${netPage.name} has its description and its Open Graph tags`, async ({ page }) => {
    await retryWhenLive(async () => {
      await page.goto(netPage.address);

      const description = await metaContent(page, 'meta[name="description"]');
      expect(description.length).toBeGreaterThan(0);
      expect(description.length).toBeLessThanOrEqual(160);

      expect(await metaContent(page, og('og:description'))).toBe(description);
      expect(await metaContent(page, og('og:title'))).toBe(
        (await page.getByRole('heading', { level: 1 }).innerText()).trim(),
      );
      const canonical = page.locator('link[rel="canonical"]');
      await expect(canonical).toHaveCount(1);
      expect(await metaContent(page, og('og:url'))).toBe(await canonical.getAttribute('href'));
      expect(await metaContent(page, og('og:site_name'))).toBe('Luca Lorenzon');
      expect(await metaContent(page, og('og:locale'))).toBe('en_GB');

      const isArticle = netPage.address.startsWith('./articles/');
      const isProfile = ['./curriculum.html', './about.html'].includes(netPage.address);
      expect(await metaContent(page, og('og:type'))).toBe(
        isArticle ? 'article' : isProfile ? 'profile' : 'website',
      );

      if (isArticle) {
        const published = await metaContent(page, og('article:published_time'));
        expect(published).toMatch(/^\d{4}-\d{2}-\d{2}$/);
        expect(Number.isNaN(Date.parse(published))).toBe(false);
      } else {
        await expect(page.locator(og('article:published_time'))).toHaveCount(0);
      }

      if (isProfile) {
        expect(await metaContent(page, og('profile:first_name'))).toBe('Luca');
        expect(await metaContent(page, og('profile:last_name'))).toBe('Lorenzon');
      } else {
        await expect(page.locator('meta[property^="profile:"]')).toHaveCount(0);
      }
    });
  });
}

test('the "page not found" has no description and no Open Graph tags', async ({ page }) => {
  if (!live && !existsSync(resolve(__dirname, '../../target/site/404.html'))) {
    throw new Error(
      'target/site/404.html not found: run scripts/build-site.sh from the repository root before this test.',
    );
  }

  await retryWhenLive(async () => {
    // Live, only the host decides what a missing address shows: ask it for one.
    await page.goto(live ? 'a/b/c/missing-page' : './404.html');
    await expect(page.locator('meta[name="description"]')).toHaveCount(0);
    await expect(page.locator('meta[property^="og:"]')).toHaveCount(0);
    await expect(page.locator('meta[property^="profile:"], meta[property^="article:"]')).toHaveCount(0);
  });
});
