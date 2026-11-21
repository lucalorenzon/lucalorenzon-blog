# Refer to cards by name in frozen documents

- **Created:** 2026-11-01
- **Status:** In work

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
- **Live documents.** `docs/plan.md` and the open cards are kept current,
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
  a count of the links that lead nowhere, which has to be zero.

## References

- [Choose the site's public address](../done/2026-11-01_choose-site-address.md):
  the move that first showed the problem.
