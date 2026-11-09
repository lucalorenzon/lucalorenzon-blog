# Start the SEO check first, so it overlaps the other tests

- **Opened:** 2026-11-08, by the agent, measuring where the browser
  tests spend their time at the close of the card
  [check at every width only what depends on the width](2026-11-08_check-at-every-width-only-what-depends-on-it.md);
  in Next.
- **Status:** done, 2026-11-09.

## Why

The SEO check (`end2end/tests/seo.spec.ts`, project `lighthouse`) is a
single test of about 30 seconds: Lighthouse checks every page of the
sitemap in turn. In the pipeline the tests run on two workers, and this
test started last, alone (run 42514256662: test 315 of 315, 31.5
seconds). So about 30 seconds of the browser test step were spent with
one worker idle. Started first, the other tests run beside it, and the
step loses a good part of those 30 seconds.

## Answers

### How to make it start first?

**Listing the `lighthouse` project first in
`end2end/playwright.config.ts` is enough** (agent, approved by the
user). Playwright builds the groups of tests project by project, in the
order of the configuration, and the workers take them from that queue;
no other setting is needed. The measurement below confirms it: the SEO
check now starts within the first second, as test 2 of 227 on two
workers (it was 226 or 227 of 227).

Rejected: a dependency between projects. It makes one project wait for
another, the opposite of what is wanted here.

### Does Lighthouse's own Chromium slow the tests beside it?

**Not measurably on this machine.** On two workers the SEO check took
29.0 and 29.1 seconds with the other tests running beside it, against
29.5 alone. The other tests, on the one worker left, finished in about
35 seconds; on two workers they took about 19.5, so they run at about
the speed of one worker out of two, with no visible loss.

**The pipeline's machine was not measured.** It would take a push only
for that, and the user chose the local measurement (2026-11-09). The
first pipeline run after the next push shows the answer there: the
browser test step and the SEO check's duration in its log.

### Does the live check after a publication gain the same?

**Yes, by construction, not measured.** The `verify-live` job in
`.github/workflows/deploy.yml` runs `npx playwright test` with the same
configuration, on the same kind of machine, so the same order applies.
Measuring it would mean publishing a version.

## Results

The suite, 227 tests (226 passed, 1 skipped), run four times before and
four times after the change on the same machine, without rebuilding
the site in between:

| Workers | Before | After | Saved |
|---|---|---|---|
| 2, as in the pipeline | 49.6 and 48.9 s | 35.8 and 35.5 s | about 13.6 s, 28% |
| 12, this machine's default | 33.6 and 33.7 s | 30.8 and 31.3 s | about 2.6 s, 8% |

On twelve workers the SEO check already started early (about 3 seconds
in), so the suite was already bound by its 30 seconds; it now starts at
once. On two workers the gain is smaller than the full 30 seconds,
because the other tests then run on a single worker.
