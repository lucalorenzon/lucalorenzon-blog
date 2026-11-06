# Serve the site's own "page not found"

- **Opened:** 2026-11-01
- **Status:** done on 2026-11-02, published with version 0.6.0. Parked
  the same day for the card on how pages write the addresses of their
  files, and resumed once it closed.

## Why

An address that does not exist on the site, for example
`https://www.lucalorenzon.com/missing-page`, got GitHub's own page,
titled "Page not found · GitHub Pages", with GitHub's text and branding.
The user wanted the site's own page instead.

GitHub Pages serves a file named `404.html` at the root of the published
site, with status 404, for every address that does not exist; this works
with a deployment from GitHub Actions too. The build wrote only
`index.html`, so there was none.

Limits known from the start, still true: GitHub Pages adds the header
`server: GitHub.com`, which cannot be removed; and the page "There isn't
a GitHub Pages site here", which GitHub serves for a domain it does not
recognise, is outside the site and not covered.

## Answers

### What does the page show?

**Chosen: the minimum for the visitor to find their way,** by the user:

- **Tab title:** "Page not found - Luca Lorenzon".
- **Header and footer:** the same as the home page, with "Page not
  found" as the main heading; only the content differs.
- **Content:** a link to the home page, "Go to the home page".

Rejected: a page saying only "404", the user's first proposal. It came
from a memory of an application answering an AJAX call, where a short
answer avoids handing information to attackers; it does not apply to a
static page on GitHub Pages, and it would leave a visitor who followed
a wrong link with no way to tell which site they are on.

### In which order do the tab title's parts go?

**Chosen: the page's name first, then the site's name, on every page,**
by the user. Where only the start of a title shows (many open tabs,
bookmarks), the pages are told apart at a glance. The home page changed
from "Luca Lorenzon - Curriculum Vitae" to "Curriculum Vitae - Luca
Lorenzon". The earlier order had no reason of its own: the rule of
2026-10-28 only asked that the author's name be there
([card](2026-10-28_add-header-footer-and-favicon.md)).

Rejected: different orders on the two pages, which nothing justified.

### How does the page reach its stylesheet and icon?

The page is served at any depth (`/a/b/c` too), where the relative
paths the home page uses break. The question turned out to concern every
page, so the user parked this card and opened a spike for it:
[choose how pages write the addresses of their files](2026-11-02_choose-how-pages-write-addresses.md),
which compares the options and records the rejected ones.

**Chosen there:** every page keeps paths relative to itself; the build
takes the site's full address (`SITE_ADDRESS`, by default
`https://www.lucalorenzon.com/`), and only this page writes its
addresses from its path part (`/` today)
([ADR-002](../adr/ADR-002-relative-paths-and-site-address-at-build.md)).
This card introduced that setting.

### How is it checked?

**Chosen: the page's content locally, the real behaviour on the live
site.** The local browser test serves the site with Python's
`http.server`, which answers a missing address with its own page, not
with `404.html`: locally the test opens `404.html` directly. A missing
deep address answering 404 with the page and its look is checked on the
live site after each publication.

Rejected: changing the local test server to behave like GitHub Pages;
the live check already covers the behaviour, and the user accepted the
direct check locally.

### What about search engines?

The page goes on the block-list of the card
[add SEO and Open Graph tags to every page](../wip/add-seo-and-open-graph-tags.md):
GitHub answers with status 404, so it is not indexed anyway.

## Results

- **Implementation,** by a sub-agent on Sonnet in three commits: the
  home page's tab title (`f29338d`); a page frame shared by every page
  (`f58071f`), with the home page's HTML unchanged byte for byte; the
  site's address setting and `404.html` (`1da48fc`). The build stops on
  an invalid address. Each page writes its own `<main>`, because the
  home page fills it with HTML converted from Markdown, which can only
  be set on the element itself.
- **Tests:** 35 unit tests and 59 browser tests passed locally, with the
  accessibility checks run on the new page too.
- **Seen by the user** on a local server that answers missing addresses
  like GitHub Pages: the page shows at a deep address with its look.
- **Published** with version 0.6.0 on 2026-11-02. The live check
  passed: `https://www.lucalorenzon.com/a/b/c/missing-page` answers 404
  with "Page not found" and its stylesheet, and the home page's tab
  reads "Curriculum Vitae - Luca Lorenzon".
