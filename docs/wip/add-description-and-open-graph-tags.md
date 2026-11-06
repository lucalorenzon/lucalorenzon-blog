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

## Answers

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

The articles keep the one-line description they already have in the
list `ARTICLES` (`src/article_page.rs`, from the
[card on the home page's list](../done/2026-11-06_list-articles-on-home-page.md)),
taken from Medium. The other four were proposed by the agent and
reworded with the user, who approved each one; all stay within 160
characters, the length search engines show in full:

| Page | Description |
|---|---|
| Home page | Articles by Luca Lorenzon, product engineer: what I have learned, my thoughts on topics of public debate, and stories of paths I have taken. |
| Curriculum | Curriculum of Luca Lorenzon: collaboration, architecture and hands-on implementation in e-commerce and enterprise integration, in international contexts. |
| About | Who writes this site and why: Luca Lorenzon, a product engineer sharing what I have learned. The views here are my own. |
| Licence | How you may reuse this site: the articles under CC BY 4.0, the code samples under MIT-0, the curriculum under CC BY-ND 4.0. |

- **Home page:** "at work" was dropped, at the user's request: it
  narrowed the site for no reason.
- **Curriculum:** the user's wording, with "other solutions" replaced
  by "enterprise integration", which the curriculum's summary already
  names; "product engineer" was dropped to stay within 160 characters.
  The international context is the author's whole career, though the
  published curriculum, cleaned of employer data, says so explicitly
  only once.
- **About:** the part on reuse was dropped as redundant with the
  licence page, which the about page links; at 77 characters what was
  left was too short (search engines tend to replace a short
  description with text of the page) and promised a "why" it did not
  say, so the author and the views were added back.

### The other values

- **`og:title`:** the page's heading, without the site's name ("After
  Save Event", "Home Page"), chosen by the user; the home page keeps
  "Home Page", which the user finds universal.
- **`og:site_name`:** "Luca Lorenzon", as in the tab titles; "Luca
  Lorenzon Blog" was considered and rejected, since the site also holds
  the curriculum and the about page, and the name would differ from the
  tab titles. The logo's "Lvk@73r" was not chosen: the logo is still to
  be reviewed.
- **`og:locale`:** `en_GB`, chosen by the user; the site's spelling is
  British ("licence", "colour"). The pages keep `lang="en"`, of which
  `en_GB` is a case.
- **`og:type`:** `article` for the articles, `profile` for the
  curriculum and the about page, which both speak of the author (with
  `profile:first_name` "Luca" and `profile:last_name` "Lorenzon"),
  `website` for the home page and the licence page. Chosen by the user;
  the protocol lets several pages of the same person be profiles, each
  told apart by its address.
- **`og:url`:** the canonical address of the page.

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
