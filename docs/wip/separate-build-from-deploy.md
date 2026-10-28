# Separate the build from the deploy, publish from a tag

- **Opened:** 2026-10-27, asked by the user while the Playwright image card
  (`docs/done/2026-10-27_run-browser-tests-in-playwright-image.md`) was
  being worked on.
- **Status:** being worked on since 2026-10-28. The questions are answered
  (below); the work itself is not done yet.

## Why this was asked

Today one workflow (`.github/workflows/deploy.yml`) does everything: on
every push to `main` it checks, builds and tests the site and, if all of
that passes, publishes it straight away. Every code change merged into
`main` therefore goes live immediately; there is no moment where the
author decides "this is a version worth publishing".

The user wants publishing to be a deliberate act, separate from the
build, and a written record of what changed between one published
version and the next. Not every change is worth publishing: some do not
change what a visitor sees, others do but are not mature enough yet.

## What the user asked

- **Build.** Checks, builds and tests the site, as today, including the
  browser test on the freshly built site, so every change is still checked
  for regressions. It never publishes.
- **Deploy.** Publishes the site to GitHub Pages and then runs the browser
  test on the live site, as today. It is started by a version tag: the tag
  says "this version of the site is ready to become public".
- **CHANGELOG.** A `CHANGELOG.md` file that keeps the list of changes, so
  each published version says what it brought. Part of this card,
  confirmed by the user on 2026-10-27: once versions are introduced, a
  document tracking what each one contains belongs with them.

The user chose on 2026-10-28 to keep the three together in one card
rather than split it: they only have value together and the work on them
is connected.

## Answers

Settled with the user on 2026-10-28, one question at a time.

### Which site the deploy publishes

**Chosen: two paths.** On every push to `main`, the build, only after the
browser test on the freshly built site passed, replaces a zip of the site
in a GitHub Release named `snapshot` (a pre-release, so it is never shown
as the latest version), together with the commit it was built from. The
deploy publishes that snapshot when it was built from the tagged commit;
otherwise it rebuilds the site from the tagged commit, runs the same
checks and tests, and publishes that.

Why: the usual publication is fast and publishes exactly the site that was
already tested. Comparing the snapshot's commit with the tagged one,
rather than asking "is the tag on the last commit of `main`?", also covers
a tag pushed while the build of its commit is still running, and a tag on
a documentation-only commit, for which no build runs. The cost, accepted
by the user: the rebuild path is rarely used, so it can break without
anyone noticing until it is needed.

Rejected:

- **Always rebuild from the tagged commit.** One path, always exercised;
  no snapshot and no write permission for the build. Rejected because the
  user prefers publishing the site that was already tested, without
  waiting for a second build and its tests.
- **Reuse the site the build uploads today as a workflow artifact.**
  Workflow artifacts expire (after one day here, 90 days at most), while a
  version may wait longer than that before it is tagged. Files attached to
  a GitHub Release do not expire.

### How a version is published

**Chosen:** by hand, from the terminal: `git tag vX.Y.Z` then
`git push origin vX.Y.Z`. Only tags of that form start the deploy; the
`snapshot` tag does not.

Rejected: **creating a release from GitHub's web page**, which creates the
tag with it. The user prefers the terminal.

### Tag format

**Chosen:** `vMAJOR.MINOR.PATCH`, with a meaning defined for this site:

- **major:** a completely new design, or a different navigation;
- **minor:** an additional feature;
- **patch:** a security fix or a bug fix.

Adding an article is not a software change. While test articles live in
this repository, adding one changes the repository and counts as a patch.
Real articles will live in a separate repository, published in a way that
a future card will decide (plan, "Later").

Why: semantic versioning was designed for libraries, where a "breaking"
change is one that breaks the programs using them. A blog has no such
programs, so each number needed a meaning of its own before the format
could be used without arbitrary choices.

### Which commits may be published

**Chosen:** only tags on commits that are on `main`. A tag on any other
commit makes the deploy fail, visibly, without publishing.

Why: a commit outside `main` has not gone through a pull request, even if
its tests pass. The deploy fails rather than stopping silently, so a
misplaced tag is noticed and not mistaken for a published version. Whether
the commit passed its tests needs no separate check: a snapshot exists
only for a build that passed, and the rebuild path runs the tests again.

