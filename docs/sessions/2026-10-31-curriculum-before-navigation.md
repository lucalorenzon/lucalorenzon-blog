# Session 2026-10-31 — Curriculum before navigation

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Claude Haiku 5.5 (git commands).
**Goal:** start the card
[add site navigation](../backlog/add-site-navigation.md). The session
ended instead by reordering the road and opening the card
[put the curriculum on the home page](../backlog/put-curriculum-on-home-page.md).
Reasons are in the two cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Make the menu now, with the curriculum as the home page's content (withdrawn by decision 4) | user | agent | — |
| 2 | Every menu entry disabled in that increment, About me included | user | agent | — |
| 3 | Split the card's move and its new text into two commits, so the history follows the move | user | agent | — |
| 4 | New road: curriculum on the home page, then the domain, then the two articles already on Medium, then the navigation | user | agent | — |
| 5 | Drop the two unpushed commits of decision 1 instead of reverting them | user | agent | — |
| 6 | Keep the user's menu entries in the navigation card, as ideas | user | agent | — |
| 7 | The curriculum is a page of its own, not an article: the licence still comes before the first article | user | agent | — |

## Discussion

1. **Menu now or later.** The user picked the navigation card. The agent
   noted that the site has one page and that the address card asks for
   the domain before a second page. The user asked whether a card for
   articles existed (it did not), then proposed a menu of placeholder
   entries, with the curriculum as the first real content, as a step
   toward the domain. The agent raised five tensions at once (disabled
   placeholders, the address card, the light theme, the menu position,
   the number of decisions); the user replied that the only decision was
   menu now or later, and answered the tensions one by one. →
   decisions 1, 2
2. **Second page or not.** The agent took the curriculum for a second
   page with an internal link. The user clarified that it replaces
   "Sito in costruzione" on the home page, with no links, so the address
   card was untouched. The user then made About me disabled too, since
   it would lead to the same page. → decision 2
3. **History not following the move.** After the first commit the agent
   found that git saw the card as deleted and re-added, because move and
   rewrite were in one commit, and proposed two commits instead. →
   decision 3
4. **The menu is not needed for the domain.** The user found the card's
   paragraph on the rejected option unclear. The agent explained it, and
   admitted that half of it was its own inference. The user then saw
   that the menu was not what enabled the domain, and reordered the road.
   The agent proposed dropping the two local commits rather than adding
   a revert, so the history does not carry a decision and its reversal
   minutes apart, and keeping the menu entries as ideas. → decisions 4,
   5, 6
5. **Curriculum and licence.** The user had called the curriculum "one
   article"; the agent asked whether the licence card, due before the
   first article, applied. The user answered that the curriculum is a
   page of its own. → decision 7

## Outcome

- `aae6694`: the curriculum card created and put in Now, the address in
  Next, the navigation card back in Later with the user's entries and
  when to pick it up.
- Dropped before any push: `a0e82c3` and `5ae76c1`, which had replaced
  `bc560aa` (decisions 1 and 3).
- Nothing pushed; this minute is the session's last commit.

## Open points

- The two articles already on Medium have no card yet; the plan still
  lists articles as "not planned yet". To be opened when the road gets
  there.

## Next steps

- Put the curriculum on the home page, starting with how its Markdown
  becomes part of the page: card
  [put the curriculum on the home page](../backlog/put-curriculum-on-home-page.md).
- Then choose the site's public address: card
  [choose the site's public address](../backlog/choose-site-address.md).
