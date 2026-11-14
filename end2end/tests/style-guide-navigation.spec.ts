import { test, expect, type Page } from '@playwright/test';

// The navigation the style guide's pages share (src/style_guide/navigation.rs):
//   - the breadcrumb above the text, a navigation named "Breadcrumb": the guide's
//     link leads to the index and the current page is marked as current
//   - the "Back to top" link below the text: with the keyboard it brings the
//     view to the top and the next Tab starts from the top of the page
// Per ADR-004 the component is checked once, on the code blocks page, the
// longest of the guide's pages; the net checks the markup of every page for
// accessibility, layout and contrast. It checks what the reader sees.

const PAGE = './style-guide/code.html';
const WIDTHS = [
  { name: 'iPhone SE', width: 320, height: 568 },
  { name: 'Laptop', width: 1280, height: 720 },
];

async function open(page: Page) {
  await page.goto(PAGE);
  await page.evaluate(() => document.fonts.ready);
}

for (const { name, width, height } of WIDTHS) {
  test(`the breadcrumb leads to the index and marks the current page, ${name}`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height });
    await open(page);

    const breadcrumb = page.getByRole('navigation', { name: 'Breadcrumb' });
    await expect(breadcrumb).toHaveCount(1);
    await expect(breadcrumb.getByRole('listitem')).toHaveText(['Style guide›', 'Code blocks']);
    // The current page is named, not linked, and the screen reader hears no "›".
    await expect(breadcrumb.getByRole('link')).toHaveCount(1);
    await expect(breadcrumb.locator('[aria-current="page"]')).toHaveText('Code blocks');
    await expect(breadcrumb.getByText('›')).toHaveAttribute('aria-hidden', 'true');
    // It sits below the header and above the text.
    const box = await breadcrumb.boundingBox();
    const header = await page.getByRole('banner').boundingBox();
    const firstSection = await page.getByRole('heading', { level: 2 }).first().boundingBox();
    expect(box!.y).toBeGreaterThanOrEqual(header!.y + header!.height);
    expect(box!.y + box!.height).toBeLessThanOrEqual(firstSection!.y);

    await breadcrumb.getByRole('link', { name: 'Style guide' }).click();
    await expect(page).toHaveURL(/\/style-guide\/(index\.html)?$/);
    await expect(page.getByRole('heading', { level: 1 })).toHaveText('Style guide');
    // The index has neither the breadcrumb nor the link to the top.
    await expect(page.getByRole('navigation', { name: 'Breadcrumb' })).toHaveCount(0);
    await expect(page.getByRole('link', { name: 'Back to top' })).toHaveCount(0);
  });

  test(`Back to top, by keyboard, brings the view and the next Tab to the top, ${name}`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height });
    await open(page);

    const backToTop = page.getByRole('link', { name: 'Back to top', exact: true });
    await expect(backToTop).toHaveCount(1);
    await backToTop.focus();
    await page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));
    expect(await page.evaluate(() => window.scrollY)).toBeGreaterThan(height);
    // It closes the text, above the footer.
    const link = await backToTop.boundingBox();
    const footer = await page.getByRole('contentinfo').boundingBox();
    expect(link!.y + link!.height).toBeLessThanOrEqual(footer!.y);

    await page.keyboard.press('Enter');
    await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(0);

    // The next Tab starts from the top of the page: the first thing to focus, the logo.
    await page.keyboard.press('Tab');
    await expect(page.getByRole('link', { name: 'Lvk@73r, home page' })).toBeFocused();
    expect(await page.evaluate(() => window.scrollY)).toBe(0);
  });
}
