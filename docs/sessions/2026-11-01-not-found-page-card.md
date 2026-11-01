# Session 2026-11-01 — Card on the site's own "page not found"

**Participants:** the user (author); the agent (Claude Opus 5.5).
**Goal:** a follow-up to the session
[the site's public address](2026-11-01-site-address.md), after its minute
was committed: open the card
[serve the site's own "page not found"](../backlog/serve-own-not-found-page.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Open a card for a "page not found" of the site's own, and put it in Now | user | agent | — |
| 2 | The card's questions are taken up in another session | user | — | agent |

## Discussion

1. **GitHub's page for a missing address.** The user noticed that a
   missing page shows GitHub's error and asked for one that gives no
   information, just "404". The agent confirmed that a `404.html` at the
   site's root does it, and pointed out two limits: GitHub's `server`
   header stays, and the domain-level page seen earlier is not covered.
   Writing the card, it found that relative paths to the stylesheet break
   on a page served at any depth. → decision 1
2. **What the page shows.** The agent asked whether "no information" also
   excludes the site's name and a link to the home page; the user chose
   to answer in another session. → decision 2

## Outcome

- `958eea5`: the card, in Now.
- Not pushed: `958eea5`, this minute, and the five commits of the session
  on the site's public address.

## Open points

- The stale `404` for `http://www.lucalorenzon.com` in browsers had not
  expired at 17:12, when the check was stopped.

## Next steps

- Decide what the "page not found" shows, and build it: card
  [serve the site's own "page not found"](../backlog/serve-own-not-found-page.md).
