# Run the live-site test when the snapshot is published

- **Opened:** 2026-10-28, while verifying the publication of version
  0.2.0 ([card](../done/2026-10-28_verify-publication-at-next-version.md)).
- **Status:** being worked on since 2026-10-28. A malfunction: handled with the
  `fix` skill.

## Why

The deploy of version 0.2.0
([run 40653969114](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40653969114))
published the `snapshot` of the tagged commit, as designed, and skipped
the job `build`. It also skipped the job `verify-live`, so the browser
test on the published site did not run. Nothing in the run said so: the
run ended green.

Probable cause, in `.github/workflows/deploy.yml`: `verify-live` has no
`if:`, so GitHub applies the default condition, which requires every job
it depends on, directly or not, to have succeeded. `verify-live` needs
`deploy`, which needs `build`; when `build` is skipped, GitHub skips
`verify-live` too. The job `release`, which has an explicit condition
(`!cancelled() && needs.deploy.result == 'success'`), ran. With version
0.1.0 the site was always rebuilt, so the gap could not show.

For 0.2.0 the missing check was made by hand on 2026-10-28: the same
browser test, run locally against the public address.

## Proposal

- Give `verify-live` the same explicit condition as `release`.
- The failing case already exists: the 0.2.0 deploy above. The fix is
  proven only by a deploy that publishes the snapshot and runs
  `verify-live`, which means the next version. By the rules in
  `CHANGELOG.md` a bug fix is a patch, so it can be 0.2.1.
- Check whether any other job in `deploy.yml` or `build.yml` depends,
  directly or not, on a job that can be skipped, and lacks an explicit
  condition.
