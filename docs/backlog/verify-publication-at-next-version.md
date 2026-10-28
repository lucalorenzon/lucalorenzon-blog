# Verify the publication at the next version

- **Opened:** 2026-10-28, when the card
  [separate the build from the deploy](../done/2026-10-28_separate-build-from-deploy.md)
  was closed.
- **Status:** noticed, not started. Done together with the next version
  that changes the site; no version is made only for this check.

## Why

Two parts of the publication were built but have not run on GitHub yet,
because no version was published after them:

- the deploy publishing the `snapshot` of the tagged commit instead of
  rebuilding the site;
- the deploy creating the GitHub Release of the version, with its
  CHANGELOG section as text and the zip of the published site.

The deploy's refusals (malformed tag, commit outside `main`, missing
CHANGELOG section) have not been seen on GitHub either.

## What to check at the next version

1. Wait for the build of the preparation commit before tagging, so the
   snapshot of that commit exists.
2. In the deploy run: the job `snapshot` says the snapshot was found, and
   the job `build` is skipped.
3. The Release `vX.Y.Z` exists, is marked as latest, its text is that
   version's CHANGELOG section only, and `site-vX.Y.Z.zip` holds the
   published site.

The refusals can be checked on the same occasion only if it costs
nothing; otherwise they stay unseen until one happens by mistake, when
the deploy is expected to fail visibly.
