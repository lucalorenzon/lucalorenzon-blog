# Give each article a page of its own

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Now.
- **Status:** done on 2026-11-04. The pages are built, not linked from
  any other page and not published: no version is published until the
  [navigation](../wip/list-articles-on-home-page.md) exists.

## Why

The texts of the two articles from Medium were prepared in their own
cards ([After Save Event](2026-11-03_add-after-save-event-markdown.md),
[hybris PK values](2026-11-03_add-generate-pk-article-markdown.md)).
This card gave them a page each, the simplest way: their address, how
they are reached, the page itself.

## The answers

### The address

**Chosen, by the user: a sub-folder `articles/`,**
`articles/after-save-event.html` and
`articles/how-to-generate-hybris-pk-values-without-hybris.html`, named
like the Markdown files under `content/articles/`. Once published, an
address ends up in shared links and search engines, and changing it
means redirects, which GitHub Pages does not make easy. The pages
write their addresses relative to themselves, with the prefix `../`
([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md)),
which the shared frame already supported.

### How a reader gets there

**Chosen, by the user: by the address only,** with no link from the
home, about or licence pages. The link from "After Save Event" to the
other article now points to its address on the site, as planned when
it was written. Search engines are unlikely to find the pages this
way, which is consistent with leaving them to the
[card on search-engine tags](../backlog/add-seo-and-open-graph-tags.md).

### When they are published

**Chosen, by the user: not before the navigation,** and not before
the article pages are accessible and in as good a state as the
curriculum: the automatic checks pass and the user validates the
pages. Publishing happens only on a version tag, so the pages can be
on `main` without going live; but until the navigation is done no
version is published, other changes included. The order in the plan
became: these pages, the
[card on Markdown annotations for wide content](../wip/annotate-markdown-for-wide-content.md),
the [look card](../wip/design-look-of-articles.md), the
navigation, then a version.

### Where the title lives

The shared frame gives every page its tab title and its only `h1`,
and a test checks there is one; the Markdown of both articles started
with its own title. **Chosen, by the user: the title in the code,
removed from the Markdown,** as on the other pages, whose Markdown has
no title; it keeps the pages the simplest. Rejected: the title in the
Markdown, read by the page, which keeps the text complete on its own
but anticipates the model of an "article" the user left to the
[run of `residuality`](../backlog/run-full-residuality-round.md). The
closing sentence on the licence stays in the Markdown, so the text
still carries its attribution when reused alone.

### Wide code and tables on a phone

The readability test forbids a page that scrolls sideways and also a
box that scrolls inside itself, counted as cut-off text: only wrapping
would pass. On Medium both code and tables scroll sideways on a phone.
The user saw in it a need that was bound to come, telling the
presentation layer how to show a piece of text without putting HTML
in the Markdown, and opened the
[card on Markdown annotations for wide content](../wip/annotate-markdown-for-wide-content.md),
which takes the question from the look card. **Chosen, by the user:
the article pages stay out of the readability and content-layout
checks until that card is done;** they are in the local accessibility
checks.

### The user's validation

On reading the pages the user found the text too small, the
sub-titles heavier than the section titles, the line above the
sections out of place, and the code in need of a visible box; and set
a direction for the headings of articles, different from the
curriculum's. All of it is the look of the articles: **chosen, by the
user: close this card with the pages as they are and move the
observations to the [look card](../wip/design-look-of-articles.md).**

## Results

- `src/article_page.rs` builds both pages from the shared frame; the
  build writes them into `articles/`. Tables are switched on in the
  Markdown conversion (`src/markdown.rs`).
- 78 unit tests pass; the browser tests pass (154, one skipped as
  before), axe included on both new pages at three screen widths.
- Not added to the accessibility-tree check: its expected trees are
  written by hand, which for two long articles is better weighed with
  the look card.
