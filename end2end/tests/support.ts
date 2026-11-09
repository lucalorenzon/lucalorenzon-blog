import { test } from '@playwright/test';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { relative, resolve } from 'node:path';

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

// The widths for tests of what is inside the text column. Measured in the
// browser: the column grows with the screen from 304 to 584 px up to about
// 600 px, stays at 584 px up to 767 px (text 18 px), and is 704 px wide from
// 768 px up (text 20 px). The table forms and the code "wrap" toggle follow
// the column. So a test of what is inside the column needs one width per
// column size: 659 and 767 give the same result, and so do the seven widths
// from 768 to 1920. The widths between 412 and 600 are sampled at three
// points only, as before. Why: docs/done/2026-11-08_check-at-every-width-only-what-depends-on-it.md
const COLUMN_WIDTH_NAMES = ['iPhone SE', 'iPhone 15', 'Pixel 7', 'boundary 767', 'iPad Mini'];
export const COLUMN_WIDTHS = VIEWPORTS.filter(({ name }) => COLUMN_WIDTH_NAMES.includes(name));

// The pages show different elements per width band (and axe checks only what
// is shown), so some tests run once per band, with the same names and sizes
// used in header-layout.spec.ts.
export const BANDS = [
  { name: 'iPhone 15', width: 393, height: 659 }, // text logo
  { name: 'iPad Mini', width: 768, height: 1024 }, // ASCII-art logo, title below the line
  { name: 'Laptop', width: 1280, height: 720 }, // title beside the logo
];

// The pages whose text (the `main` element) the content tests check. The
// home page (a list of articles, each entry a heading, a description and a date) has no roles (h3) and
// no consecutive paragraphs; the curriculum page has both; the licence page has neither, so those two
// checks apply only where the page has some. The After Save Event article has no roles (h3)
// and several consecutive paragraphs. The hybris PK article has no roles
// (h3) and several consecutive paragraphs; its wide table becomes a record
// viewer below a readable width.
export const TEXT_PAGES = [
  { name: 'home page', address: './', hasRoles: false, hasConsecutiveParagraphs: false },
  { name: 'curriculum page', address: './curriculum.html', hasRoles: true, hasConsecutiveParagraphs: true },
  { name: 'licence page', address: './licence.html', hasRoles: false, hasConsecutiveParagraphs: false },
  { name: 'about page', address: './about.html', hasRoles: false, hasConsecutiveParagraphs: false },
  { name: 'After Save Event page', address: './articles/after-save-event.html', hasRoles: false, hasConsecutiveParagraphs: true },
  { name: 'hybris PK page', address: './articles/how-to-generate-hybris-pk-values-without-hybris.html', hasRoles: false, hasConsecutiveParagraphs: true },
];

// The built site, and its pages, found by walking the folder: a new page is
// covered by the tests that use these without being listed. A live run has no
// folder to walk, so the lists are empty and those tests skip.
export const SITE = resolve(__dirname, '../../target/site');

export function pagesOf(folder: string): string[] {
  if (!existsSync(folder)) return [];
  return readdirSync(folder, { withFileTypes: true }).flatMap((entry) => {
    const path = resolve(folder, entry.name);
    if (entry.isDirectory()) return pagesOf(path);
    return entry.name.endsWith('.html') ? [path] : [];
  });
}

export const ADDRESSES = pagesOf(SITE).map((path) => './' + relative(SITE, path));

// The pages that have at least one table (every Markdown table is rendered
// inside a wrapper holding the table and its record viewer).
export const TABLE_ADDRESSES = ADDRESSES.filter((address) =>
  readFileSync(resolve(SITE, address), 'utf8').includes('table-with-viewer'),
);

// The paths, on the server under test, of the pages a sitemap lists.
export function pathsOfSitemap(xml: string): string[] {
  const addresses = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((match) => match[1].trim());
  // The <loc> addresses always point to the site's production address, whatever
  // server is under test. Locally the pages are served from 127.0.0.1, and
  // Lighthouse's canonical rule does not compare domains, so each address is
  // mapped to the server under test by keeping its path and resolving it,
  // relative, against the test's base URL.
  return addresses.map((address) => {
    const url = new URL(address);
    return './' + url.pathname.replace(/^\/+/, '') + url.search;
  });
}

// The pages the "net" tests (written once, run on every page) check. The list
// is the build's sitemap in both moments: locally and in the build the site is
// in target/site; in the live check after a publication the pipeline puts only
// the build's sitemap.xml there (see deploy.yml), so the tests that walk the
// folder (ADDRESSES, TABLE_ADDRESSES) still find no pages and skip. It is read
// synchronously, because Playwright builds the list of tests before running
// any. A missing or empty sitemap throws: a net check with nothing to check
// must not pass.
function netPages(): { name: string; address: string }[] {
  const file = resolve(SITE, 'sitemap.xml');
  if (!existsSync(file)) {
    throw new Error(
      `${file} not found: run scripts/build-site.sh from the repository root before the tests` +
        ' (for the live run, the pipeline copies the build\'s sitemap.xml there).',
    );
  }
  const paths = pathsOfSitemap(readFileSync(file, 'utf8'));
  if (paths.length === 0) {
    throw new Error(`${file} lists no <loc>: the net tests would have no page to check.`);
  }
  return paths.map((address) => ({ name: address === './' ? 'home page' : address.replace(/^\.\//, ''), address }));
}

export const NET_PAGES = netPages();

// What a reader's own style sheet may set, from the text of the criterion.
export const READER_SPACING = `
  * {
    line-height: 1.5 !important;
    letter-spacing: 0.12em !important;
    word-spacing: 0.16em !important;
  }
  p { margin-bottom: 2em !important; }
`;
