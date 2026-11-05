# Session 2026-11-05 — Wide tables: the record viewer

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Sonnet 5.5 (prototype, measurements, code and tests of the
first slice).
**Goal:** decide what a table too wide for a phone does, in the card
[let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md),
section "Slice 2: wide tables". Reasons and rejected options are in
the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Tables carry no annotation: the build measures them | user | agent | — |
| 2 | On a phone, a wide table becomes a record viewer: labels still, one record a gesture, vertically | user | agent | — |
| 3 | No sideways scrolling in tables, ever | user | — | agent |
| 4 | Each field as tall as its tallest value, so the box never changes height | user | agent | — |
| 5 | A script is acceptable, but HTML and CSS only is better: a spike first | user | agent | — |
| 6 | The prototype published as a private page on claude.ai, to try the swipe on a phone | user | agent | — |
| 7 | The record number in one still, opaque row, switched by the scroll position; the number inside each record where that is unsupported | user | agent, Sonnet | — |
| 8 | What a Leptos island weighs is measured with the first island, in a card of its own | user | agent | — |
| 9 | A table cell with no space never wraps; a per-column annotation stays an idea | user | agent | — |
| 10 | A table becomes a record viewer below a readable width, not only when it overflows | user | agent | — |
| 11 | M is 18 characters: the hybris table stays a table from 768 pixels up | user | agent, Sonnet | — |
| 12 | The style for single-value cells is named `one-line` | Sonnet | — | agent, user |
| 13 | The browser test of that rule checks every page, not one article | user | agent | — |
| 14 | A card to make the older browser tests check behaviour, in Next | user | agent | — |

## Discussion

1. **Pivot and gesture.** The user brought a solution seen on other
   sites, a table pivoted when it overflows, and asked whether the
   annotation could be dropped since the build can measure. The agent
   named the two forms "pivot" covers; the user described a third, one
   record at a time scrolled vertically. The agent raised two tensions,
   fields of different heights across records and a gesture shared with
   the page; the user answered with the whole record moving as a block
   and the first column left to the page. → decisions 1, 2, 3
2. **Heights and script.** The agent reported that keeping still labels
   aligned with per-record heights needs a script; the user accepted
   one (Leptos was chosen for it, islands are wished for) but chose the
   tallest-value heights and asked to try without first.
   → decisions 4, 5
3. **Spike A.** A Sonnet sub-agent built the viewer with HTML and CSS
   only; the agent reviewed it and, since the user could reach no local
   server, published it as a private page; the user tried it on a phone
   and found it works. Chromium's mouse wheel stays an open limit.
   → decision 6
4. **The number.** The user found the record number showing through;
   made opaque and sticky, the next number was seen arriving from
   below. The agent proposed one row outside the box, switched by a
   scroll-driven animation, warning about Firefox; Sonnet built it, the
   user called it what they wanted. → decision 7
5. **Spike B.** With no script needed, the agent pointed out the island
   measurement no longer served this card; the user postponed it to the
   first island. → decision 8
6. **The bits table.** Hearing about the threshold, the user recalled
   that "63–44" breaks in the bits table and proposed an optional
   minimum per column. The agent checked it on the page, noted the clash
   with decision 1, and showed the build can tell a single value
   itself. The user agreed. → decision 9
7. **The threshold.** Asked whether the build can know a table's width
   in pixels, the agent explained what it can measure (word and line
   widths from the font) and what it cannot (how the browser shares the
   width among columns); asked for suggestions, it proposed overflow
   only or a minimum readable width M, preferring the second. Sonnet's
   side-by-side images showed the text column's maximum width leaves
   only two cases; the user judged the table at 768 "slightly at the
   limit, but acceptable" and preferred the viewer at 659.
   → decisions 10, 11
8. **Tests wired to an article.** Reviewing the first slice, the user
   found its browser test tied to the bits table's values, and proposed
   a card for the tests. The agent showed the unit tests were already
   general and the older browser tests share the weakness; the test was
   rewritten before its commit. → decisions 12, 13, 14

## Outcome

- The card's section "Slice 2: wide tables" holds the decisions above
  and the spike's results: `d5dc148` spike A, `1888d2b` the record
  number, `afaa34b` single-value cells, `4f4810f` the threshold.
- New cards in the backlog: `0c57618`
  [measure what a Leptos island weighs](../backlog/measure-weight-of-a-leptos-island.md),
  in Later; `e40068b`
  [make the browser tests check behaviour, not articles](../backlog/make-browser-tests-check-behaviour.md),
  in Next.
- On the site: `18c22a0` a table cell with no space stays on one line.
  101 unit tests and 500 browser tests pass (2 skipped, as before).
- Plus this minute.

## Open points

- The commits of this session and of the previous one are not pushed.
- A header that stays in view while a long table scrolls with the page,
  an idea of the user's: the agent proposed a card of its own; not
  decided.
- What a wide table does on a wide screen, where the space beside the
  text stays empty.
- Not verified: Chromium's mouse wheel on the viewer, Firefox, and a
  contrast finding axe reported once with the viewer on a switch point.
- At 320 pixels with doubled text, something on the hybris PK page other
  than the table is a few pixels too wide.
- The prototype stays online as a private page on claude.ai, visible
  only to the user, for reference in the next slice.

## Next steps

- Slice 2: the record viewer and its threshold in the build, with the
  readability rule revised for a box that cuts text vertically by
  design. Card
  [let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md).
- Slice 3: the hybris PK article back in the checks, which closes that
  card.
