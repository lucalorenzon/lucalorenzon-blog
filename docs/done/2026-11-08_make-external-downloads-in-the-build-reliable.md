# Make the build's external downloads reliable

- **Opened:** 2026-11-08, by the user, after the pipeline failed three
  times on the download of Tailwind
  ([session](../sessions/2026-11-08-articles-front-matter.md)).
- **Status:** done, on 2026-11-08.

## Why

On 2026-11-08 the pipeline's build job failed three times in a row,
once on push and twice run again, at the same step: `scripts/build-site.sh`
downloaded the Tailwind command-line tool from
`github.com/tailwindlabs/tailwindcss/releases/download/...`, and the
server answered `504`. The code was not the cause: formatting, lint and
unit tests had passed in the same job. But the build stopped there, so
the browser tests did not run, and the change of that day
([card](2026-11-08_read-article-fields-from-front-matter.md)) could not
be shown free of regressions in the pipeline; it was only on the
author's machine.

A build that stops because a third party's server does not answer
cannot tell a broken change from a broken network.

What was observed that day: the release address answered `504` from
the pipeline's machines and, most of the time, from the agent's
commands, and `200` from the author's browser and terminal, while
GitHub's status page said every system was operational. The same file
arrived whole through GitHub's programming interface.

## Answers

### Which downloads?

Tailwind and the community list of AI robots: both come from GitHub,
so they share the same risk.

- **Rejected for now:** Playwright's container image and the browser
  tests' npm packages. They come from other providers (Microsoft and
  npm) and have never failed so far.

### How?

Each of the two in the way that fits its size.

**The list of AI robots is committed in the repository**
(`scripts/robots-list/ai-robots-txt/`, `robots.json` with the project's
`LICENSE`), and `scripts/robots-list/robots_list.py` reads it from
there, checking the same pinned checksum. Writing the blocked list no
longer uses the network; only `--check-updates` does, and it now also
says where to download the new file and where to put it.

- **Why:** the file weighs 90 KB and changes only when the pin is moved
  on purpose. The user proposed committing both downloads.
- **Its licence allows it:** the agent read it at the source, at the
  pinned commit: MIT, which requires the copyright and permission
  notice to travel with the copy, hence the `LICENSE` next to it.

**Tailwind is kept in GitHub's cache**, and fetched by a script of its
own, `scripts/get-tailwind.sh`, which owns the pinned version and
checksum:

- **The cache** is keyed on version and checksum, printed by
  `scripts/get-tailwind.sh --cache-info`, so they are written in one
  place only. It accepts only the exact pinned file: there is no
  fallback to another version, even when the download fails. The user
  asked about a fallback to a version already in the cache; it was
  rejected because the commit saying one version would build the site
  with another, unnoticed. The steps that restore, get and save the
  tool are a local action, `.github/actions/get-tailwind/`, used by the
  build and by the daily job.
- **A daily job** (`.github/workflows/daily.yml`) restores the cache
  every day, because GitHub removes a cache nobody uses for seven days,
  and the build runs only on code changes. The user proposed it, every
  six days; it runs daily because GitHub may delay or skip scheduled
  runs, so one skipped run would lose the cache.
- **The download is retried** (six times, fifteen seconds apart), for
  the moments the cache is empty: a new version, or a cache lost by
  GitHub.
- **A file already there is checked** against the pinned checksum
  before use, so a broken or tampered file, for example from the
  cache, is downloaded again.

**Rejected:**

- **Committing Tailwind too:** the tool weighs about 107 MB, and GitHub
  refuses files over 100 MB. With Git LFS the pipeline would download
  it from GitHub again, the same provider that failed, and every new
  version would add about 100 MB to the history for good; the whole
  history weighed 22 MB.
- **A second way to download Tailwind**, through GitHub's programming
  interface, which worked on 2026-11-08: it needs the pipeline's token
  and adds code for a case seen once. It is a
  [card of its own](../backlog/add-a-second-way-to-download-tailwind.md),
  to pick up if the build stops again despite the cache and the
  retries.

### What does the build do when a download fails anyway?

It stops, as before: without Tailwind the site has no stylesheet and
the browser tests cannot run. But only when there is no valid file and
the download fails after every attempt, and the failure can now be
told apart from one of the code:

- Tailwind is fetched in a step of its own, "Get Tailwind, from the
  cache or downloaded", after formatting, lint and the Rust tests, so
  those already show as passed, and the failed step names the cause.
- The error says plainly that it is a network problem on the server's
  side, not a problem in the code, and to run the build again later.

## Results

Pushed on 2026-11-08, commits `fefc303` (robots list), `68d86f7`
(`get-tailwind.sh`), `e76d630` (cache), `013cca9` (daily job and local
action):

- the build of the push did not find the cache, downloaded Tailwind
  and saved it (about 38 MB compressed);
- a second build, run by hand, found it in the cache and downloaded
  nothing;
- the daily job, run by hand, found it in the cache too;
- Dependabot accepted its new configuration, which also covers the
  local action (`directories`, syntax checked in GitHub's
  documentation).

Checked by hand before the push: with a valid file and the network
blocked, `get-tailwind.sh` does not try to download; with a broken
file, it downloads again; with no file and the network blocked, it
stops after about a minute and a half with the network message and
leaves nothing half-written. The blocked list of robots is identical
before and after the change.

Opened along the way: [check for newer versions of the pinned
tools](2026-11-08_check-for-newer-versions-of-pinned-tools.md), since
Tailwind 4.3.5 had been out since 2026-11-06 without anyone noticing.
