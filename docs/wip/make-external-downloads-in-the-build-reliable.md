# Make the build's external downloads reliable

- **Opened:** 2026-11-08, by the user, after the pipeline failed three
  times on the download of Tailwind
  ([session](../sessions/2026-11-08-articles-front-matter.md)).
- **Status:** being worked on, since 2026-11-08.

## Why

On 2026-11-08 the pipeline's build job failed three times in a row,
once on push and twice run again, at the same step: `scripts/build-site.sh`
downloads the Tailwind command-line tool from
`github.com/tailwindlabs/tailwindcss/releases/download/...`, and the
server answered `504`. The code was not the cause: formatting, lint and
unit tests had passed in the same job. But the build stopped there, so
the browser tests did not run, and the change of that day
([card](../done/2026-11-08_read-article-fields-from-front-matter.md))
could not be shown free of regressions in the pipeline; it was only on
the author's machine.

A build that stops because a third party's server does not answer
cannot tell a broken change from a broken network.

## Known from the start

- **What was observed on 2026-11-08:**
  - the same address answered `504` from the pipeline's machines and,
    most of the time, from the agent's commands, and `200` from the
    author's browser and terminal: an intermittent failure on GitHub's
    side, while its status page said every system was operational;
  - the release of another project downloaded with no trouble;
  - the same file, downloaded through GitHub's programming interface
    (`api.github.com/repos/tailwindlabs/tailwindcss/releases/assets/<id>`
    with `Accept: application/octet-stream`), arrived whole, with the
    checksum pinned in `scripts/build-site.sh`. Without a token, that
    interface allows 60 requests an hour per address, and the
    pipeline's machines are shared; in the pipeline the token GitHub
    Actions already provides would be needed.
- **What the build downloads from outside today:**
  - the Tailwind tool, pinned by version and checksum, kept in
    `target/tools/` on the author's machine, downloaded on every run in
    the pipeline;
  - the community list of AI robots, at a pinned commit with a checksum,
    cached in `target/tools/` (`scripts/robots-list/robots_list.py`);
  - Playwright's container image and the browser tests' npm packages.
- **Tailwind 4.3.5** was published on 2026-11-06; updating does not
  help here, since it is downloaded the same way.

## Questions

- Which downloads to make reliable: Tailwind only, or all of them.
- How: a cache in the pipeline, kept as long as the checksum matches; a
  second way to download; trying again before giving up; or more than
  one of these.
- What the build does when a download fails anyway: stop, as today, or
  tell it apart from a failure of the code.
