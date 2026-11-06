# Add the description and the Open Graph tags to every indexed page

- **Opened:** 2026-11-06, by the user, splitting the card on SEO and
  Open Graph tags, opened on 2026-11-02 (the canonical link, its first
  part, is [done](../done/2026-11-06_add-canonical-link-to-every-page.md)).
  The other parts went to
  [check SEO with Lighthouse in the build](../backlog/check-seo-with-lighthouse.md)
  and [add preview images](../backlog/add-preview-images.md). In Now.
- **Status:** started on 2026-11-06.

## Why

An article is worth something only if it can be found and shared. The
user wants each page to carry its own description for search engines,
and the Open Graph tags that make a link shared on LinkedIn or
elsewhere show a proper preview, so that search engines describe the
pages well and a shared link says what it leads to.

## Answers so far

### What does this slice give?

Chosen by the user: every indexed page gets its own description and
every Open Graph tag it needs, each with its own value. The values the
site does not have yet are written in this session. The "page not
found" asks not to be indexed, so it gets neither.

The tags, as read from the Open Graph protocol (ogp.me):

- **Required by the protocol:** `og:title`, `og:type`, `og:url`,
  `og:image`. `og:image` is left out of this slice, see below.
- **For a complete preview:** `og:description`, `og:site_name`,
  `og:locale`.
- **Articles only:** `og:type` is `article`, with
  `article:published_time` from the date the article already has.
- **The search-engine description:** `<meta name="description">`, the
  same text as `og:description`.

### Where do the descriptions come from?

The articles already have a one-line description in the list
`ARTICLES` (`src/article_page.rs`, from the
[card on the home page's list](../done/2026-11-06_list-articles-on-home-page.md)).
The home page, the curriculum, the about page and the licence page have
none: the agent proposes one for each, and the user approves them one
by one, since they speak for the author.

### Why no image yet?

Chosen by the user: the preview images are made in a session of their
own, maybe after the logo is restyled
([card](../backlog/review-the-logo.md)), since any image will likely
carry the logo or the site's name, and the user already has an idea for
the logo. Until then a shared link shows a small preview with text
only; the other tags are useful anyway, and the image is added later
without redoing them. The images went to the card
[add preview images](../backlog/add-preview-images.md).

### Why split from the check?

The user first put the Lighthouse check in this slice, then split it
off when the agent pointed out it holds decisions of its own (how it
runs in the pipeline, thresholds, the pages left out, `robots.txt`),
while this slice is almost decided: each part can be published alone,
and a check that turns out harder does not hold back the descriptions
and the previews. Both cards are in Now, this one first.

### Tests

As for every change to the code: unit tests on the head of each page,
and a Playwright test that reads the description and the Open Graph
tags on every indexed page, on the local build and on the live site.

## Known from the start

- **The head of every page is written in one place,** `PageFrame` in
  `src/page_frame.rs`, which already takes from every page either its
  canonical address or "do not index" (`Indexing`). An optional tag in
  a Leptos view leaves a marker (`<!>`) in the pages without it, so the
  new tags follow the same pattern: a value that every page gives, never
  an optional child.
- **The full addresses** come from the site's address the build takes
  ([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md)),
  through `SiteAddress::join`, as for the canonical link.
- **The Markdown has no fields:** pulldown-cmark 0.13 can recognise a
  metadata block at the top of a file and hand it over as raw text, but
  does not read it; reading YAML or TOML would mean a new dependency.
  The fields stay in the code for now, like the articles' list.

## Questions

- The language of `og:locale`: the pages declare `lang="en"`, the
  protocol wants a language and a territory, such as `en_US` or `en_GB`.
- The preview's title: the page's heading ("After Save Event") or its
  tab title ("After Save Event - Luca Lorenzon"), given that
  `og:site_name` already carries the site's name.
- The type of the pages other than articles: `website` for all, or
  `profile` for the curriculum.
- The text of the four new descriptions.
