# Add SEO and Open Graph tags to every page

- **Opened:** 2026-11-02, by the user; in Next.
- **Status:** noticed, not started.

## Why

Articles are about to arrive
([card](../wip/add-the-two-medium-articles.md)), and an article is worth
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

- **Today the head is almost empty:** `src/home_page.rs` writes only
  the character set, the viewport and the title. There is no
  description, no canonical link, no Open Graph tag, no robots
  directive.
- **The Markdown has no fields yet:** `content/curriculum.md` starts
  directly with the text. pulldown-cmark 0.13 can recognise a metadata
  block at the top of a file (YAML-style `---`, or `+++`) and hand it
  over as raw text, but does not read it: reading YAML or TOML would
  mean a new dependency. The unit test that forbids HTML in the content
  stays as it is: the fields are data, not HTML.
- **Open Graph needs absolute addresses:** `og:url` and `og:image` take
  the full address, which today is `https://www.lucalorenzon.com/`
  ([card](../done/2026-11-01_choose-site-address.md)). The build will
  take it as a setting, introduced by the "page not found" card
  ([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md)):
  the same value serves these tags. The two Medium articles each have
  an image; the curriculum has none.
- **The canonical link is already a question** in the card on the
  Medium articles: which copy, the site's or Medium's, search engines
  should treat as the original. The tag this card adds is where that
  answer ends up.
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
- How thin the first slice can be: for example the tags on one page and
  the check that fails without them, before the block-list.
