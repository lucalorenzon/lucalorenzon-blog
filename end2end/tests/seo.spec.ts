import { chromium, expect, test } from '@playwright/test';
import lighthouse from 'lighthouse';
import { createServer } from 'node:net';
import { pathsOfSitemap, retryWhenLive } from './support';

// What the check reads from Lighthouse's report. Lighthouse's own types are
// large; only the fields used here are named.
type Audit = {
  title: string;
  score: number | null;
  scoreDisplayMode: string;
  explanation?: string;
  errorMessage?: string;
};
type Report = {
  runtimeError?: { code: string; message: string };
  categories: { seo?: { auditRefs: { id: string }[] } };
  audits: Record<string, Audit>;
};

// Lighthouse needs a browser opened with a remote debugging port, which
// Playwright does not open by default. Ask the system for a free one.
function freePort(): Promise<number> {
  return new Promise((done, fail) => {
    const server = createServer();
    server.once('error', fail);
    server.listen(0, '127.0.0.1', () => {
      const { port } = server.address() as { port: number };
      server.close(() => done(port));
    });
  });
}

// The pages are the ones in the sitemap, which the build generates from the
// decision each page takes about being indexed: a page stays out of this check
// by staying out of the sitemap, so no page is exempted here.
async function sitemapPaths(
  request: import('@playwright/test').APIRequestContext,
): Promise<string[]> {
  const response = await request.get('./sitemap.xml');
  expect(response.status(), 'sitemap.xml must be served').toBe(200);
  const body = await response.text();
  const paths = pathsOfSitemap(body);
  expect(paths.length, 'sitemap.xml must list at least one <loc>').toBeGreaterThan(0);
  return paths;
}

// The failures of one page: every rule of Lighthouse's SEO category that is
// neither not applicable nor passed. A "manual" rule (structured data) is
// never judged by Lighthouse and is ignored here.
function failuresOf(report: Report): string[] {
  const failures: string[] = [];
  if (report.runtimeError) {
    failures.push(`Lighthouse runtime error ${report.runtimeError.code}: ${report.runtimeError.message}`);
  }
  const refs = report.categories.seo?.auditRefs ?? [];
  if (refs.length === 0) failures.push('the report has no SEO category');
  for (const { id } of refs) {
    const audit = report.audits[id];
    if (!audit) {
      failures.push(`${id}: missing from the report`);
      continue;
    }
    const { scoreDisplayMode: mode, score } = audit;
    if (mode === 'notApplicable' || mode === 'manual') continue;
    if (mode === 'binary' && score === 1) continue;
    const detail = [audit.errorMessage, audit.explanation].filter(Boolean).join(' ');
    failures.push(
      `${id} (${audit.title}): mode ${mode}, score ${score}${detail ? `. ${detail}` : ''}`,
    );
  }
  return failures;
}

test('every page of the sitemap meets Lighthouse\'s SEO rules', async ({ request, baseURL }) => {
  // GitHub Pages may take some seconds to serve a new deployment, so only the
  // reading of the sitemap is retried when live: once the new sitemap is there,
  // the pages are too. Retrying the whole run would not fit in retryWhenLive's
  // minute: Lighthouse takes several seconds per page, and the pages (six
  // today) run one after the other.
  let paths: string[] = [];
  await retryWhenLive(async () => {
    paths = await sitemapPaths(request);
  });
  // Set after retryWhenLive, which sets 90 s when live.
  test.setTimeout(300_000);

  const port = await freePort();
  const browser = await chromium.launch({ args: [`--remote-debugging-port=${port}`] });
  const failures: string[] = [];
  try {
    for (const path of paths) {
      await test.step(path, async () => {
        const address = new URL(path, baseURL).href;
        // enableErrorReporting is never passed: Lighthouse then does not load
        // Sentry and sends nothing.
        const result = await lighthouse(address, {
          port,
          onlyCategories: ['seo'],
          output: 'json',
          logLevel: 'error',
        });
        if (!result) throw new Error(`Lighthouse returned no result for ${address}`);
        for (const failure of failuresOf(result.lhr as unknown as Report)) {
          failures.push(`${address}: ${failure}`);
        }
      });
    }
  } finally {
    await browser.close();
  }
  expect(failures, `Lighthouse SEO failures:\n${failures.join('\n')}`).toEqual([]);
});
