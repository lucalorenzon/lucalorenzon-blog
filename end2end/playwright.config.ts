import { defineConfig, devices } from '@playwright/test';

// Two moments, one test:
//  - BASE_URL not set: serve the freshly built site (../target/site) locally.
//  - BASE_URL set: check the already published site at that address.
const rawBaseUrl = process.env.BASE_URL?.trim();
const live = Boolean(rawBaseUrl);
const port = 4173;

// A trailing slash is required so that relative navigation ('./') stays
// inside a sub-path, should BASE_URL ever point to one. The site itself is
// served from the root of https://www.lucalorenzon.com/ since 2026-11-01.
const baseURL = live
  ? rawBaseUrl!.replace(/\/*$/, '/')
  : `http://127.0.0.1:${port}/`;

export default defineConfig({
  testDir: './tests',
  forbidOnly: Boolean(process.env.CI),
  retries: 0,
  reporter: process.env.CI ? [['list'], ['github']] : 'list',
  use: { baseURL },
  // Everything runs in the light theme: it is what readers with no preference
  // see (ADR-003). The dark project runs only the tests tagged @colours, the
  // ones whose answer can differ between the themes. That is valid only while
  // the themes differ in colours alone; the guard in themes.spec.ts fails if a
  // theme starts to change size or position. The SEO check runs once, in its
  // own project: Lighthouse opens its own Chromium with a control port anyway,
  // so it does not use the browser Playwright prepares for each project.
  // Why: docs/adr/ADR-004-test-components-once-and-pages-with-a-net.md and the
  // card docs/wip/run-colour-free-tests-in-one-theme.md.
  projects: [
    {
      name: 'chromium-dark',
      testIgnore: 'seo.spec.ts',
      grep: /@colours/,
      use: { ...devices['Desktop Chrome'], colorScheme: 'dark' },
    },
    {
      name: 'chromium-light',
      testIgnore: 'seo.spec.ts',
      use: { ...devices['Desktop Chrome'], colorScheme: 'light' },
    },
    {
      name: 'lighthouse',
      testMatch: 'seo.spec.ts',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: live
    ? undefined
    : {
        command: `python3 -m http.server ${port} --bind 127.0.0.1 --directory ../target/site`,
        url: baseURL,
        reuseExistingServer: false,
      },
});
