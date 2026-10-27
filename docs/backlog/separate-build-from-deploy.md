# Backlog: separate the build from the deploy, publish from a tag

- **Opened:** 2026-10-27
- **Status:** noticed, not started. Asked by the user while the
  Playwright image card
  (`docs/done/2026-10-27_run-browser-tests-in-playwright-image.md`) was being worked
  on.

## Why this was asked

Today one workflow (`.github/workflows/deploy.yml`) does everything: on
every push to `main` it checks, builds and tests the site and, if all of
that passes, publishes it straight away. Every code change merged into
`main` therefore goes live immediately; there is no moment where the
author decides "this is a version worth publishing".

The user wants publishing to be a deliberate act, separate from the
build, and a written record of what changed between one published
version and the next.

## What the user asked

- **Build.** Checks, builds and tests the site, as today, including the
  browser test on the freshly built site that today runs before
  publishing. It never publishes.
- **Deploy.** Does what it does today: publishes the site to GitHub Pages
  and then runs the browser test on the live site. It could be started by
  a tag (for example `v1.2.0`) instead of by the end of the build.
- **CHANGELOG.** A `CHANGELOG.md` file that keeps the list of changes,
  so each published version says what it brought.

## Questions to settle before starting

Not decided yet; listed so they are discussed one at a time when the card
is picked up.

- **Which site does the deploy publish?** It can rebuild the site from the
  tagged commit (simple, but the build and its tests run again), or reuse
  the site built by the build run of that commit (no second build, but
  build artifacts expire after a while and must be found by commit).
- **Tag format and who creates it.** For example `vMAJOR.MINOR.PATCH`,
  created by hand with `git tag` + `git push`, or through a GitHub
  release.
- **Which commits may be published.** Only tags on commits that are on
  `main` and whose build passed, or any tag.
- **GitHub settings to change.** The `github-pages` environment limits
  which branches may deploy (today only `main`); a tag-triggered deploy
  needs a rule for the tags. Who may push tags may need a ruleset too.
- **CHANGELOG format and how it is kept.** For example the "Keep a
  Changelog" layout, written by hand; whether each tag requires a
  CHANGELOG entry; whether the entry also becomes the text of a GitHub
  release. The CHANGELOG is grouped here because it is tied to published
  versions, but it is a separate decision and can become its own card.
- **Documentation-only changes.** The skip added on 2026-10-27
  (`docs/done/2026-10-27_run-pipeline-only-on-code-changes.md`) keeps
  working for the build; with a manual tag it is no longer needed for the
  deploy.

## When to pick this up

Whenever the user wants, after the Playwright image card is done, since
both change the same workflow. It may also come up in the `residuality`
review (`docs/backlog/challenge-skeleton-with-residuality.md`).
