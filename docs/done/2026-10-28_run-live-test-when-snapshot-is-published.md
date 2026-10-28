# Run the live-site test when the snapshot is published

- **Opened:** 2026-10-28, while verifying the publication of version
  0.2.0 ([card](2026-10-28_verify-publication-at-next-version.md)).
- **Status:** done on 2026-10-28, with version 0.2.1. A malfunction,
  handled with the `fix` skill.

## Why it was opened

The deploy of version 0.2.0
([run 40653969114](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40653969114))
published the `snapshot` of the tagged commit, as designed, and skipped
the job `build`. It also skipped the job `verify-live`, so the browser
test on the published site did not run. Nothing in the run said so: the
run ended green. With version 0.1.0 the site was always rebuilt, so the
gap could not show. For 0.2.0 the missing check was made by hand: the
same browser test, run locally against the public address.

## Answers

### What was wrong

A job with no `if:` runs only if every job before it succeeded, even
indirectly: GitHub treats a skipped job as "not succeeded".
`verify-live` needs `deploy`, which needs `build`; when the snapshot is
published, `build` is skipped on purpose, and `verify-live` was skipped
with it. The job `release` was not affected because it already had an
explicit condition.

The bug, stated as a contract:

- **Condition:** the deploy publishes the snapshot, so `build` is
  skipped, and `deploy` succeeds.
- **Expected:** `verify-live` runs on the published site.
- **Unchanged otherwise:** if `deploy` fails, is skipped or the run is
  cancelled, `verify-live` does not run; the rebuild path behaves as
  before.

### What was changed

`verify-live` was given the same condition as `release`:
`!cancelled() && needs.deploy.result == 'success'`, with a comment in
`.github/workflows/deploy.yml` saying why it is needed. It was chosen
because it is the smallest change, and because the same condition was
already proven by `release` in the 0.2.0 run.

### Whether other jobs had the same gap

No. Every other job in `deploy.yml` and `build.yml` either has an
explicit condition, or depends only on jobs that are never skipped when
it should run. The closest case is `snapshot` in `build.yml`: it relies
on the implicit "every job before succeeded", which is what it should
do, since a site is kept only after its browser test passed.

### How the fix was proven

The failure could not be reproduced locally: it lives in how GitHub
decides which jobs to skip. The failing case was the 0.2.0 run above;
the proof is the deploy of version 0.2.1, a bug-fix patch by the rules
in `CHANGELOG.md`, published from the snapshot just like 0.2.0.

Rejected: checking the workflow with `actionlint`. It checks syntax,
expressions and the scripts inside the workflow, not which jobs end up
skipped, so it would not have found this bug. Whether to use it for
the rest is a separate question, left to a card in the backlog.

## Results

Deploy of version 0.2.1
([run 40782792212](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40782792212)),
tag on commit `517d739`: the snapshot was found and published, `build`
was skipped, and `verify-live` ran and passed. `release` succeeded as
before.
