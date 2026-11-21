# Refer to cards by name in frozen documents

- **Created:** 2026-11-01
- **Status:** Closed
- **Closed:** 2026-11-21

## Why

Session minutes and finished cards are frozen: once committed they are not
edited. Yet they link to other cards by path, and a card's path changes
every time it moves between `docs/backlog/`, `docs/wip/` and `docs/done/`.
Each move therefore breaks links that, by rule, nobody may fix. Keeping
such links current by hand would mean editing frozen records on every
move, which does not scale.

## What is known

- **How it showed up.** On 2026-11-01, moving the card on the site's
  address (`choose-site-address`) to `docs/wip/`: three live documents
  were updated, while seven frozen ones (three finished cards, four
  minutes) kept pointing at `backlog/choose-site-address.md`, which no
  longer exists.
- **How it grew.** On 2026-11-21, when this card was taken into work, a
  count of the relative links to Markdown files that lead nowhere gave
  133 broken links in 63 frozen documents: 98 in minutes, 35 in
  finished cards. Taking this card into work broke four more: four
  minutes link to it in `docs/backlog/`.
- **The proposal, from the user.** In minutes and in finished cards,
  refer to other cards by their name, not by a link.
- **A second way, found on 2026-11-21.** Since this card was opened, the
  global rules (`~/.claude/CLAUDE.md`, "The Board") gave a layout in
  which a card never moves: every card lives in
  `docs/board/cards/<name>.md`, and its state is a symbolic link in a
  state folder. A path that never changes cannot break. This repository
  overrides that layout in its own `CLAUDE.md`, "Backlog cards", with
  cards that move between folders.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which way: refer to cards by name, or give cards a path that never
  changes.** The card was written before the global rules gave the
  second way. Referring by name keeps the layout and drops clickable
  links from frozen documents; the global layout keeps the links and
  changes where every card, the plan and the minutes live.

  **Answered by the user on 2026-11-21.** The global layout, as the more
  efficient answer: it removes the cause, not the symptom. This
  repository aligns with the global rules wherever they speak of cards
  and minutes, and its `CLAUDE.md` keeps only what the global rules do
  not say.
- **What "name" is.** The file name without folder and date prefix
  (`choose-site-address`) is the same in all three folders and finds the
  card with a single search; the title (`Choose the site's public
  address`) reads better but can change.

  **Answered by the user's choice of the global layout, 2026-11-21.** The
  file name without date prefix, which is the card's file name in
  `docs/board/cards/`; a document refers to a card by that path. On
  2026-11-21 no two cards turned out to share a name once the date
  prefix of the finished ones is dropped.
- **Rules that contradict it today.** `CLAUDE.md`, "Session minutes", asks
  for "a link to the card" in the goal and in each next step, and says
  "What a card already says is linked, not repeated". Those lines, and the
  reference minute they point at, change with this decision.

  **Answered by the user's choice of the global layout, 2026-11-21.**
  Links stay, so these lines do not contradict the decision any more.
  The sections of `CLAUDE.md` on cards, on the plan and on the minutes
  are aligned with the global rules and with the skill `session-minute`
  within this card.
- **Links already broken.** Whether the frozen documents that already link
  to moved cards are left as they are (the git history keeps them
  readable) or corrected once, as a one-off exception.

  **Answered by the user on 2026-11-21.** Corrected once. Moving to the
  global layout would otherwise break nearly every link in the frozen
  documents: on 2026-11-21, 996 relative links in 205 documents, and
  about 60 references in the code and the top-level files, pointed at
  the folders that move. The exception covers only the rewriting of
  paths that follows from this card, nothing else in a frozen document.
  The same rewriting mends the links already broken, since a card's
  name does not change.
- **Links to finished cards.** A card in `docs/done/` never moves again,
  so a link to it does not break. Whether such links stay allowed, or the
  rule is the same for every card for simplicity.

  **Answered by the user's choice of the global layout, 2026-11-21.** The
  question no longer arises: every card is referred to by its path in
  `docs/board/cards/`, whatever its state.
- **Live documents.** `docs/board/plan.md` and the open cards are kept current,
  so their links can stay; this card does not change them.

  **Answered by the user on 2026-11-21.** This card does change them: the
  rewriting of paths covers every document, live or frozen.
