# Add the canonical link to every page

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

The canonical link, chosen by the user. Each page declares its own full
address as the original, built from the site's address and the page's
address, for example
`https://www.lucalorenzon.com/articles/after-save-event.html`; the home
page declares the site's address itself, `https://www.lucalorenzon.com/`.

- **Every page, not only the articles.** The user first chose the
  articles only, since the other pages have no copy elsewhere. The first
  version of the code then needed two almost identical heads in the
  page frame: an optional tag in a Leptos view leaves a marker (`<!>`)
  in the head of the pages without it. The user asked whether a link on
  every page would remove the optional case, and it does. It is also
  the common advice for search engines: a page naming itself the
  original guards against accidental duplicates of its own address,
  such as `/` and `/index.html`.
- **The "page not found" says "do not index me" instead.** It has no
  address of its own: GitHub shows it in place of any address that does
  not exist, with status 404. Asked directly at `/404.html`, though, it
  answers with status 200 (checked on the live site on 2026-11-06), so a
  search engine finding it could index it. Its head carries
  `<meta name="robots" content="noindex">` and no canonical link, since
  the two signals contradict each other. Chosen by the user in this
  slice rather than left to the block-list of the card on search-engine
  tags: every page then declares exactly one thing about its indexing,
  and the page frame keeps one head.
- **Rejected: the canonical link on the articles only, with two heads**
  (the first version of the code): no marker, but every later optional
  tag would multiply the heads. **Rejected: one head with an optional
  link:** shorter code, but a useless marker in the head of every other
  page.
- **The compliance check comes later.** Deciding which search-engine
  rules the build checks, and which pages are left out, is a decision of
  its own, and it belongs to the card on search-engine tags.
- **This slice still has its own tests,** as every change to the code
  has so far, and as the user confirmed: unit tests that the head of
  each page holds its full address, or the "do not index" tag on the
  "page not found", and a Playwright test that reads them on the local
  build and on the live site.

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
