# Session 2026-10-30 — Run the screen reader check in a container

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Haiku 5.5 (moving the card, building the site) and Sonnet 5.5
(the trial files, the container command).
**Goal:** open the card
[run the screen reader check in a container](../wip/run-screen-reader-check-in-a-container.md)
and answer its first question with a timeboxed trial. Technical details
and reasons are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Open the container card | user | agent | — |
| 2 | Trial: one hour at most; success if Orca reads the page in the container | user | agent | — |
| 3 | Trial on Debian trixie, Xvfb, openbox, Orca before Firefox | user | agent | — |
| 4 | Trial files copied into the card when the user paused | user | agent | — |
| 5 | The container's reading is the reference, not the desktop's | user | agent | — |
| 6 | Orca 51.1 from trixie-backports | user | agent | — |
| 7 | New card for the logo's name not read by Orca 51, placed in Now | user | agent | — |
| 8 | Languages checked in the build (new card, in Next); Orca run on demand, not in the pipeline | user | agent | — |
| 9 | The local command is removed by this card; the container writes the audio to a file | user | agent | — |
| 10 | Remaining work cut in three slices, local command last | user | agent | — |
| 11 | Files in `scripts/screen-reader/` | user | agent | — |
| 12 | Slice 1 choices: only Orca pinned, transcript from the landmark summary, `sudo docker` fallback, first container run as reference | user | agent | — |
| 13 | `nss_wrapper` for the host uid; the failed run's record entry removed | agent | — | user |
| 14 | Slice 1 committed with a broken end-of-reading signal, fixed in slice 2 | user | agent | — |

## Discussion

1. **Opening the card.** The agent gave Haiku instructions that ran
   `git pull --rebase` on a tree with uncommitted changes; the pull
   failed and Haiku stopped as told. The agent checked the remote by
   other means, found nothing new, and committed. → decision 1
2. **The trial.** The user set one hour and a plain criterion: Orca
   reads the page. The agent chose Debian trixie because Firefox there is
   not a snap. Run 1: Orca announced the page and went silent. Run 2,
   with a stack dump, showed Orca stuck waiting on speech-dispatcher,
   which had no sound device. Run 3, with a null audio sink, read the
   page whole, in about 17 minutes of the hour. The user paused once;
   the agent warned that the trial files lived only in the session's
   temporary folder, and the user had them copied into the card. The
   agent made the small script fixes between runs itself, not through a
   Sonnet sub-agent as the model routing asks, to save time inside the
   timebox. → decisions 2, 3, 4
3. **Which Orca.** The user asked why Orca 48: it came with trixie. The
   user asked for a trusted prebuilt Orca 50 for trixie; the agent found
   none in Debian's archive, but Orca 51.1 in trixie-backports. The
   agent pointed out that 51 would not match the desktop either; the user
   made the container the reference and chose 51 from the backports.
   The first build failed on a package mix, fixed by taking
   speech-dispatcher from the backports too. → decisions 5, 6
4. **The logo.** With Orca 51 the logo is read as "code start", "code
   end", without its name, while the accessibility tree and Firefox give
   the right name. The agent flagged it as a possible defect, with 75%
   confidence (the version mix in the image is not ruled out). The user
   had it recorded as a card, in Now. → decision 7
5. **What the check is for.** The user asked whether this was a manual
   test, and whether listening was their job at all. The agent explained
   what the site controls (structure, names, languages) and what the
   visitor's software controls (the voice). The user asked whether Orca
   was then needed, since the build already has the accessibility tree,
   and whether the logo case was only a missing ARIA attribute; the
   agent showed the attributes were present and correct, the case the
   tree cannot see. The agent proposed languages in the build and Orca on
   demand, rejecting Orca in the pipeline; the user agreed. → decision 8
6. **The local command.** The agent named the tension: the local
   command exists for listening, now occasional. The user had it removed
   and the audio written to a file instead. The agent read "removed" as
   "by this card, once the container works", so the site is never
   without a check. → decision 9
7. **Slice 1.** The agent proposed three slices and, for the first, a
   folder and five minor choices at once; the user accepted all. Sonnet
   wrote the command and its tests. First run: dbus-daemon refused a uid
   with no passwd entry; the agent fixed it with `nss_wrapper` and
   removed the failed run's entry from the record, as a setup failure
   rather than a check of the site. Second run: Orca read the page and
   the user accepted the new reference. The end-of-reading signal did
   not work, so the run ended at its 90-second maximum; the agent
   proposed committing anyway and fixing the signal in slice 2, from the
   silence of the recorded audio. → decisions 10 to 14

## Outcome

- The container reads the site with Orca 51.1 and is the reference:
  `docs/accessibility/screen-reader-transcript.txt`, first container
  run recorded in `docs/accessibility/screen-reader-checks.md`.
- New command `python3 scripts/screen-reader/check.py`, 23 unit tests;
  the local command's 23 tests still pass.
- Commits: `52cc63b` card opened, `8da3a2f` and `047df8e` the trial,
  `3f0077c` logo card, `be41675` plan, `98d356d` reference and Orca
  version, `a41c5f7` languages card and Orca on demand, `ad53d95` local
  command and audio, `7b06eeb` slice 1, plus this minute.

## Open points

- Whether the logo's missing name is Orca 51's own behaviour or the mix
  of Firefox from trixie with libraries from the backports.
- Orca 51 spends about seven seconds looking for a braille display
  before reading on.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Slice 2, the audio written to a file and the end of the reading found
  from its silence (decisions 9, 14); slice 3, the local command removed
  (decision 9): card
  [run the screen reader check in a container](../wip/run-screen-reader-check-in-a-container.md).
- Find out why Orca 51 skips the logo's name, and fix the markup if
  needed (decision 7): card
  [logo name not read by Orca 51](../backlog/logo-name-not-read-by-orca-51.md).
- Check the declared languages in the build (decision 8): card
  [check the declared languages in the build](../backlog/check-declared-languages-in-the-build.md).
