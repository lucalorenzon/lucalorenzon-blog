# Verify the publication at the next version

- **Opened:** 2026-10-28, when the card
  [separate the build from the deploy](2026-10-28_separate-build-from-deploy.md)
  was closed.
- **Status:** done, closed on 2026-10-28, with version 0.2.0. One
  malfunction found, left to the card
  [run the live-site test when the snapshot is published](2026-10-28_run-live-test-when-snapshot-is-published.md).

## Why it was opened

Two parts of the publication had been built but had not run on GitHub,
because no version was published after them:

- the deploy publishing the `snapshot` of the tagged commit instead of
  rebuilding the site;
- the deploy creating the GitHub Release of the version, with its
  CHANGELOG section as text and the zip of the published site.

It was done together with the next version that changed the site, so
that no version was made only for this check: version 0.2.0, the
[header, footer and favicon](2026-10-28_add-header-footer-and-favicon.md).

## Answers

### Does the deploy publish the snapshot of the tagged commit?

**Yes.** The preparation commit `9ad3962` was pushed, and its build
([run 40653744209](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40653744209))
passed and saved `site-9ad3962….zip` in the `snapshot` Release. Only
then was the tag `v0.2.0` pushed on that commit. In the deploy
([run 40653969114](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40653969114))
the job `snapshot` reported "Snapshot of commit 9ad3962… found: it is
published, the site is not rebuilt", and the job `build` was skipped.

### Does the deploy create the version's Release?

**Yes.** The Release `v0.2.0` exists and is marked as latest; the
`snapshot` Release stays a pre-release. Its text is the 0.2.0 section of
`CHANGELOG.md` only, and `site-v0.2.0.zip` (21,259 bytes, the same size
as the snapshot's zip) is attached.

### Does the rest of the deploy still work on the snapshot path?

**No, one job did not run.** The job `verify-live`, the browser test on
the published site, was skipped, and the run still ended green. When
`build` is skipped, GitHub also skips the jobs that depend on it
indirectly and have no explicit condition; `verify-live` is one of them.
With version 0.1.0 the site was always rebuilt, so this could not show.
The fix was left by the user to the card
[run the live-site test when the snapshot is published](2026-10-28_run-live-test-when-snapshot-is-published.md).

For 0.2.0 the missing check was made by hand: see "Results".

### What about the deploy's refusals?

**Not seen.** A malformed tag, a commit outside `main` or a missing
CHANGELOG section would each have needed a deliberate failed run, and
the card allowed them only at no cost. They stay unseen until one
happens by mistake, when the deploy is expected to fail visibly.

## Results

- Version 0.2.0 is live at
  https://lucalorenzon.github.io/lucalorenzon-blog/: the tab title reads
  "Luca Lorenzon - Home Page", and the favicon and the font are served.
- The check `verify-live` should have made was run by hand on
  2026-10-28: the same Playwright test, run locally against the public
  address, passed (heading, text, footer, logo name, stylesheet, font
  and favicon).
- The push of the preparation commit went to `main` directly, as every
  push of this project does, so GitHub reported the required checks as
  bypassed; the checks then ran on that commit and passed.
