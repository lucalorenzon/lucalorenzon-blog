# Session 2026-10-30 — Run a screen reader check on demand

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Sonnet 5.5 (the command, its fixes and its clean-up) and
Haiku 5.5 (a type annotation).
**Goal:** close the card
[run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md).
Technical details and reasons are in the card. The session came right
after the session
[check the accessibility tree in the build](2026-10-30-check-accessibility-tree-in-the-build.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Captured transcript compared automatically, plus the user's answers on the sound; a timeboxed trial first | user | agent | — |
| 2 | The agent starts Orca on the user's desktop, with the user present | user | agent | — |
| 3 | Records in `docs/accessibility/` | user | agent | — |
| 4 | The four questions, with "Lvk@73r" and "Home Page" as accepted readings | user | agent | — |
| 5 | On demand, no fixed rule; a changed transcript accepted or recorded as a regression by the user; the command never commits | agent | — | user |
| 6 | New card: cover the WCAG criteria axe cannot check, in Later | user | agent | — |
| 7 | Keep the local command for now; new card to run the check in a container, on any computer, in Next | user | agent | — |
| 8 | Clear the editor's findings on the Python files before committing | user | agent | — |

## Discussion

1. **Shape of the check.** The agent found in Orca 50.3 a debug file
   with every utterance and a D-Bus remote control, and proposed a
   captured transcript plus the user's answers, after a trial. The user
   agreed: it was what they wanted. → decision 1
2. **The trial.** With the user's consent the agent started Orca. Orca
   did not see the user's Firefox (started before Orca) nor Zed; a
   separate Firefox did. `SayAll` read the terminal, then only the
   footer; the log showed Orca had already read the page whole on load.
   The agent cleaned up everything it had started. → decision 2
3. **Design.** One question at a time: where the records live, which
   questions. The user said the logo reading and "Home Page" sound
   right, so they became expected readings. → decisions 3, 4, 5
4. **ARIA and WCAG.** The user asked whether ARIA is complete and
   whether axe detects wrong or missing ARIA. The agent listed what axe
   checks and that structural rules are only "best practice", off in
   the build. The user pointed out that the aim is the WCAG criteria,
   not best practice; the card was written that way. → decision 6
5. **First runs failed.** Firefox did not open (snap not detected
   behind Ubuntu's launcher), then the reference could not be saved
   (folder missing, after the agent had deleted the failed records).
   Each was reproduced by a failing test before the fix. → decision 5
6. **Fragile and local.** The user observed that the check is fragile
   and runs only on this computer. The agent agreed, listed why, and
   proposed a container; the user wants it to work anywhere, but not
   urgently. The agent read this as keeping the local command until
   then, and said so. → decision 7
7. **Editor findings.** The user saw 5 errors and 10 warnings on the
   Python files; the agent reproduced them with the editor's own tools.
   The first clean-up introduced 2 new errors, which the agent's
   command-line check had missed because it could not resolve the
   test's import; reproduced with the editor's search path and fixed. →
   decision 8

## Outcome

- `scripts/screen_reader_check.py` with 23 unit tests; first reference
  and run record in `docs/accessibility/`; on GitHub, run
  [41019543750](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41019543750)
  passed.
- Commits: `2cbdc1e` card to `wip`, `a99a821` trial recorded, `3a47fdf`
  design agreed, `d9d4893` WCAG card, `815db4b` container card,
  `1d952b1` the command, `97569ec` first reference, `ad7fa61` card
  closed, plus this minute.

## Open points

- The command's unit tests run locally only, not in the build.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Make the screen reader check work on any computer (decision 7): card
  [run the screen reader check in a container](../backlog/run-screen-reader-check-in-a-container.md).
- Cover the WCAG criteria axe cannot check (decision 6): card
  [cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md).
