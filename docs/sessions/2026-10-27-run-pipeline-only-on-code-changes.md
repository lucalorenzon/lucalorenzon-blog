# Session 2026-10-27 — Run the pipeline only on code changes

## Goal

Stop rebuilding and republishing the site when a commit changes only
documentation, without blocking documentation-only pull requests on the
required `build` check. The user chose this card for the session instead of
the one the previous minute had planned (browser tests in the Playwright
image), which stays next.

## Decisions and why

- **Option B of the card, with the skip per job.** The workflow is always
  triggered; a first job, `changes`, lists the changed files with
  `git diff`, and `build` is skipped when all of them are documentation
  (`deploy` and `verify-live` are skipped with it). Why: according to the
  GitHub documentation, a job skipped by its own `if:` counts as a success
  for a required check, while a workflow skipped by a path filter
  (option A) leaves the check pending and would block every
  documentation-only pull request for good. Option C (a mirror workflow)
  was dropped as an easy place for a silent mistake. Full reasoning in the
  card.
- **When in doubt, run everything.** Manual runs, a first push of a
  branch, a force push, an empty change list, or a failing detection job
  all lead to `build` running. Renames count as code changes. Why: a
  wrong guess must cost an extra run, never let an untested change pass.
- **Card folders: `backlog` → `wip` → `done`.** Asked by the user. Cards
  move with `git mv`; a finished card gets the date it was finished as a
  `yyyy-MM-dd_` prefix, so `docs/done/` reads in completion order. Each
  folder keeps a `.gitkeep` so links to it never break. The rule is in
  `CLAUDE.md`, section "Backlog cards". Moving finished cards to `done`
  rather than deleting them was the user's choice.

## What was done

| Commit | Content |
|---|---|
| `633aef0` | `docs/wip/` created, the pipeline card moved there, references updated |
| `7a21480` | Workflow: `changes` job and conditional `build`; decision recorded in the card |
| `3601f7f` | Card closed in `docs/done/2026-10-27_run-pipeline-only-on-code-changes.md`; card-folder rule added to `CLAUDE.md` |
| `a2deb6f` | `.gitkeep` in `docs/backlog/` and `docs/done/` too |

The workflow code was written by a sub-agent on Sonnet and reviewed here;
its detection logic was exercised locally on sample change lists before
pushing.

**Verification on GitHub:**

- `7a21480` touches `.github/`: detection answered "not documentation", and
  all four jobs ran and succeeded.
- `3601f7f` touches only documentation: detection answered "documentation
  only"; `build`, `deploy` and `verify-live` were skipped, their checks
  reported as completed and skipped, and the run as a whole succeeded.

## Open points

- **Not yet observed:** that a documentation-only pull request can actually
  be merged with `build` skipped. Direct pushes bypass the ruleset, so this
  session could not see it; it rests on the GitHub documentation. A test
  pull request would settle it, whenever the user wants.
- **Accepted trade-off:** if the detection job itself fails, `build` still
  runs but publication may be skipped and the run shows as failed. Fine for
  now: the failure is visible and nothing untested is published.

## Next step

As planned in the previous minute: run the browser tests inside the
official Playwright image
(`docs/done/2026-10-27_run-browser-tests-in-playwright-image.md`).
