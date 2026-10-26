# Backlog: run build, deploy and tests only when the code changes

- **Opened:** 2026-10-26
- **Status:** proposed, not decided.

## Why this was asked

Every push to `main` runs the whole workflow (`.github/workflows/deploy.yml`):
checks, build, browser test, publication, browser test on the live site.
Commits that only touch documentation (`docs/`, `README.md`,
`CLAUDE.md`) change nothing on the site, yet they rebuild and republish
it. It happened right away: the session minute and the backlog cards of
2026-10-26 each triggered a full run.

The cost is runner minutes and waiting time, and exposure to occasional
slowdowns such as the 9-minute one recorded in
`docs/backlog/run-browser-tests-in-playwright-image.md`, all for a
publication identical to the previous one.

## The catch: the required `build` check

The `main` ruleset ("Require CI on main") requires the `build` check. The
GitHub documentation states that when a workflow is skipped because of a
path filter, its checks stay "Pending", and a pull request that requires
them cannot be merged. So the simplest fix would block every
documentation-only pull request. Direct pushes from the author are not
affected, because the author's account bypasses the rule.

## Options

**A. `paths-ignore` on the workflow triggers** (`docs/**`, `**.md`, …).
- Pro: one small change; documentation pushes no longer run anything.
- Con: documentation-only pull requests are blocked forever by the
  pending `build` check. Acceptable only while all documentation goes in
  by direct push.

**B. Always start the workflow, skip the heavy steps when only
documentation changed.** A first step detects which files changed; the
build, test and publish steps run only if a code file is among them. The
`build` check is then always reported, as success when skipped.
- Pro: pull requests are never blocked; the rule keeps protecting code
  changes.
- Con: the workflow gets more conditions to read and maintain; change
  detection needs either a small script on `git diff` or a third-party
  action (one more dependency to pin and keep updated).

**C. A second, tiny workflow for documentation-only changes** that
reports a `build` check with the same name and does nothing else.
- Pro: the main workflow stays simple.
- Con: two workflows that must stay mirror images of each other (same
  check name, opposite path filters); an easy place for a silent mistake.

## What counts as code

Better to list what does *not* trigger (documentation) than what does: a
new code folder added later and forgotten in an allow-list would silently
never be published, while a documentation file missing from an
ignore-list only causes one unnecessary run. Starting point for the
ignore-list: `docs/**`, `README.md`, `CLAUDE.md`, `LICENSE`. Everything
else (`src/`, `style/`, `scripts/`, `end2end/`, `Cargo.*`,
`rust-toolchain.toml`, `.github/`) keeps triggering. Manual runs
(`workflow_dispatch`) stay available in every option.

## When to pick this up

Whenever the user wants; it is independent of the other cards. It may
also come up in the `residuality` review
(`docs/backlog/challenge-skeleton-with-residuality.md`).
