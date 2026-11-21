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

- **What "name" is.** The file name without folder and date prefix
  (`choose-site-address`) is the same in all three folders and finds the
  card with a single search; the title (`Choose the site's public
  address`) reads better but can change.
- **Rules that contradict it today.** `CLAUDE.md`, "Session minutes", asks
  for "a link to the card" in the goal and in each next step, and says
  "What a card already says is linked, not repeated". Those lines, and the
  reference minute they point at, change with this decision.
- **Links already broken.** Whether the frozen documents that already link
  to moved cards are left as they are (the git history keeps them
  readable) or corrected once, as a one-off exception.
- **Links to finished cards.** A card in `docs/done/` never moves again,
  so a link to it does not break. Whether such links stay allowed, or the
  rule is the same for every card for simplicity.
- **Live documents.** `docs/plan.md` and the open cards are kept current,
  so their links can stay; this card does not change them.

## References

- [Choose the site's public address](../done/2026-11-01_choose-site-address.md):
  the move that first showed the problem.
