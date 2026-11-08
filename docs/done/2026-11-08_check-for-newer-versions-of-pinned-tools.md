# Check for newer versions of the pinned tools

- **Opened:** 2026-11-08, by the user, while making the build's external
  downloads reliable
  ([card](2026-11-08_make-external-downloads-in-the-build-reliable.md)).
- **Status:** done, on 2026-11-08.

## Why

The site pinned Tailwind at 4.3.3, while 4.3.5 had been published on
2026-11-06: the user learnt it only by reading a card. Dependabot
proposes updates every week, but only for what it knows: the Rust
crates, the npm packages of the browser tests and the actions of the
workflows. Two things the build uses are pinned by hand, and nobody was
told when they had a newer version:

- **Tailwind:** version and checksum in `scripts/get-tailwind.sh`;
- **the list of AI robots:** commit and checksum in
  `scripts/robots-list/robots_list.py`, the file committed next to it.
  `--check-updates` already told what would change if the pin moved,
  but only when someone ran it.

The Rust toolchain follows the `stable` channel, so it has no version
to watch.

## Answers

### How is the user told?

With an issue in the repository, opened by a second job of the daily
workflow (`.github/workflows/daily.yml`), with the label
`version-check`. The user chose it.

- **The job stays green when it finds something:** the issue is the
  signal. It fails, and so GitHub mails the user, only when the check
  itself cannot run.
- **Rejected:** a daily run that fails when something is newer, so a
  mail. It was the agent's first proposal, as the simplest; but the
  mail comes back every day until the update, and the run shows red
  when nothing is broken.

### When is an issue opened, for Tailwind?

One issue per newer version, with the version in the title. Each issue
carries a hidden marker in its body with the tool and the version, so
the job recognises it: if an issue for that version exists, open or
closed, the job does nothing. Closing the issue without updating means
"skip this version"; the next issue comes with the next version.

- **Rejected:** one issue per tool, updated at each new version. Closing
  it without updating would have it reopened the next day, unless more
  code recorded the versions to skip.

### Is the list of AI robots worth a signal?

Only when moving the pin would change the list of blocked robots. The
list changed 29 times in the three months before 2026-11-08, two or
three times a week, and most changes do not touch the robots the site
blocks.

- **One issue at a time, kept up to date:** the list has no named
  versions, only commits. If an issue for the latest commit exists,
  open or closed, the job does nothing; if an older one is open, the
  job updates its title and text; otherwise it opens one. Closing it
  without moving the pin means "not now": the job comes back only when
  the list changes again in a way that touches the blocked robots. The
  agent pointed out that, without the commit in the marker, a closed
  issue would come back the next day; the user agreed.
- **Rejected:** no signal, as it was: the forgetting that happened with
  Tailwind. A signal at most once a month for any change: it would
  report changes that do not touch the site.

### Is the update to Tailwind 4.3.5 done here?

No: in a [card of its own](2026-11-08_update-tailwind-to-4-3-5.md),
after the check. Updating first would have left the check nothing to
find, so no proof that it works.

## How it works

- `scripts/get-tailwind.sh --version` prints the pinned version, so it
  stays written in one place.
- `scripts/robots-list/robots_list.py --check-updates --json` gives its
  result to another program. Requests to GitHub's programming interface
  carry the pipeline's token when there is one, and only to that host.
- `scripts/version-check/version_check.py` finds what is newer and
  decides what to do with the issues, in a function of its own covered
  by the unit tests. Without arguments it only tells what it would do;
  with `--apply`, as in the daily job, it does it.

## Results

Pushed on 2026-11-08, commits `c4833b9` (the check, read-only) and
`9f1518b` (the issues and the daily job):

- run by hand, the daily job opened
  [issue #33](https://github.com/lucalorenzon/lucalorenzon-blog/issues/33),
  "Tailwind 4.3.5 is available (pinned: 4.3.3)"; the user received the
  notification;
- run a second time, it found issue #33 and opened nothing;
- the list of AI robots had no commit after the pin, so its finding was
  checked by the unit tests only.

Noticed along the way: a unit test of the robots list prints an
expected error message in the job's log, which reads like a failure
([card](2026-11-08_silence-expected-error-in-robots-list-tests.md)).
