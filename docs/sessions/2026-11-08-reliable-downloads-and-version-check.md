# Session 2026-11-08 — Reliable downloads in the build, and a check for newer versions

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agents Sonnet 5.5 (the code of every change) and Haiku 5.5 (the
first card's move, the two pushes and the checks on GitHub).
**Goal:** take the card
[make the build's external downloads reliable](../done/2026-11-08_make-external-downloads-in-the-build-reliable.md),
first in Now after the pipeline failed on Tailwind's download. Two more
cards came out of it and were done in the same session:
[check for newer versions of the pinned tools](../done/2026-11-08_check-for-newer-versions-of-pinned-tools.md)
and
[silence the expected error in the robots list's tests](../done/2026-11-08_silence-expected-error-in-robots-list-tests.md).
Reasons and rejected options are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Make reliable Tailwind and the list of AI robots; Playwright's image and npm left out | user | agent | — |
| 2 | The list of AI robots is committed in the repository, with its MIT licence | user | agent | — |
| 3 | Tailwind is not committed: over GitHub's 100 MB limit, and Git LFS downloads from GitHub again | user | agent | — |
| 4 | Tailwind kept in GitHub's cache, with retried downloads; a second way to download in a card of its own | user | agent | — |
| 5 | A daily job keeps the cache alive, every day rather than every six | user | agent | — |
| 6 | The build stops only with no valid file and a failed download; Tailwind fetched in a step of its own; only the pinned version, never another one from the cache | user | agent | — |
| 7 | The check for newer versions is a card of its own, done right after | user | agent | — |
| 8 | GitHub's documentation read again after a blocked attempt, with the source repository as a fallback | user | agent | — |
| 9 | The cache's steps in a local action shared by the build and the daily job; the daily workflow named `daily.yml` | agent | — | user |
| 10 | Two pushes, with manual runs of the build and the daily job | user | agent | — |
| 11 | Newer versions are reported by an issue, not by a failed run | user | agent | — |
| 12 | Tailwind: one issue per version; closing it skips that version | user | agent | — |
| 13 | The robots list: an issue only when the blocked robots would change, the open one kept up to date | user | agent | — |
| 14 | The robots list's issue carries its commit, so closing it means "not now" | user | agent | — |
| 15 | The update to Tailwind 4.3.5 in a card of its own, after the check, as its proof | user | agent | — |
| 16 | The noisy test fixed now, Tailwind's update in another session | user | agent | — |
| 17 | The same fix applied to three tests of the content dates step | agent | — | user |

## Discussion

1. **What to make reliable, and how.** The agent set out what the build
   downloads and proposed Tailwind and the robots list, which share the
   provider that failed; the user agreed. The agent proposed a cache
   with retries; the user proposed committing both files instead. The
   agent found that Tailwind weighs about 107 MB, over GitHub's limit,
   and read the robots list's licence at the source: MIT. The user
   asked how the cache works; after the explanation, the user proposed
   a job refreshing it every six days, and the agent proposed every day,
   since GitHub may skip a scheduled run. → decisions 1 to 5
2. **What a failed download does.** The agent proposed stopping the
   build, with Tailwind in a step of its own and a message naming the
   network. The user pointed out that a tool already in the cache needs
   no download; the agent confirmed it was the plan, and asked whether
   another version in the cache should be used when the pinned one
   cannot be downloaded. The user chose the pinned version only.
   → decision 6
3. **A version nobody noticed.** The user asked for a check of
   Tailwind's version too, having learnt of 4.3.5 from the card. The
   agent noted it was a different decision from the card's and proposed
   a card of its own, right after; the user agreed. → decision 7
4. **A blocked read.** Checking Dependabot's new configuration, a
   sub-agent could not read GitHub's documentation: the site's safety
   check did not answer. It stopped and reported; the agent explained
   it to the user and proposed trying again, then the documentation's
   source on GitHub. The user agreed; the second attempt worked and
   confirmed the syntax. → decision 8
5. **The pushes.** The cache was verified on GitHub: the first build
   downloaded and saved Tailwind, a second build and the daily job found
   it. GitHub reported, at each push, that the rule requiring the two
   checks on `main` was bypassed, as happens when the owner pushes to
   `main` directly; the agent told the user. → decisions 9, 10
6. **How to be told.** The agent proposed a failed daily run, so a
   mail, as the simplest; the user chose an issue. The agent proposed
   one issue per version, so closing one skips a version; the user
   agreed. For the robots list, which changed 29 times in three months,
   the agent proposed a signal only when the blocked robots would
   change, which needed an exception, one issue kept up to date; the
   user chose it. The agent then found that a closed issue would come
   back the next day, and proposed the commit in the issue's marker;
   the user agreed. → decisions 11 to 14
7. **Proving the check.** The agent proposed updating Tailwind only
   after the check was live, so that its first issue would prove it:
   the daily job opened issue #33, and the user received the
   notification. → decision 15
8. **A misleading log.** In the daily job's log the agent found an
   error message printed by a passing test, which had misled a
   sub-agent. The user had it fixed in the session; the sub-agent found
   the same in the content dates step's tests and fixed them too.
   → decisions 16, 17

## Outcome

- Downloads: robots list in the repository `fefc303`;
  `scripts/get-tailwind.sh` `68d86f7`; cache in the build `e76d630`;
  daily job and local action `013cca9`. On GitHub: build of the push
  run [42488793351](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/42488793351)
  (cache missed, Tailwind saved), manual build
  [42489182808](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/42489182808)
  and daily job
  [42489544699](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/42489544699)
  (cache found). The browser tests ran in the pipeline on the previous
  session's change too, which closes that session's open point.
- Version check: the check, read-only, `c4833b9`; the issues and the
  daily job `9f1518b`. The daily job opened
  [issue #33](https://github.com/lucalorenzon/lucalorenzon-blog/issues/33)
  on its first run (run
  [42491957048](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/42491957048))
  and nothing on its second.
- Tests: `c0ea24f`; the four Python test suites print only their
  results.
- Cards: started `8d99ab2`, `571bf3a`, `4ec2b3b`; opened `3cd5950`;
  closed `55ac46d`, `d5b4595` (with the cards it raised), `6a1c68a`;
  plus this minute.

## Open points

- The robots list's issue was checked by unit tests only: the list had
  no commit after the pin. Its first real issue will be the proof.
- The workflows were checked by running them on GitHub; `actionlint`,
  which would check them before a push, is not installed (card
  [check the workflows with actionlint](../backlog/lint-workflows-with-actionlint.md)).

## Next steps

- Update Tailwind to 4.3.5 and close issue #33: card
  [update Tailwind to 4.3.5](../backlog/update-tailwind-to-4-3-5.md).
- A second way to download Tailwind, if the build stops again on its
  download: card
  [add a second way to download Tailwind](../backlog/add-a-second-way-to-download-tailwind.md).
