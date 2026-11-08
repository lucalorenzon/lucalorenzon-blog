# Check for newer versions of the pinned tools

- **Opened:** 2026-11-08, by the user, while making the build's external
  downloads reliable
  ([card](../wip/make-external-downloads-in-the-build-reliable.md)).
- **Status:** noticed, not started.

## Why

The site pins Tailwind at 4.3.3, while 4.3.5 was published on
2026-11-06: the user learnt it only by reading a card. Dependabot
proposes updates every week, but only for what it knows: the Rust
crates, the npm packages of the browser tests and the actions of the
workflows. Two things the build uses are pinned by hand, and nobody is
told when they have a newer version.

## Known from the start

- **Tailwind:** version and checksum in `scripts/get-tailwind.sh`,
  downloaded from GitHub's releases.
- **The list of AI robots:** commit and checksum in
  `scripts/robots-list/robots_list.py`, the file committed next to it.
  `--check-updates` already tells what would change if the pin moved,
  but only when someone runs it.
- **The daily workflow** `.github/workflows/daily.yml` runs every day
  since 2026-11-08 and was meant to hold this check as well. When a
  scheduled run fails, GitHub sends a mail to whoever last changed its
  schedule.
- The Rust toolchain follows the `stable` channel
  (`rust-toolchain.toml`), so it has no version to watch.

## Questions

- How the user is told: a failed daily run, so a mail; an issue opened
  by the job; or something else.
- Whether a newer version of the robots list is worth a signal at all,
  since it changes often and moving the pin is a choice, not a fix.
- Whether the update to Tailwind 4.3.5 is done here or in a card of its
  own, the first one this check would have raised.
