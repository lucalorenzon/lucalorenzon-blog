# Check the workflows with actionlint

- **Opened:** 2026-10-28, while fixing the live-site test after a
  snapshot publication
  ([card](../done/2026-10-28_run-live-test-when-snapshot-is-published.md)).
- **Status:** noticed, not started.

## Why

The two workflows in `.github/workflows/` are only checked by running
them: a mistake shows up after a push, or after a version tag. The agent
has mentioned [actionlint](https://github.com/rhysd/actionlint) more than
once as a check it could not run, because it is not installed.

actionlint reads a workflow without running it and finds: syntax errors,
wrong `${{ }}` expressions, wrong inputs to actions, and, through
`shellcheck`, mistakes in the scripts of `run:` steps. It does not find
which jobs end up skipped, so it would not have found the bug of the
card above.

## Proposal

- Run it in the pipeline, on every change to `.github/`, rather than
  installing it on one machine: then the check does not depend on who
  works or where.
- Open questions: as a job of `build.yml` or a workflow of its own; a
  required check or not; how to pin its version, as the other tools are
  pinned.
