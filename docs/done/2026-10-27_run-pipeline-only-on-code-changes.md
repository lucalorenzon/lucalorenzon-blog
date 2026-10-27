# Run build, deploy and tests only when the code changes

- **Opened:** 2026-10-26
- **Status:** done on 2026-10-27, in commit `7a21480`.

## Why it was opened

Every push to `main` ran the whole workflow (`.github/workflows/deploy.yml`):
checks, build, browser test, publication, browser test on the live site.
Commits that only touch documentation (`docs/`, `README.md`, `CLAUDE.md`)
change nothing on the site, yet they rebuilt and republished it. It
happened right away: the session minute and the backlog cards of
2026-10-26 each triggered a full run.

The cost was runner minutes and waiting time, and exposure to occasional
slowdowns such as the 9-minute one recorded in
[the Playwright image card](2026-10-27_run-browser-tests-in-playwright-image.md),
all for a publication identical to the previous one.

## Answers

### How can documentation-only changes skip the run without blocking pull requests?

The catch was the required `build` check. The `main` ruleset ("Require
CI on main") requires it, and the GitHub documentation states that when a
workflow is skipped because of a path filter, its checks stay "Pending":
a pull request that requires them cannot be merged. Direct pushes from the
author are not affected, because the author's account bypasses the rule.

**Chosen (by the user): always start the workflow, and skip the heavy jobs
when only documentation changed.** A job skipped by its own `if:`
condition reports its check as successful. So a first job, `changes`,
lists the changed files with `git diff` (no third-party action), and
`build` runs only if that job did not answer "documentation only";
`deploy` and `verify-live` depend on `build` and are skipped with it. The
skip happens per job, not per step, which is simpler than first proposed:
the heavy steps stay free of conditions. It costs one extra small job.

Rejected:

- **`paths-ignore` on the workflow triggers.** One small change, but it
  would block, forever, any pull request that touches only documentation,
  for example one opened by a remote session.
- **A second, tiny workflow** for documentation-only changes, reporting a
  `build` check with the same name. It keeps the main workflow simple, but
  two workflows that must mirror each other (same check name, opposite
  path filters) are an easy place for a silent mistake.

### What happens when the answer is uncertain?

**Everything runs.** Manual run, first push of a branch, force push,
empty change list: in every such case `build` runs. If the detection job
itself fails, `build` (with its tests) still runs, while publication may
be skipped and the run shows as failed. A broken detector can cost an
extra run, never let an untested change through the required `build`
check.

### What counts as code?

**Everything except a short list of documentation:** `docs/`,
`README.md`, `CLAUDE.md`, `LICENSE`. Listing what does *not* trigger is
safer than listing what does: a new code folder forgotten in an
allow-list would silently never be published, while a documentation file
missing from an ignore-list only causes one unnecessary run.

Renames count as code changes. The comparison ignores renames
(`--no-renames`), so a file moved from code into `docs/` is seen as a code
file deleted, and triggers a run. Manual runs (`workflow_dispatch`) stay
available.

## Results

- `7a21480` touches `.github/`: detection answered "not documentation",
  and all four jobs ran and succeeded.
- `3601f7f` touches only documentation: `build`, `deploy` and
  `verify-live` were skipped, their checks reported as completed and
  skipped, and the run as a whole succeeded.
- Not observed: that a documentation-only pull request can actually be
  merged with `build` skipped. It rests on the GitHub documentation; a
  test pull request would settle it.
