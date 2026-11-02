# Session 2026-11-02 — The site's own "page not found", and how pages write addresses

**Participants:** the user (author); the agent (Claude Opus 5.5), with a
sub-agent on Claude Sonnet 5.5 (code).
**Goal:** open the card
[add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md),
then close the card
[serve the site's own "page not found"](../done/2026-11-02_serve-own-not-found-page.md);
the spike
[choose how pages write the addresses of their files](../done/2026-11-02_choose-how-pages-write-addresses.md)
was opened and closed on the way. Technical details and reasons are in
the cards and in
[ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | New card on SEO and Open Graph tags, with an automatic check and a block-list, in Next | user | — | agent |
| 2 | The "page not found" gives the minimum to find one's way: tab title, the home page's header and footer, heading "Page not found", a link home; "only 404" dropped | user | agent | — |
| 3 | A base path in every address, one rule for every page (first version of ADR-002), later revised by decision 6 | user | agent | — |
| 4 | Addresses inside the stylesheet stay relative | agent | — | user |
| 5 | Park the "page not found" card and open a spike on how pages write addresses | user | agent | — |
| 6 | Relative paths in every page; only the "page not found" uses a base path | user | agent | — |
| 7 | One build value, the site's full address; the "page not found" uses its path part | user | agent | — |
| 8 | ADR-002 rewritten in place rather than replaced by ADR-003 | agent | user | — |
| 9 | Every tab title puts the page's name first, then "Luca Lorenzon" | user | agent | — |
| 10 | Locally the browser test opens `404.html` directly; the behaviour at a missing address is checked on the live site | user | agent | — |
| 11 | The "page not found" goes on the SEO card's block-list | user | agent | — |
| 12 | The shared page frame leaves `<main>` to each page | sub-agent (Sonnet) | — | agent, user |
| 13 | Version 0.6.0, MINOR by the changelog's rules; published | user | agent | — |

## Discussion

1. **SEO and Open Graph.** The user asked for a card: fields in each
   article turned into tags, a check in the build, a block-list. The
   agent wrote what the head holds today and the questions. → decision 1
2. **What the page shows.** The agent laid out the constraints at once
   (accessibility rules, paths at depth, the local server that cannot
   serve `404.html`, search engines) and asked what a visitor should
   find. The user dropped the "only 404" proposal, explaining it came
   from an unrelated memory, and listed what the page carries. → decisions 2, 10, 11
3. **Paths and context path.** The user asked for an address from the
   root and recalled a configurable context path for portability behind
   a gateway. The agent found that discussion on the previous attempt
   (`old_main`), and that the relative-paths rule had been taken by the
   agent on 2026-10-28 without a record of its own; the user asked where
   it was tracked. The user chose a base path at build, for every page,
   and an ADR was written. → decisions 3, 4
4. **The ADR questioned.** Reading the ADR, the user challenged a risk
   the agent had worded loosely; the agent corrected it. The discussion
   moved to how a gateway rewrites addresses and to the `<base>`
   element, raised by the user; the agent listed what that element
   breaks. The user parked the card and opened a spike so the question
   would not be settled inside another card. → decision 5
5. **The spike.** The user asked for every option with its benefits and
   drawbacks and what static sites commonly do. The agent's comparison
   showed that relative paths give more portability than a base path in
   every address; the user chose relative paths with the base path on
   the "page not found" only, taken from the site's full address that
   the SEO card will need. The user asked for ADR-002 to be updated or
   deleted; the agent rewrote it, since it had never been applied. → decisions 6, 7, 8
6. **Tab title.** The agent pointed out that the two pages put the
   site's name in different places; the user chose one order. The agent
   then asked about the footer, not yet discussed: kept. → decision 9
7. **Implementation.** The Sonnet sub-agent wrote three commits; the
   agent checked them and ran the tests again. → decision 12
8. **Publication.** The user saw the page on a local server that
   behaves like GitHub Pages, then asked for push and publication. The
   first attempt to push the version tag was blocked by the permission
   check as a production deploy; the agent stopped and reported, and
   the user asked to try again. → decision 13

## Outcome

- New card in Next (`0258b0f`).
- Spike opened and closed: `ab53989`, `b6773d2`, `5a5ad14`; ADR-002
  first written in `587dc6e`, rewritten in `5a5ad14`.
- "Page not found" card: moves and answers `0149106`, `f1584aa`,
  `ca98aa7`, `8cf19f8`; code `f29338d` (tab title), `f58071f` (shared
  page frame), `1da48fc` (site address and `404.html`); documentation
  `2033b00`; closed in `2a8e688`.
- Version 0.6.0: changelog `be7dce0`; build run
  [41413698793](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41413698793)
  and deploy run
  [41414797014](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41414797014)
  passed, live check included:
  `https://www.lucalorenzon.com/a/b/c/missing-page` answers 404 with
  the site's page and its look.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Put the "page not found" on the block-list, and use the site's
  address setting for the full addresses: card
  [add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md).
