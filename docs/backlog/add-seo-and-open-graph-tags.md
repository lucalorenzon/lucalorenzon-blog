# Add SEO and Open Graph tags to every page

- **Opened:** 2026-11-02, by the user. Split by the user on
  2026-11-06: the canonical link on the article pages went to a card of
  its own, [add the canonical link to the article pages](../wip/add-canonical-link-to-articles.md).
  In Next.
- **Status:** noticed, not started.

## Why

Articles are about to arrive
([card](../done/2026-11-04_add-article-pages.md)), and an article is worth
something only if it can be found and shared. The user wants each
article to carry its own search-engine fields and Open Graph fields,
turned into the matching tags in the page's head, so that search
engines describe it well and a link shared on LinkedIn or elsewhere
shows a proper preview: title, description, image.

The user also wants the build to check this automatically: that every
page meets the chosen search-engine rules and has its Open Graph tags,
so a page published without them does not go unnoticed. Some pages must
be able to stay out of it, through a block-list of pages the tags and
the check do not apply to.

## Known from the start

- **Today the head is almost empty:** `src/page_frame.rs`, the frame
  every page shares, writes only the character set, the viewport and
  the title; the card split off on 2026-11-06 adds the canonical link
  to the article pages. There is no description, no Open Graph tag, no
  robots directive.
- **The Markdown has no fields yet:** `content/curriculum.md` starts
  directly with the text. pulldown-cmark 0.13 can recognise a metadata
  block at the top of a file (YAML-style `---`, or `+++`) and hand it
  over as raw text, but does not read it: reading YAML or TOML would
  mean a new dependency. The unit test that forbids HTML in the content
  stays as it is: the fields are data, not HTML.
- **The articles' fields are written in the code meanwhile:** since
  2026-11-06 the list `ARTICLES` in `src/article_page.rs` holds each
  article's title, tab title, address, one-line description and date,
  for the article pages and the home page's list
  ([card](../done/2026-11-06_list-articles-on-home-page.md)). Many of
  the fields this card needs are already there; whether they move to
  the Markdown is still open.
- **Open Graph needs absolute addresses:** `og:url` and `og:image` take
  the full address, which today is `https://www.lucalorenzon.com/`
  ([card](../done/2026-11-01_choose-site-address.md)). The build will
  take it as a setting, introduced by the "page not found" card
  ([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md)):
  the same value serves these tags. The two Medium articles had an image
  each, but both come from the vendor's documentation and become tables
  on the site (decided on 2026-11-03), so no page has an image of its
  own for the preview yet.
- **The canonical link belonged to this card,** decided by the user on
  2026-11-03, when the card on the Medium articles was split: the
  site's copy of each article is the original, so its page declares
  itself canonical. On 2026-11-06 the user moved it to a card of its
  own, [add the canonical link to the article pages](../wip/add-canonical-link-to-articles.md),
  since it is the only part that loses something each day it waits;
  that card also carries Medium's side, by the author. Whether the
  other pages get a canonical link too stays here.
- **A first candidate for the block-list:** the site's own "page not
  found" ([card](../done/2026-11-02_serve-own-not-found-page.md)), which should not be
  indexed nor previewed.
- **Lighthouse has an SEO category,** with basic checks (title,
  description, status code, links, `robots`). The card
  [check performance in the build](check-performance-in-the-build.md)
  may bring Lighthouse into the build; its SEO score does not check
  Open Graph.
- **The build already opens every page in a browser** (Playwright, on
  the local build and on the live site after publication), where the
  tags of each page can be read.

## Questions

- Which fields each article declares, and which the site fills in by
  itself (for example the site name, the language, the address).
- Where the fields live: a metadata block at the top of each Markdown
  file, or a separate file; and what happens when one is missing.
- Which pages get the tags: the articles only, or every page, the home
  page included.
- What "SEO compliance" means for this site: which rules are checked,
  and with which tool.
- What the block-list is: a list kept in one place, or a field in each
  page; and whether a page on it drops the check only, or the tags too.
- How thin the next slice can be: for example the tags on one page and
  the check that fails without them, before the block-list.
