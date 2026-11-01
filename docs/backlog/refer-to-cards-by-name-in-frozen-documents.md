# Refer to cards by name in frozen documents

- **Opened:** 2026-11-01
- **Status:** open, proposal from the user.

## Why

Session minutes and finished cards are frozen: once committed they are not
edited. Yet they link to other cards by path, and a card's path changes
every time it moves between `docs/backlog/`, `docs/wip/` and `docs/done/`.
Each move therefore breaks links that, by rule, nobody may fix.

It showed up on 2026-11-01, moving the card on the site's address
(`choose-site-address`) to `docs/wip/`: three live documents were updated,
while seven frozen ones (three finished cards, four minutes) kept pointing
at `backlog/choose-site-address.md`, which no longer exists. Keeping such
links current by hand would mean editing frozen records on every move,
which does not scale.

## Proposal

From the user: in minutes and in finished cards, refer to other cards by
their name, not by a link.

## Questions to answer

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
