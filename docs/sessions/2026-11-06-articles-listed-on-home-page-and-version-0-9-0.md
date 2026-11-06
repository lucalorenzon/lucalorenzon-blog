# Session 2026-11-06 — The articles listed on the home page, version 0.9.0

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Sonnet 5.5 (the curriculum page; the home page as the list
of articles, with its tests and the fix of its bullets).
**Goal:** bring the changelog up to date, move the navigation to Now,
and start it. The navigation card was split; its first part,
[list the articles on the home page](../done/2026-11-06_list-articles-on-home-page.md),
was built, validated and published with version 0.9.0. Reasons and
rejected options are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The changelog's Unreleased section lists what `main` brought after 0.8.0, in the agent's wording | user | agent | — |
| 2 | The navigation in Now; the WCAG and logo cards to Next; Later's line on further articles corrected | user | agent | — |
| 3 | The first published navigation lets a reader find and read the articles; the rest later | user | agent | — |
| 4 | The list on the home page, the curriculum on a page of its own | user | agent | — |
| 5 | No visible tool in the header for now: the logo, the list and the footer's link are enough | user | agent | — |
| 6 | The navigation card split into "list the articles on the home page", in Now, and "add a menu for the site", in Next | user | agent | — |
| 7 | The curriculum at `curriculum.html`, at the root; languages later | user | agent | — |
| 8 | No visible way from the home page to the curriculum: the old address was given to no one, and the site is not promoted before observability and search-engine tags | user | agent | — |
| 9 | Each entry with title, one-line description and date, written in the code | user | agent | — |
| 10 | The two articles dated by Medium: 25 and 30 April 2023 | user | agent | — |
| 11 | An article first published on this site takes this site's date | agent | — | user |
| 12 | Newest first | user | agent | — |
| 13 | The descriptions written by the user, with three touches proposed by the agent | user | agent | — |
| 14 | The footer and the about page's link follow the rule already agreed | agent | — | user |
| 15 | The home page's title is "Home Page" | user | agent | — |
| 16 | Two slices: the curriculum page, then the list | user | agent | Sonnet |
| 17 | The list's markup: an HTML list, titles as headings, order from the dates in the code, the date as `<time>` | agent | Sonnet | user |
| 18 | Variant A: title, description, then the date written out | user | agent | — |
| 19 | The card closed after the user's validation in the browser, and pushed | user | — | agent |
| 20 | Version 0.9.0 published | user | agent | — |
| 21 | The changelog's rule on version 0 no longer says the articles are to come | user | agent | — |

## Discussion

1. **The changelog.** The user asked for it before moving the
   navigation. The agent drafted the Unreleased section from the cards
   closed after 0.8.0; the user approved it, and asked to correct the
   line in Later and to take the WCAG and logo cards out of Now. →
   decisions 1, 2
2. **What the navigation is for.** The agent laid out what the entries
   listed on 2026-10-31 would each need, none of which the site has,
   and asked what the first published navigation must let a reader
   do. The user answered: find and read the articles. → decision 3
3. **Where the list lives, and how to reach it.** The agent pointed out
   that a list on the home page moves the curriculum, and that with the
   logo already leading home, a header tool might not be needed yet.
   The user chose the home page and no tool. → decisions 4, 5
4. **The split.** The agent read the change as narrowing the card; the
   user named it a split into two cards, one in Now and one in Next. →
   decision 6
5. **The curriculum's address.** The agent set out what lasts in an
   address and the card on more languages; the user chose
   `curriculum.html`. Asked whether the old address had been given out,
   the user said no, and that the site will not be promoted before it
   has observability and search-engine tags. → decisions 7, 8
6. **The entries.** The agent pointed out that only the title exists,
   and that richer fields were left to the `residuality` run and the
   search-engine card. The user first chose the title only; offered a
   description written in the code, they took it and added the date,
   from Medium, with the list newest first. → decisions 9 to 13
7. **The descriptions.** The user wrote them; the agent pointed out
   how the articles write "SAP hybris" and two English phrasings, and
   the user accepted the touches. → decision 13
8. **The home page's title.** The agent raised that "Curriculum Vitae"
   no longer fits and that a title naming the list would change with
   the home page; the user chose "Home Page". → decision 15
9. **The build.** Sonnet built the two slices; the agent's screenshots
   showed the text column's bullets still on the list, which the tests
   had missed. Sonnet fixed it and added a browser check. The agent
   showed three variants side by side; the user chose A. Sonnet tried
   `git stash` against its brief; the permission check blocked it and
   nothing changed. → decisions 16 to 18
10. **Closing and publishing.** The user validated the site served
    locally, had the card closed and pushed, and confirmed the tag.
    The agent proposed correcting the changelog's rule on version 0
    before this minute. → decisions 19 to 21

## Outcome

- The changelog: `2178cd2` Unreleased filled; `eeba19a` version 0.9.0;
  `20209d9` the rule on version 0.
- The plan and the cards: `781dc50` the navigation in Now; `9390245`
  the navigation card started; `762caa8` the card split; `12e8dc2`,
  `0b0945f`, `c6ccf7c` the list card started and its answers;
  `1031b33` the card closed; `d5515a4` the plan after the version.
- On the site: `8f8e077` the curriculum on its own page; `09ad2e5` the
  home page lists the articles, newest first. 150 unit tests and 820
  browser tests pass (2 skipped, as before).
- Version 0.9.0 published from the tag `v0.9.0` on `eeba19a`: the
  snapshot reused, the Release created, the browser tests passed on the
  public site.

## Open points

- The screen reader check's reference transcript still describes an
  earlier home page; it was run by hand before, and is the subject of
  the card on updating it.

## Next steps

- The menu, with search, categories, subscribing and settings, in Next:
  [add a menu for the site](../backlog/add-site-menu.md).
- The screen reader reference, in Next:
  [update the screen reader check for an English site](../backlog/update-screen-reader-check-for-english-site.md).
- The next card in Now, chosen by the user from Next:
  [plan](../plan.md).
