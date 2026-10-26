import { defineConfig, devices } from '@playwright/test';

// Two moments, one test:
//  - BASE_URL not set: serve the freshly built site (../target/site) locally.
//  - BASE_URL set: check the already published site at that address.
const rawBaseUrl = process.env.BASE_URL?.trim();
const live = Boolean(rawBaseUrl);
const port = 4173;

// A trailing slash is required so that relative navigation ('./') stays
// inside a sub-path such as https://lucalorenzon.github.io/lucalorenzon-blog/.
const baseURL = live
  ? rawBaseUrl!.replace(/\/*$/, '/')
  : `http://127.0.0.1:${port}/`;

export default defineConfig({
  testDir: './tests',
  forbidOnly: Boolean(process.env.CI),
  retries: 0,
  reporter: process.env.CI ? [['list'], ['github']] : 'list',
  use: { baseURL },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: live
    ? undefined
    : {
        command: `python3 -m http.server ${port} --bind 127.0.0.1 --directory ../target/site`,
        url: baseURL,
        reuseExistingServer: false,
      },
});