- **What the card covers, and in how many commits.**

  **Answered by the user on 2026-11-21.** Besides the cards, the card
  moves the plan and the minutes into `docs/board/`, aligns the rules
  on minutes with the skill `session-minute`, and updates the
  references in the code. Each in a commit of its own, on a branch of
  its own (`adopt-the-global-board`), so that a start gone wrong can be
  dropped and begun again clean. The rewriting is done by a throwaway
  script kept outside the repository, checked by `card.sh check` and by
  a count of the links that lead nowhere, which has to reach zero for
  every link the card is about (see "Cautions" for the five the count
  still finds).

  **Added by the user on 2026-11-21, once the four steps were done.** An
  `AGENTS.md` at the root, a symbolic link to `CLAUDE.md`, so that tools
  reading their rules from `AGENTS.md` find the same ones. Outside the
  card's problem, but it costs nothing and completes the alignment.
- **The local rule that deletes a card when it is split.** The local
  rules deleted a split card and sent its parts to new cards; the global
  rules keep in the card the part done now and send only the rest to
  new cards. With the global layout, deleting a card breaks again the
  links to it.

  **Answered by the user on 2026-11-21.** The global rule: a card is
  never deleted.
- **Links to cards deleted when they were split.** After the rewriting,
  24 links still led nowhere: they pointed at nine cards deleted when
  they were split, each split into two to five cards, three of them
  twice in a row. No path leads to them, so mending them changes what
  the frozen document says, beyond the exception granted.

  **Answered by the user on 2026-11-21, after looking at all 24.** The
  link becomes plain text followed by the cards that exist today:
  `<old link text> (later split into [<title>](<path>), …)`. Only the
  cards that exist today are listed, also where a split was itself
  split, since this concerns only these old links: from now on no card
  is deleted. The exception is widened to this replacement. A live
  comment in `end2end/tests/accessibility.spec.ts` pointing at one of
  those cards was updated as ordinary upkeep, to the cards that cover
  checking by hand today.

## Recap

The repository now uses the global board: every card at
`docs/board/cards/<name>.md`, its state a symbolic link in
`docs/board/backlog/`, `wip/` or `done/`, the plan at
`docs/board/plan.md`, the minutes in `docs/board/sessions/`. A card's
path never changes, so moving a card breaks no link. All on the branch
`adopt-the-global-board`, brought onto `main` when the card closed.

- `a1af041`: the card taken into work and translated into the card
  format.
- `d145520`: the choice of the global layout recorded, with the
  exception for frozen documents.
- `de5aa55`: the 136 cards moved, with their links; 944 links to cards
  and about 40 other references rewritten, the links inside the cards
  recomputed, the 24 links to deleted cards replaced by the cards of
  today; `CLAUDE.md`, "Cards", defers to the global rules. Broken links
  went from 141 to the five in "Cautions".
- `813342f`: the plan and the 67 minutes moved, 438 links inside them
  recomputed and the references to them rewritten.
- `13574d0`: `CLAUDE.md`, "Session minutes", reduced to a pointer to the
  skill `session-minute` and the three things the skill does not say.
- `c281236`: `AGENTS.md`, a symbolic link to `CLAUDE.md`.
- The closing commit: this card closed and taken out of the plan's Now.

The rewriting was done by two throwaway scripts, outside the repository,
written and run by a sub-agent on Claude Sonnet 5.5; the by-hand edits,
the checks and the decisions' wording by the main session on Claude Opus
5.5.

## Cautions

- **The name of a card's file is now its address.** Renaming a card's
  file breaks every link to it, as a move used to. A card's title can
  change; its file name should not.
- **Frozen documents still mention the old folders in their prose**, for
  example "moves between `docs/backlog/` and `docs/done/`". They were
  left on purpose: they describe the layout of their day, and are not
  links.
- **The count of links that lead nowhere still finds five**, none of
  them a fault: four in `content/` link to pages of the built site
  (`curriculum.html`, `licence.html`, an article, `../about.html`),
  which exist only after the build; one is the template
  `[<title>](<path>)` in this card, inside backticks, which the count
  does not tell apart from a link.
- **This card's title** still says "by name": the answer turned out to
  be a path that never changes, but the file name is kept, by the rule
  above.

## What it leaves

- A rule in `ll-dev-setup` giving every repository an `AGENTS.md`: the
  global rules do not ask for one yet. Not opened, by the user's choice
  on 2026-11-21.

## References

- [Choose the site's public address](choose-site-address.md):
  the move that first showed the problem.
- [Run a screen reader check on demand](run-screen-reader-check-on-demand.md)
  and [Cover the WCAG criteria axe cannot check](cover-wcag-criteria-axe-cannot-check.md):
  where the live comment on checking by hand now points.
