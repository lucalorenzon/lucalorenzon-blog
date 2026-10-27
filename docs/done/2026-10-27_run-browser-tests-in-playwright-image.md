# Backlog: run the browser tests inside the official Playwright image

- **Opened:** 2026-10-26
- **Status:** done on 2026-10-27, see "Decision". First deferred on
  2026-10-26 to watch whether the slowdown recurred; it recurred in the
  next two runs (see "Measurements"), so the user decided to address it.

## Decision

Implemented as proposed below, in commit `14800f8`; the image tag is
derived from the code (option B, chosen by the user on 2026-10-27).

- **The image follows the lock file.** Dependabot does not update
  container images referenced in a workflow: it updates only actions
  written as `owner/repo@version`, and its documentation lists container
  registry references as unsupported. So the `build` job reads the
  installed `@playwright/test` version from `end2end/package-lock.json`,
  checks that it looks like `1.2.3`, and both test jobs run in
  `mcr.microsoft.com/playwright:v<version>-noble`. The code stays the
  single source of truth: a Dependabot update of the package needs no
  workflow edit. Rejected: writing the tag by hand in the workflow, which
  would make every such update fail until the workflow is fixed by hand.
- **Jobs.** `build` no longer runs the browser test; it uploads the built
  site as an artifact kept for one day. A new job, `browser-test`, runs
  the test before publishing inside the image; `deploy` waits for both;
  `verify-live` runs in the same image.
- **`browser-test` runs whenever `build` succeeded**, even if the change
  detection job failed. Without that explicit condition a failed
  detector would skip the test, and a skipped job counts as passed: the
  required check would be green with no test run.
- **Required check.** The ruleset "Require CI on main" now requires
  `browser-test` as well as `build` (changed with `gh api` on 2026-10-27,
  at the user's request).

### Results

- Tried locally first, with the user running Docker (the user's account
  is deliberately not in the `docker` group): the image runs as root,
  contains Node 24 and Python 3.12 (so the local test server needs no
  change), and the test passes.
- First run on GitHub,
  [40425877211](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40425877211):
  every job passed; whole run 2 min 13 s; `browser-test` 29 s and
  `verify-live` 35 s, most of it pulling the image from Microsoft's
  registry. Nothing is installed from Ubuntu's package server any more.
- The "Cons" below still hold for "more moving parts"; the version
  alignment one is solved by the decision above.

## Why this was asked

On the first publication of the site (GitHub Actions run
[40409740212](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40409740212)),
the job that tests the live site took 9 min 40 s for a test that runs in
about 1 second.

Almost all of that time went into one step: installing the system
libraries Chromium needs (`npx playwright install --with-deps chromium`).
Ubuntu's package server delivered 24.9 MB at 45 kB/s, which took 9 min 11 s.
Chromium itself downloaded in 4 seconds. In the same run, a few minutes
earlier, the same step in the build job took 19 seconds.

So the cause was a slowdown of the package server GitHub's runners use,
not a defect in the pipeline. Both test runs, before and after
publishing, depend on that server on every run, so the slowdown can come
back at any time and delay a publication.

## Measurements

Duration of the step that installs Chromium and its system libraries,
per run of the "Build and deploy" workflow. The rest of the workflow takes
about a minute and a half in total.

| Run | Date (UTC) | Before publishing (`build`) | After publishing (`verify-live`) | Whole run |
|---|---|---|---|---|
| [40409740212](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40409740212) | 2026-10-26 22:05 | 19 s | **568 s** | ~13 min |
| [40413126574](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40413126574) | 2026-10-26 22:40 | normal | normal | ~2 min |
| [40415178173](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40415178173) | 2026-10-26 22:48 | **657 s** | **205 s** | ~15 min |

Slow in 3 of the 6 installs, across 2 of the 3 runs: recurring, not a
one-off, at least on that day. Chromium itself always downloaded in a few
seconds; the time goes into the system libraries.

## Proposal

Run both browser tests inside Playwright's official container image
(`mcr.microsoft.com/playwright:v<version>`). The image already contains
Chromium and every system library it needs, so nothing is downloaded from
Ubuntu's package server at test time.

What it changes in `.github/workflows/deploy.yml`:

- The browser test before publishing moves out of the `build` job (which
  needs Rust) into a separate job running in the container. That job
  receives the built site from `build` as an artifact.
- The test after publishing (`verify-live`) runs in the same container.
- Publishing waits for the new test job as well as for `build`.
- The `main` branch ruleset ("Require CI on main") must require the new
  test job too, otherwise a pull request could be merged with a failing
  browser test.

## Pros

- The slow, unreliable step disappears from both test runs.
- Test runs get faster on average, because nothing is installed at run
  time.
- The browser and its libraries are exactly the ones the Playwright team
  tested together, instead of whatever the runner's Ubuntu version
  provides. This also shields the tests from GitHub moving `ubuntu-latest`
  to a new Ubuntu release, which was in progress at the time.

## Cons

- **Two versions to keep aligned.** The image tag must match the
  `@playwright/test` version in `end2end/package.json`. Dependabot updates
  the package; whether it also updates an image referenced in a workflow
  was not verified. If it does not, every Playwright update needs a
  manual change to the workflow, and forgetting it breaks the test run.
- **More moving parts.** One more job, a site artifact passed between
  jobs, and one more required check in the ruleset.
- **The problem may be temporary.** The measurements cover a single
  evening; the package server may recover. This no longer argues for
  waiting: three slow installs out of six already cost about 24 minutes.

## When to pick this up

Next working session, as decided by the user on 2026-10-27. The
"Cons" above stay to be weighed when choosing how to do it, in
particular how to keep the image tag aligned with the Playwright
package.
