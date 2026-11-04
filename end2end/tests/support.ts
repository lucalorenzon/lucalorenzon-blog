import { test } from '@playwright/test';

export const live = Boolean(process.env.BASE_URL?.trim());

// GitHub Pages may take some seconds to serve a new deployment: when
// checking the live site, retry the whole check for up to a minute.
// Locally the check runs once.
export async function retryWhenLive(check: () => Promise<void>): Promise<void> {
  if (live) {
    test.setTimeout(90_000);
    await test.expect(check).toPass({ timeout: 60_000, intervals: [2_000, 5_000] });
  } else {
    await check();
  }
}

// The devices every layout test runs on.
export const VIEWPORTS = [
  { name: 'iPhone SE', width: 320, height: 568 },
  { name: 'iPhone 15', width: 393, height: 659 },
  { name: 'Pixel 7', width: 412, height: 839 },
  { name: 'iPhone 15 landscape', width: 659, height: 393 },
  { name: 'boundary 767', width: 767, height: 800 },
  { name: 'iPad Mini', width: 768, height: 1024 },
  { name: 'iPad 7th gen', width: 810, height: 1080 },
  { name: 'iPad Mini landscape', width: 1024, height: 768 },
  { name: 'iPad Pro 11 landscape', width: 1194, height: 834 },
  { name: 'boundary 1279', width: 1279, height: 800 },
  { name: 'Laptop', width: 1280, height: 720 },
  { name: 'Full HD', width: 1920, height: 1080 },
];

// The pages whose text (the `main` element) the content tests check. The
// home page shows the curriculum, with roles (h3) and several consecutive
// paragraphs; the licence page has neither, so those two checks apply only
// where the page has some. The about page opens with a paragraph, so its
// first section is not the first element and has a line above it.
// The After Save Event article has no roles (h3), several consecutive
// paragraphs and opens with a paragraph. The hybris PK article stays out
// until the card docs/wip/annotate-markdown-for-wide-content.md handles its
// 6-column table (code blocks are done).
export const TEXT_PAGES = [
  { name: 'home page', address: './', hasRoles: true, hasConsecutiveParagraphs: true, startsWithSection: true },
  { name: 'licence page', address: './licence.html', hasRoles: false, hasConsecutiveParagraphs: false, startsWithSection: true },
  { name: 'about page', address: './about.html', hasRoles: false, hasConsecutiveParagraphs: false, startsWithSection: false },
  { name: 'After Save Event page', address: './articles/after-save-event.html', hasRoles: false, hasConsecutiveParagraphs: true, startsWithSection: false },
];
