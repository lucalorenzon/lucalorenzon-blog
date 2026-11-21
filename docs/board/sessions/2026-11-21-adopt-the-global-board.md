# Session 2026-11-21 — Adopt the global board

**Participants:** the user (author); the agent (Claude Opus 5.5), with
two sub-agents on Claude Sonnet 5.5 (one moved the cards and rewrote the
references to them, one moved the plan and the minutes and rewrote the
references to them, each with a throwaway script outside the repository).
**Goal:** stop the links in frozen documents from breaking when a card
moves, card
[refer to cards by name in frozen documents](../cards/refer-to-cards-by-name-in-frozen-documents.md).
Technical details and reasons are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The session takes up the card on broken links, first in the plan's Now | user | — | — |
| 2 | Adopt the global board, where a card's path never changes, instead of referring to cards by name | user | agent | — |
| 3 | Rewrite the paths once, also in frozen documents, as an exception limited to this card's rewriting | user | agent | — |
| 4 | Work on a branch of its own, to start again clean if the rewriting went wrong | user | — | — |
| 5 | The card also moves the plan and the minutes, aligns the rules on minutes and updates the code's references, each in a commit of its own | user | agent | — |
| 6 | A split card is never deleted, as in the global rules, instead of the local rule that deleted it | user | agent | — |
| 7 | Of the other local rules on cards, keep the backlog's purpose and cleanup card, the usual single card in work, the title without prefix and the `.gitkeep`; drop the rules on `git mv` and on updating links at each move | agent | — | user |
| 8 | Links to cards deleted when split become their old text followed by "later split into" and links to the cards of today | user | agent | — |
| 9 | Where a split was itself split, list only the cards that exist today | user | agent | — |
| 10 | The live comment in `accessibility.spec.ts` on checking by hand points to the screen reader check and to the WCAG criteria axe cannot check | agent | — | user |
| 11 | `CLAUDE.md`, "Session minutes", reduced to a pointer to the skill `session-minute` and the three things it does not say | user | agent | — |
| 12 | Add `AGENTS.md` as a symbolic link to `CLAUDE.md` | user | agent | — |
| 13 | No card in `ll-dev-setup` for now on an `AGENTS.md` in every repository | user | agent | — |
| 14 | Close the card, bring the branch onto `main` without a merge commit, no push | user | agent | — |

## Discussion

1. **Which way to fix the links.** The agent counted 133 broken links in
   63 frozen documents and pointed out that, since the card was opened,
   the global rules had given a layout in which cards never move. The
   user judged it the more efficient answer, and asked that this
   repository align with the global rules on cards and minutes.
   → decision 2
2. **The cost of moving.** The agent showed that moving to the new layout
   would break 996 links in 205 documents unless frozen documents were
   rewritten once. The user accepted the exception for the rewriting of
   paths only, and asked for a branch of its own. → decisions 3, 4
3. **What the card covers.** The agent listed four pieces of work; its
   description of the second one, the move of the plan and the minutes,
   was badly worded and read as if their links would not be fixed. The
   user objected; the agent explained that every link is fixed in the
   first piece, and the second only moves the files. The user put all
   four in the card, in separate commits. → decision 5
4. **The rule on split cards.** Before rewriting `CLAUDE.md`, the agent
   raised that the local rule deleted a split card while the global one
   keeps it, and that deleting cards would break links again. The user
   chose the global rule. → decisions 6, 7
5. **Links to deleted cards.** After the rewriting, 24 links still led
   to nine cards deleted when split. The agent asked whether to mend
   them; the user asked to see them all before deciding, and the agent
   laid them out with the cards each split produced. The user proposed
   the "later split into" form; the agent agreed, noted that it widens
   the exception, and asked about splits made twice in a row. The user
   chose to list only the cards of today, since no card is deleted from
   now on. → decisions 8, 9, 10
6. **The rules on minutes.** The agent compared the local section with
   the skill `session-minute`, found no contradiction, and proposed
   keeping only what the skill does not say. The user agreed.
   → decision 11
7. **`AGENTS.md`.** The user asked for it as a completion of the
   alignment. The agent added it and noted that the global rules do not
   ask for it in each repository yet; the user chose not to open a card
   in `ll-dev-setup` for now. → decisions 12, 13

## Outcome

- The card taken into work and translated into the card format
  (`a1af041`), its answers recorded (`d145520`).
- The 136 cards in `docs/board/cards/` with their links, about a
  thousand references rewritten, the links to deleted cards replaced,
  `CLAUDE.md` aligned on cards (`de5aa55`).
- The plan and the 67 minutes in `docs/board/` (`813342f`).
- `CLAUDE.md`, "Session minutes", left to the skill (`13574d0`).
- `AGENTS.md` linked to `CLAUDE.md` (`c281236`).
- The card closed and taken out of the plan's Now (`2b6aeb9`); the branch
  `adopt-the-global-board` brought onto `main` and deleted. Nothing
  pushed.
- Outside the repository: the agent's memory note on cards for small
  fixes updated to the new layout.

## Next steps

No new action came out of this session. The plan's Now continues with
[decide whether to cut the text column again](../cards/decide-whether-to-cut-the-text-column-again.md).
