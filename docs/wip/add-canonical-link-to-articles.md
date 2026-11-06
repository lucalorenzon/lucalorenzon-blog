# Add the canonical link to the article pages

- **Opened:** 2026-11-06, by the user, splitting the card
  [add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md),
  opened on 2026-11-02. In Now.
- **Status:** started on 2026-11-06.

## Why

The two articles are on the site and on Medium, and each copy claims to
be the original. Until the site's copy says it is the original, search
engines are likely to treat Medium's copy as the original. The user
decided on 2026-11-03 that the site's copy is the original, when the
card on the two Medium articles was split; the decision was recorded in
the card on search-engine tags.

Among the parts of the card on search-engine tags, only the canonical
link loses something each day it waits. The description and the Open
Graph preview make the pages better when they come, but do not make
them worse while they are missing.

## Answers

### What does the first slice give?

The canonical link on the article pages, chosen by the user. Each
article page declares its own full address as the original, built from
the site's address and the article's address, for example
`https://www.lucalorenzon.com/articles/after-save-event.html`.

- **Only the articles.** The other pages have no copy elsewhere, so the
  link would add nothing to them today. Whether every page gets one is
  left to the card on search-engine tags.
- **The compliance check comes later.** Deciding which search-engine
  rules the build checks, and which pages are left out, is a decision of
  its own, and it belongs to the card on search-engine tags.
- **This slice still has its own tests,** as every change to the code
  has so far, and as the user confirmed: a unit test that the head of an
  article page holds its full address, and a Playwright test that reads
  the link on the two article pages, on the local build and on the live
  site.

### Where does the full address come from?

From the site's address the build already takes
([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md),
point 4: the full address is kept for what needs a full address). The
program keeps only its path part today, so the type that holds it also
keeps the full text.

## Medium's side, by the author

Each story on Medium has a canonical link setting in its advanced
settings. It should point at the site's address of the same article,
once the site is published with this card. Without it two copies still
claim to be the original.
