# Start the SEO check first, so it overlaps the other tests

- **Opened:** 2026-11-08, by the agent, measuring where the browser
  tests spend their time at the close of the card
  [check at every width only what depends on the width](../done/2026-11-08_check-at-every-width-only-what-depends-on-it.md);
  in Next.
- **Status:** being worked on, since 2026-11-09.

## Why

The SEO check (`end2end/tests/seo.spec.ts`, project `lighthouse`) is a
single test of about 30 seconds: Lighthouse checks every page of the
sitemap in turn. In the pipeline the tests run on two workers, and this
test starts last, alone (run 42514256662: test 315 of 315, 31.5
seconds). So about 30 seconds of the browser test step are spent with
one worker idle. If it started first, the other tests would run beside
it, and the step could lose most of those 30 seconds. Locally, on 14
workers, it alone sets the suite's 33 seconds.

## Questions

- How to make it start first: Playwright runs projects in the order
  given unless they depend on each other, and splits the tests among
  the workers; whether moving the `lighthouse` project first in
  `end2end/playwright.config.ts` is enough, or the order needs another
  setting.
- Whether its own Chromium, opened by Lighthouse with a control port,
  slows the tests running beside it on the pipeline's machine.
- Whether the live check after a publication gains the same.