### GitHub settings

**Chosen:**

- the `github-pages` environment also accepts tags matching `v*` (today it
  accepts only `main`);
- a tag ruleset on `v*`: a version tag cannot be moved, and only the
  repository administrator can delete one;
- a deleted version name is never reused: the next version is used
  instead. This is a written rule, stated at the top of `CHANGELOG.md`,
  where it is seen at every publication.

Why: a version name must point to one site only, otherwise the CHANGELOG
would describe something other than what went live. Deleting a tag and
creating it again on another commit amounts to moving it, and no setting
can forbid reusing a name, because creating tags must stay allowed. The
user accepted a written rule for that case.

Rejected: **letting version tags move freely.**

### CHANGELOG

**Chosen:** `CHANGELOG.md` at the repository root, in the
[Keep a Changelog](https://keepachangelog.com/) layout, written by hand,
and checked by the deploy. A publication goes like this:

1. While working, changes are noted under "Unreleased".
2. To publish, the author turns "Unreleased" into the version's section
   (`## [1.2.0] - yyyy-mm-dd`) and commits it on `main`.
3. The build runs on that commit and saves its snapshot.
4. The author tags that commit and pushes the tag. Tagging before the
   build has finished still works, but takes the slower rebuild path.
5. The deploy fails if the tagged commit's `CHANGELOG.md` has no section
   for the tag's version. It checks the file's content, not whether the
   tagged commit changed it.

The deploy never commits: a commit made by it would come after the tagged
one, so the published site would no longer be the one the tag names.

Why: with the check, the CHANGELOG cannot fall behind; the cost is one
preparation commit before each tag. The layout's change types fit the
version numbers: an addition leads to a minor, a fix or a security patch
to a patch.

Rejected: **no check.** Publishing would need only the tag, but the
CHANGELOG could fall behind without anyone noticing.

### A GitHub Release for each version

**Chosen:** after publishing, the deploy creates a GitHub Release for the
tag. Its text is the version's section of the CHANGELOG, and the zip of
the published site is attached to it.

Why: each version gets its own page on GitHub, and each site that went
live is archived exactly, so an earlier version can be published again
from its zip without rebuilding it. Accepted costs: the deploy needs
permission to write to the repository, and the description is in two
places, the second copied automatically from the first.

Rejected: **the tag and the CHANGELOG alone.**

### Documentation-only changes

**Chosen:** nothing changes for the build, which keeps skipping
documentation-only pushes
(`docs/done/2026-10-27_run-pipeline-only-on-code-changes.md`). The deploy
never skips: the tag is already the author's decision. A tag on a
documentation-only commit takes the rebuild path, since no snapshot exists
for that commit.

`CHANGELOG.md` must stay out of the documentation-only list, so that a
version's preparation commit is built and gets its snapshot.

## Steps

Agreed with the user on 2026-10-28. Each step works on its own. The
build stops publishing in the same step where the deploy from a tag
starts, so there is never a time when nothing, or two things, publish.

1. **GitHub settings.** Done on 2026-10-28 with `gh`: the `github-pages`
   environment also accepts tags `v*`; ruleset "Version tags cannot be
   moved" (no bypass); ruleset "Only admins delete version tags" (bypass:
   repository administrators). Two rulesets, because a bypass applies to
   a whole ruleset: a single one would also let administrators move tags.
2. **Deploy from a tag, rebuild path only.** The workflow is split into
   `build.yml` (never publishes) and `deploy.yml` (from a version tag),
   `CHANGELOG.md` and the guide in `README.md` are added, and the
   environment stops accepting `main`. Then a first version is published
   from a tag, end to end; its number is chosen then. The deploy does not
   keep its own copy of the build: it calls `build.yml`, so the rebuild
   path runs on every push and cannot break unnoticed, which removes most
   of the cost accepted above for the two paths.
3. **The snapshot.** The build keeps the `snapshot` Release up to date and
   the deploy uses it when it was built from the tagged commit.
4. **A GitHub Release for each version**, with the CHANGELOG text and the
   site's zip.
